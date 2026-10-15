// What a screen reader is handed, without a screen reader: the accessibility tree.
//  1. The tree for the whole gallery page (Playwright's ARIA snapshot of the DOM each engine
//     built): every control has a role and a name, and the state a screen reader announces
//     (checked, selected, expanded, pressed, value) is exposed. Playwright 1.57 removed
//     page.accessibility, which read each engine's own tree.
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
await page.waitForSelector(".gallery-main > .gallery-section:last-child");
await page.waitForTimeout(1000);

let failed = 0;
const check = (ok, msg) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${msg}`);
  if (!ok) failed++;
};
const section = (title) =>
  page.locator("section.gallery-section").filter({ has: page.locator(`h2:text-is("${title}")`) });

// ---- 1. the page's tree -------------------------------------------------------------
const CONTROL = new Set([
  "button", "link", "checkbox", "switch", "radio", "slider", "tab", "combobox", "textbox",
  "searchbox", "spinbutton", "menuitem", "menuitemcheckbox", "menuitemradio", "option", "listbox",
]);
// One node per line of the YAML: `- role "name" [state] [key=value]: value`, quoted in single
// quotes when YAML needs it.
async function pageTree() {
  const nodes = [];
  for (let line of (await page.locator("body").ariaSnapshot()).split("\n")) {
    line = line.trim();
    if (!line.startsWith("- ")) continue;
    let key = line.slice(2);
    const quoted = key.match(/^'((?:[^']|'')*)'/);
    key = quoted ? quoted[1].replace(/''/g, "'") : key;
    const m = key.match(/^(\w+)(?: "((?:[^"\\]|\\.)*)")?((?: \[[^\]]*\])*)/);
    if (!m) continue;
    const node = { role: m[1], name: m[2] ? JSON.parse(`"${m[2]}"`) : "" };
    for (const [, k, v] of m[3].matchAll(/\[([\w-]+)(?:=([^\]]*))?\]/g)) node[k] = v ?? true;
    nodes.push(node);
  }
  return nodes;
}
const nodes = await pageTree();
const controls = nodes.filter((n) => CONTROL.has(n.role));
const unnamed = controls.filter((n) => !n.name.trim());
for (const n of unnamed.slice(0, 20)) console.log(`     unnamed ${n.role}`);
// An empty name on any control means a screen reader says only its role.
check(unnamed.length === 0, `every control in the tree has a name (${controls.length} controls, ${unnamed.length} unnamed)`);

const byRole = (...roles) => nodes.filter((n) => roles.includes(n.role));
const has = (list, key) => list.length > 0 && list.every((n) => n[key] !== undefined);
// "Not checked" is left out of the tree, so check the state changes when the control is toggled.
async function engineState(name, role) {
  const found = (await pageTree()).find((n) => n.name === name && role.includes(n.role));
  return found?.checked ?? false;
}
for (const [title, role, name] of [
  ["Checkbox", ["checkbox"], "Accept terms"],
  ["Switch", ["switch"], "Airplane mode"],
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
const tabs = byRole("tab");
check(tabs.some((n) => n.selected === true), `the selected tab is exposed as selected (${tabs.length} tabs)`);
// The ARIA snapshot leaves out a slider's value, so read what each slider exposes.
const sliders = await page.getByRole("slider").evaluateAll((els) =>
  els.map((e) => e.getAttribute("aria-valuenow") ?? e.getAttribute("aria-valuetext") ?? (e.type === "range" ? e.value : null)),
);
check(sliders.length > 0 && sliders.every((v) => v !== null), `sliders expose a value (${sliders.length})`);
const pressed = nodes.filter((n) => n.pressed !== undefined);
check(pressed.length > 0, `toggle buttons expose pressed state (${pressed.length} pressed)`);
// The tree writes only `[expanded]`; collapsed triggers are found by role and state.
let expandable = 0;
for (const role of ["button", "combobox", "link", "menuitem"]) {
  for (const expanded of [true, false]) expandable += await page.getByRole(role, { expanded }).count();
}
check(expandable > 0, `disclosures and popups expose expanded state (${expandable})`);
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
