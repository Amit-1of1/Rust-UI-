// What a screen reader is handed, without a screen reader: the accessibility tree.
//  1. The engine's own tree (page.accessibility.snapshot: Chromium, Firefox and WebKit each
//     build theirs) for the whole gallery page: every control has a role and a name, and the state
//     a screen reader announces (checked, selected, expanded, pressed, value) is exposed.
//  2. Per main component, Playwright's ARIA snapshot (role, name, state as YAML), closed and
//     opened from the keyboard, compared with tests/e2e/snapshots/aria/<name>.yml.
//     UPDATE_SNAPSHOTS=1 rewrites the files; read the diff before committing it.
// What this cannot tell: how NVDA, JAWS, VoiceOver or TalkBack phrase it, browse-mode
// behaviour, or live-region timing. docs/SCREEN_READERS.md lists those manual checks.
// GALLERY_URL=... node a11y_tree.mjs
import { ENGINE, launch } from "./engine.mjs";
import { mkdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";

const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const UPDATE = !!process.env.UPDATE_SNAPSHOTS;
const DIR = new URL("./snapshots/aria/", import.meta.url);
const browser = await launch();
const page = await browser.newPage({ viewport: { width: 1100, height: 900 }, reducedMotion: "reduce" });
await page.goto(url);
await page.waitForSelector(".gallery-main", { timeout: 30000 });
await page.waitForTimeout(1000);

let failed = 0;
const check = (ok, msg) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${msg}`);
  if (!ok) failed++;
};
const section = (title) =>
  page.locator("section.gallery-section").filter({ has: page.locator(`h2:text-is("${title}")`) });

// ---- 1. the engine's tree -------------------------------------------------------------
// Roles the engines report for controls. Firefox and WebKit use a few names of their own.
const CONTROL = new Set([
  "button", "pushbutton", "togglebutton", "link", "checkbox", "checkbutton", "switch", "radio",
  "radiobutton", "slider", "tab", "pagetab", "combobox", "textbox", "entry", "searchbox", "spinbutton",
  "menuitem", "menuitemcheckbox", "menuitemradio", "option", "listbox", "textarea", "textfield",
]);
// The whole page: with a `root`, Playwright drops the subtree when the root itself is not
// "interesting", and the gallery's main pane is a plain container.
const tree = await page.accessibility.snapshot({ interestingOnly: true });
const nodes = [];
(function walk(n) {
  if (!n) return;
  nodes.push(n);
  for (const c of n.children ?? []) walk(c);
})(tree);
const controls = nodes.filter((n) => CONTROL.has(n.role));
const unnamed = controls.filter((n) => !(n.name ?? "").trim());
for (const n of unnamed.slice(0, 20)) console.log(`     unnamed ${n.role}${n.value ? ` (value "${n.value}")` : ""}`);
// Text fields may take their name from a visible label the engine reports as `value`-less;
// an empty name on any control means a screen reader says only its role.
check(unnamed.length === 0, `every control in the engine tree has a name (${controls.length} controls, ${unnamed.length} unnamed)`);

const byRole = (...roles) => nodes.filter((n) => roles.includes(n.role));
const has = (list, key) => list.length > 0 && list.every((n) => n[key] !== undefined);
// Engines differ in how they report "not checked" (Firefox leaves the field out), so check the
// state the engine reports changes when the control is toggled.
async function engineState(name, role) {
  const t = await page.accessibility.snapshot({ interestingOnly: true });
  let found;
  (function walk(n) {
    if (!n || found) return;
    if (n.name === name && role.includes(n.role)) found = n;
    for (const c of n.children ?? []) walk(c);
  })(t);
  return found?.checked ?? false;
}
for (const [title, role, name] of [
  ["Checkbox", ["checkbox", "checkbutton"], "Accept terms"],
  ["Switch", ["switch", "checkbox", "checkbutton", "togglebutton"], "Airplane mode"],
]) {
  const before = await engineState(name, role);
  await section(title).getByRole(role[0]).first().focus();
  await page.keyboard.press("Space");
  await page.waitForTimeout(200);
  const after = await engineState(name, role);
  check(before !== after, `${role[0]} "${name}": engine reports the checked state change (${before} -> ${after})`);
  await page.keyboard.press("Space");
  await page.waitForTimeout(200);
}
const tabs = byRole("tab", "pagetab");
check(tabs.some((n) => n.selected === true), `the selected tab is exposed as selected (${tabs.length} tabs)`);
const sliders = byRole("slider");
check(
  sliders.length > 0 && sliders.every((n) => n.valuetext !== undefined || n.value !== undefined || n.valuemax !== undefined),
  `sliders expose a value (${sliders.length})`,
);
const pressed = nodes.filter((n) => n.pressed !== undefined);
// Playwright's WebKit snapshot does not carry `pressed` at all (aria-pressed itself is
// covered by the ARIA snapshots below and keyboard.mjs).
if (ENGINE === "webkit") console.log(`skip toggle pressed state: not in the WebKit snapshot API`);
else check(pressed.length > 0, `toggle buttons expose pressed state (${pressed.length})`);
const expandable = nodes.filter((n) => n.expanded !== undefined);
check(expandable.length > 0, `disclosures and popups expose expanded state (${expandable.length})`);
const headings = byRole("heading");
check(headings.every((n) => n.level !== undefined), `headings expose a level (${headings.length})`);

// ---- 2. per-component ARIA snapshots ------------------------------------------------------
async function compare(name, locator) {
  const got = (await locator.ariaSnapshot()).trim() + "\n";
  const file = new URL(`${name}.yml`, DIR);
  if (UPDATE || !existsSync(file)) {
    mkdirSync(DIR, { recursive: true });
    writeFileSync(file, got);
    console.log(`wrote snapshots/aria/${name}.yml`);
    return;
  }
  const want = readFileSync(file, "utf8");
  if (got === want) return check(true, `${name}: tree matches`);
  check(false, `${name}: tree differs from snapshots/aria/${name}.yml`);
  const a = want.split("\n"), b = got.split("\n");
  for (let i = 0; i < Math.max(a.length, b.length); i++) {
    if (a[i] !== b[i]) {
      console.log(`     line ${i + 1}\n       want: ${a[i] ?? ""}\n       got:  ${b[i] ?? ""}`);
      break;
    }
  }
}
async function closeAll() {
  await page.keyboard.press("Escape");
  await page.evaluate(() => document.activeElement?.blur());
  await page.waitForTimeout(250);
}
async function scenario(name, body) {
  try {
    await body();
  } catch (e) {
    check(false, `${name}: ${e.message.split("\n")[0]}`);
  }
  await closeAll();
}
const openWith = async (trigger, key = "Enter") => {
  await trigger.focus();
  await page.keyboard.press(key);
  await page.waitForTimeout(400);
};

// Static components: the section as rendered.
for (const [file, title] of [
  ["button", "Button"],
  ["checkbox", "Checkbox"],
  ["switch", "Switch"],
  ["radio-group", "Radio group"],
  ["slider", "Slider"],
  ["tabs", "Tabs"],
  ["accordion", "Accordion"],
  ["toggle-group", "Toggle group"],
  ["field", "Field and validation"],
  ["progress", "Progress"],
  ["pagination", "Pagination"],
  ["breadcrumb", "Breadcrumb"],
  ["menubar", "Menubar"],
  ["input-otp", "Input otp"],
]) {
  await scenario(file, () => compare(file, section(title)));
}

// Overlays, opened from the keyboard; the snapshot is the open popup.
await scenario("dialog", async () => {
  await openWith(section("Dialog").getByRole("button").first());
  await compare("dialog-open", page.getByRole("dialog"));
});
await scenario("alert dialog", async () => {
  await openWith(section("Alert dialog").getByRole("button").first());
  await compare("alert-dialog-open", page.getByRole("alertdialog"));
});
await scenario("dropdown menu", async () => {
  const trigger = section("Dropdown menu").getByRole("button", { name: "Actions" });
  await openWith(trigger);
  check((await trigger.getAttribute("aria-expanded")) === "true", "dropdown menu: trigger reports expanded while open");
  await compare("dropdown-menu-open", page.locator("[class*=dx-dropdown-menu-content][data-state=open]"));
});
await scenario("select", async () => {
  const trigger = section("Select").locator("button[aria-haspopup=listbox]").first();
  await compare("select", section("Select"));
  await openWith(trigger);
  await compare("select-open", page.getByRole("listbox").first());
});
await scenario("combobox", async () => {
  const input = section("Combobox").getByRole("combobox").first();
  await input.focus();
  await page.keyboard.press("ArrowDown");
  await page.waitForTimeout(400);
  check((await input.getAttribute("aria-expanded")) === "true", "combobox: ArrowDown opens it and reports expanded");
  const list = await input.getAttribute("aria-controls");
  check(!!list && (await page.locator(`[id="${list}"]`).count()) === 1, "combobox: aria-controls points at the open list");
  await compare("combobox-open", page.getByRole("listbox").first());
});
await scenario("popover", async () => {
  await openWith(page.getByRole("button", { name: "Open popover" }));
  await compare("popover-open", page.locator("[class*=dx-popover-content][data-state=open]"));
});
await scenario("tooltip", async () => {
  const trigger = page.getByText("Hover or focus me");
  await trigger.focus();
  await page.waitForTimeout(600);
  const tip = page.getByRole("tooltip");
  check(await tip.isVisible(), "tooltip: shows on keyboard focus");
  const described = await trigger.getAttribute("aria-describedby");
  check(!!described && (await tip.getAttribute("id")) === described, "tooltip: trigger is described by the tooltip while open");
});
await scenario("command palette", async () => {
  await page.keyboard.press("Control+k");
  await page.waitForTimeout(400);
  await compare("command-palette-open", page.getByRole("dialog"));
});
await scenario("toast", async () => {
  // A toast must land in a live region that exists before it appears, or it is not announced.
  const before = await page.locator("[aria-live], [role=status], [role=alert], [role=log], [role=region]").count();
  await openWith(section("Toast").getByRole("button").first());
  const live = page.locator("[aria-live=polite], [aria-live=assertive], [role=status], [role=alert], [role=log]");
  check(before > 0 && (await live.count()) > 0, "toast: shown inside a live region");
});

console.log(`${ENGINE}: ${nodes.length} accessibility nodes, ${controls.length} controls`);
await browser.close();
if (failed) {
  console.error(`${failed} accessibility tree check(s) failed`);
  process.exit(1);
}
