// Nested submenus in the gallery's dropdown menu, context menu and menubar: keyboard (APG menu
// pattern), hover intent, placement, closing, and axe on the open submenus.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node submenus.mjs`.
import { launch, TIMEOUT } from "./engine.mjs";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";

const axeSource = readFileSync(createRequire(import.meta.url).resolve("axe-core/axe.min.js"), "utf8");
const browser = await launch();
const page = await browser.newPage({ viewport: { width: 1100, height: 900 } });
page.setDefaultTimeout(TIMEOUT);
const errors = [];
page.on("pageerror", (e) => errors.push(e.message));
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8765/");
await page.waitForSelector(".gallery-main > .gallery-section:last-child");
await page.waitForTimeout(1000);

let failures = 0;
const check = (ok, what) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${what}`);
  if (!ok) failures++;
};
const section = (title) =>
  page.locator("section.gallery-section").filter({ has: page.locator(`h2:text-is("${title}")`) });
const press = (key) => page.keyboard.press(key);
const focusedText = () => page.evaluate(() => document.activeElement?.textContent?.trim() ?? "");
/** Wait (polling) until `fn` is true, up to `ms`; returns its last value. */
async function until(fn, ms = 1500) {
  const end = Date.now() + ms;
  let value = await fn();
  while (!value && Date.now() < end) {
    await page.waitForTimeout(25);
    value = await fn();
  }
  return value;
}
async function axeOn(selector, what) {
  // Let the open animations finish: contrast is measured through their opacity.
  await page.waitForFunction(() => document.getAnimations().every((a) => a.playState !== "running" || a.effect?.getTiming().iterations === Infinity));
  await page.evaluate(axeSource);
  const result = await page.evaluate(
    (sel) => axe.run(document.querySelector(sel), { runOnly: ["wcag2a", "wcag2aa", "wcag21aa", "best-practice"] }),
    selector,
  );
  const found = result.violations.map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(" ")).join(", ")}`);
  check(found.length === 0, `${what}: axe finds nothing${found.length ? `\n      ${found.join("\n      ")}` : ""}`);
}
async function box(locator) {
  return locator.boundingBox();
}

// ---------------------------------------------------------------- dropdown menu (keyboard)
{
  const s = section("Dropdown menu");
  const trigger = s.getByRole("button", { name: "Actions" });
  await trigger.scrollIntoViewIfNeeded();
  await trigger.focus();
  await press("Enter");
  await press("ArrowDown");
  await press("ArrowDown");
  await press("ArrowDown");
  check((await focusedText()).startsWith("Move to"), "dropdown: arrows reach the submenu's item");
  const subTrigger = s.getByRole("option", { name: "Move to" });
  const subList = s.locator("[role=listbox] [role=group]").first();
  await press("ArrowRight");
  check(await until(() => subList.isVisible()), "dropdown: ArrowRight opens the submenu");
  check(await until(async () => (await focusedText()) === "Inbox"), "dropdown: ArrowRight focuses its first item");
  const controls = await subTrigger.getAttribute("aria-controls");
  check(controls && (await subList.getAttribute("id")) === controls, "dropdown: the item names the open list (aria-controls)");
  check((await subList.getAttribute("aria-labelledby")) === (await subTrigger.getAttribute("id")), "dropdown: the list is labelled by its item");
  const t = await box(subTrigger), l = await box(subList);
  check(l.x >= t.x + t.width - 1 && Math.abs(l.y - t.y) < 12, `dropdown: the submenu sits beside its item (${JSON.stringify({ t, l })})`);
  await axeOn("[class*=dx-dropdown-menu-content][data-state=open]", "dropdown with an open submenu");
  await press("ArrowDown");
  check((await focusedText()) === "Later", "dropdown: ArrowDown moves within the submenu");
  // Nested: "Project" opens a third level.
  await press("ArrowDown");
  await press("ArrowRight");
  check(await until(async () => (await focusedText()) === "Atoll"), "dropdown: submenus nest");
  await press("Escape");
  check(await until(async () => (await focusedText()).startsWith("Project")), "dropdown: Escape closes one level, focus on its item");
  check(await subList.isVisible(), "dropdown: the first submenu stays open");
  await press("ArrowLeft");
  check(await until(async () => (await focusedText()).startsWith("Move to")), "dropdown: ArrowLeft closes the submenu, focus on its item");
  check((await trigger.getAttribute("aria-expanded")) === "true", "dropdown: the menu itself stays open");
  await press("ArrowRight");
  await until(async () => (await focusedText()) === "Inbox");
  await press("Enter");
  check(await until(async () => (await trigger.getAttribute("aria-expanded")) === "false"), "dropdown: choosing a submenu item closes the whole menu");
  check(await until(() => trigger.evaluate((el) => el === document.activeElement)), "dropdown: focus returns to the menu button");
  check(await s.getByText("Selected: Inbox").isVisible(), "dropdown: the choice is reported");
  // Tab out of an open submenu closes everything.
  await press("Enter");
  for (let i = 0; i < 3; i++) await press("ArrowDown");
  await press("ArrowRight");
  await until(async () => (await focusedText()) === "Inbox");
  await press("Tab");
  check(await until(async () => (await trigger.getAttribute("aria-expanded")) === "false"), "dropdown: Tab out of a submenu closes the menu");
}

