// Windows High Contrast / forced colors: the browser drops box-shadow and background
// images and repaints colours, so a control whose only edge is a shadow or a fill
// disappears. Emulate forced colors and check every interactive control still has a
// visible edge (a border or outline), and that focus shows as an outline.
// GALLERY_URL=... node forced_colors.mjs
import { chromium } from "playwright";

const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const browser = await chromium.launch();
const page = await browser.newPage({ forcedColors: "active", viewport: { width: 1100, height: 900 } });
await page.goto(url);
await page.waitForSelector(process.env.READY_SELECTOR ?? ".gallery-main", { timeout: 30000 });
await page.waitForTimeout(1000);

const forced = await page.evaluate(() => matchMedia("(forced-colors: active)").matches);
if (!forced) {
  console.error("forced colors did not take effect");
  process.exit(1);
}

const bad = await page.evaluate(() => {
  const SEL = [
    "input:not([type=hidden])", "textarea", "select",
    "[role=checkbox]", "[role=switch]", "[role=radio]", "[role=slider]",
    "[role=combobox]", "[role=tab][aria-selected=true]", "[role=spinbutton]", "button[aria-haspopup]",
  ].join(",");
  const edge = (s) =>
    ["Top", "Right", "Bottom", "Left"].some(
      (side) => parseFloat(s[`border${side}Width`]) > 0 && s[`border${side}Style`] !== "none",
    ) || (parseFloat(s.outlineWidth) > 0 && s.outlineStyle !== "none");
  const out = [];
  for (const el of document.querySelectorAll(SEL)) {
    const r = el.getBoundingClientRect();
    const s = getComputedStyle(el);
    if (r.width === 0 || r.height === 0 || s.visibility === "hidden" || s.opacity === "0") continue;
    // Native checkboxes and radios are drawn by the system in forced colors.
    if (el.matches("input[type=checkbox], input[type=radio], input[type=range]")) continue;
    // A visually hidden input behind a styled control.
    if (r.width <= 1 || r.height <= 1) continue;
    // Its own edge, a drawn marker (a radio's circle), or a direct wrapper drawing the
    // edge for it (an input or date segment inside a group).
    if (edge(s) || edge(getComputedStyle(el, "::before"))) continue;
    if (el.parentElement && edge(getComputedStyle(el.parentElement))) continue;
    const name = el.getAttribute("aria-label") || el.textContent.trim().slice(0, 30) || el.id;
    const section = el.closest("section")?.querySelector("h2")?.textContent ?? "?";
    out.push(`${section}: <${el.tagName.toLowerCase()} role=${el.getAttribute("role") ?? "-"}> ${name}`);
  }
  return [...new Set(out)];
});

// Focus: tab to a few controls and check the ring is an outline, which forced colors keep.
for (const sel of ["#gallery-name", "[role=switch]", "[role=slider]", "button[aria-haspopup=listbox]"]) {
  const el = page.locator(sel).first();
  await el.focus();
  await page.keyboard.press("Shift+Tab");
  await page.keyboard.press("Tab");
  const ring = await el.evaluate((e) => {
    const s = getComputedStyle(e);
    return e === document.activeElement && parseFloat(s.outlineWidth) >= 2 && s.outlineStyle !== "none";
  });
  if (!ring) bad.push(`focus ring missing on ${sel}`);
}

for (const b of bad) console.log(`FAIL no visible edge in forced colors: ${b}`);
console.log(`forced colors: ${bad.length} control(s) without an edge`);
await browser.close();
process.exit(bad.length ? 1 : 0);
