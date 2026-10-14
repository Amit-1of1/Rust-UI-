// Floating content stays attached to its trigger and flips when there is no room
// (CSS anchor positioning). Run against a built gallery: GALLERY_URL=... node anchor.mjs
import { launch } from "./engine.mjs";

const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const browser = await launch();
const page = await browser.newPage({ viewport: { width: 1000, height: 700 } });
await page.goto(url);
await page.waitForSelector(".gallery-main");

// Without anchor positioning the kit keeps its absolute layout (content under the trigger,
// no flipping); see the @supports blocks in the overlay styles. Only the flip checks need it.
const anchored = await page.evaluate(
  () => CSS.supports("anchor-scope: all") && CSS.supports("position-try-fallbacks: flip-block"),
);
if (!anchored) console.log("note: no CSS anchor positioning in this engine; checking the fallback layout only");

let failed = 0;
const check = (ok, msg) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${msg}`);
  if (!ok) failed++;
};

// Puts the trigger at the bottom (or top) of the viewport, opens it, and returns
// the trigger and content rectangles.
async function open(trigger, content, where, how = "click") {
  await page.keyboard.press("Escape");
  await page.evaluate(() => document.activeElement?.blur());
  // scrollIntoView moves whichever container scrolls (the gallery scrolls its main pane).
  await trigger.evaluate((el, where) => {
    el.scrollIntoView({ block: where === "bottom" ? "end" : "start" });
    window.scrollBy(0, where === "bottom" ? 20 : -20);
  }, where);
  // A DOM click: Playwright's mouse click scrolls the page to "stabilise" the target.
  await trigger.evaluate((el, how) => (how === "focus" ? el.focus() : el.click()), how);
  const c = page.locator(content).first();
  await c.waitFor({ state: "visible" });
  await page.waitForTimeout(700); // let the enter spring settle
  const t = await trigger.boundingBox();
  const b = await c.boundingBox();
  return { t, b };
}

for (const [name, trigger, content] of [
  ["dropdown menu", page.getByRole("button", { name: "Actions" }), "[class*=dx-dropdown-menu-content][data-state=open]"],
  ["select", page.locator("[class*=dx-select-trigger]").first(), "[class*=dx-select-list][data-state=open]"],
]) {
  const below = await open(trigger, content, "top");
  check(below.b.y >= below.t.y + below.t.height - 1, `${name} opens below its trigger when there is room`);
  check(Math.abs(below.b.x - below.t.x) < 2, `${name} lines up with its trigger`);
  if (!anchored) continue;
  const above = await open(trigger, content, "bottom");
  check(above.b.y + above.b.height <= above.t.y + 1, `${name} flips above its trigger near the bottom edge`);
  check(above.b.y + above.b.height <= 700, `${name} stays on screen`);
}

// Side-aware content (popover, tooltip): wherever the trigger sits, the content stays on
// screen and next to its trigger.
const gap = (t, b) =>
  Math.max(b.y - (t.y + t.height), t.y - (b.y + b.height), b.x - (t.x + t.width), t.x - (b.x + b.width));
for (const [name, trigger, content, how] of [
  ["popover", page.getByRole("button", { name: "Open popover" }), "[class*=dx-popover-content][data-state=open]", "click"],
  ["tooltip", page.getByText("Hover or focus me"), "[class*=dx-tooltip-content][data-state=open]", "focus"],
]) {
  for (const where of anchored ? ["top", "bottom"] : ["top"]) {
    const { t, b } = await open(trigger, content, where, how);
    // Without anchor positioning nothing flips, so content on the far side of a trigger at the
    // screen edge is cut off (known, docs/GUARDRAILS.md); only check it stays attached.
    if (anchored) check(b.y >= 0 && b.y + b.height <= 700, `${name} stays on screen with its trigger at the ${where}`);
    check(gap(t, b) <= 16, `${name} stays next to its trigger at the ${where} (gap ${Math.round(gap(t, b))}px)`);
  }
}

await browser.close();
if (failed) {
  console.error(`${failed} anchor check(s) failed`);
  process.exit(1);
}