// ---------------------------------------------------------------- dropdown menu (pointer)
{
  const s = section("Dropdown menu");
  const trigger = s.getByRole("button", { name: "Actions" });
  await trigger.click();
  const subTrigger = s.getByRole("option", { name: "Move to" });
  const subList = s.locator("[role=listbox] [role=group]").first();
  const st = await box(subTrigger);
  await page.mouse.move(st.x + 20, st.y + st.height / 2);
  check(!(await subList.isVisible()), "dropdown: hovering does not open at once");
  check(await until(() => subList.isVisible(), 800), "dropdown: resting on the item opens the submenu");
  // Cross the item below on the way to the submenu: it stays open.
  const archive = await box(s.getByRole("option", { name: "Archive" }));
  await page.mouse.move(archive.x + archive.width - 4, archive.y + 4, { steps: 3 });
  await page.waitForTimeout(80);
  // Raw moves: Playwright's hover() waits for the target to settle first, longer than the grace.
  const later = await box(s.getByRole("option", { name: "Later" }));
  await page.mouse.move(later.x + 10, later.y + later.height / 2, { steps: 3 });
  await page.waitForTimeout(500);
  check(await subList.isVisible(), "dropdown: the pointer can cross other items to reach the submenu");
  // Leave both: it closes after the grace period, the menu stays.
  const edit = await box(s.getByRole("option", { name: "Edit" }));
  await page.mouse.move(edit.x + 20, edit.y + edit.height / 2, { steps: 3 });
  check(await until(async () => !(await subList.isVisible()), 1500), "dropdown: leaving closes the submenu");
  check((await trigger.getAttribute("aria-expanded")) === "true", "dropdown: ...and not the menu");
  await subTrigger.click();
  await s.getByRole("option", { name: "Project" }).click();
  await s.getByRole("option", { name: "Ledgerly" }).click();
  check(await until(async () => (await trigger.getAttribute("aria-expanded")) === "false"), "dropdown: clicking a nested item closes the menu");
  check(await s.getByText("Selected: Ledgerly").isVisible(), "dropdown: clicked choice is reported");
}

// ---------------------------------------------------------------- context menu
{
  const s = section("Context menu");
  const area = s.locator("[aria-haspopup=menu]").first();
  await area.scrollIntoViewIfNeeded();
  await area.click({ button: "right" });
  const menu = page.locator("[role=menu][data-state=open]").first();
  await menu.waitFor();
  const share = page.getByRole("menuitem", { name: "Share" });
  check((await share.getAttribute("aria-haspopup")) === "menu", "context menu: the item says it opens a menu");
  check((await share.getAttribute("aria-expanded")) === "false", "context menu: ...and that it is closed");
  await share.hover();
  const sub = s.locator("[role=menu] [role=menu]").first();
  check(await until(() => sub.isVisible(), 800), "context menu: hovering opens the submenu");
  check((await share.getAttribute("aria-expanded")) === "true", "context menu: the item reports it open");
  const t = await box(share), l = await box(sub);
  check(l.x >= t.x + t.width - 1, "context menu: the submenu opens beside the menu");
  await axeOn("[role=menu][data-state=open]", "context menu with an open submenu");
  await page.getByRole("menuitem", { name: "Copy link" }).click();
  check(await until(async () => (await s.locator("[role=menu][data-state=open]").count()) === 0), "context menu: choosing closes it");
  check(await s.getByText("Selected: copy-link").isVisible(), "context menu: the choice is reported");
  // Keyboard inside: ArrowRight in, ArrowLeft out.
  await area.click({ button: "right" });
  await menu.waitFor();
  for (let i = 0; i < 3; i++) await press("ArrowDown");
  check((await focusedText()).startsWith("Share"), "context menu: arrows reach the submenu's item");
  await press("ArrowRight");
  check(await until(async () => (await focusedText()) === "Copy link"), "context menu: ArrowRight opens it and focuses the first item");
  await press("ArrowLeft");
  check(await until(async () => (await focusedText()).startsWith("Share")), "context menu: ArrowLeft goes back");
  await press("Escape");
  await until(async () => (await s.locator("[role=menu][data-state=open]").count()) === 0);
}

// ---------------------------------------------------------------- menubar
{
  const s = section("Menubar");
  const file = s.getByRole("menuitem", { name: "File" });
  await file.scrollIntoViewIfNeeded();
  await file.focus();
  await press("ArrowDown");
  await press("ArrowDown");
  const recent = s.getByRole("menuitem", { name: "Open recent" });
  check(await until(() => recent.evaluate((el) => el === document.activeElement)), "menubar: arrows reach the submenu's item");
  await press("ArrowRight");
  check(await until(async () => (await focusedText()) === "notes.md"), "menubar: ArrowRight opens the submenu");
  await axeOn("[role=menubar] [role=menu] [role=menu] [role=menu]", "menubar's open submenu");
  // Escape closes just the submenu, then the menu.
  await press("Escape");
  check(await until(() => recent.evaluate((el) => el === document.activeElement)), "menubar: Escape returns to the submenu's item");
  check(await until(async () => (await s.locator("[role=menu] [role=menu] [role=menu]").count()) === 0), "menubar: ...and the submenu is gone");
  // Pointer: hover opens, a click on a submenu item chooses it and closes the menu.
  await press("Escape");
  await file.click();
  const r = await box(recent);
  await page.mouse.move(r.x + 20, r.y + r.height / 2);
  const subMenu = s.locator("[role=menu] [role=menu] [role=menu]").first();
  check(await until(() => subMenu.isVisible(), 800), "menubar: hovering opens the submenu");
  await s.getByRole("menuitem", { name: "budget.csv" }).click();
  check(await until(async () => (await s.locator("[role=menubar] > [role=menu][data-state=open]").count()) === 0), "menubar: choosing closes the menu");
}

check(errors.length === 0, `no page errors${errors.length ? ": " + errors.join("; ") : ""}`);
await browser.close();
if (failures) {
  console.log(`${failures} submenu check(s) failed`);
  process.exit(1);
}
console.log("submenus: all checks passed");
