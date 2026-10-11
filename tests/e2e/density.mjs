// Density presets: data-density on <html> scales control heights through --density.
// GALLERY_URL=... node density.mjs
import { chromium } from "playwright";

const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1100, height: 900 } });
await page.goto(url);
await page.waitForSelector(".gallery-main");

let failed = 0;
const heights = async (density) => {
  await page.evaluate((d) => {
    if (d) document.documentElement.setAttribute("data-density", d);
    else document.documentElement.removeAttribute("data-density");
  }, density);
  // Controls transition their size; measure once they settle.
  await page.waitForTimeout(600);
  return page.evaluate(() => {
    const h = (sel) => document.querySelector(sel)?.getBoundingClientRect().height ?? 0;
    return { button: h("button[data-size=default]"), input: h("#gallery-name"), select: h("button[aria-haspopup=listbox]") };
  });
};

const base = await heights(null);
for (const [density, factor] of [["compact", 0.8], ["comfortable", 1.2]]) {
  const got = await heights(density);
  for (const [name, h] of Object.entries(got)) {
    const ok = factor < 1 ? h < base[name] : h > base[name];
    console.log(`${ok ? "ok  " : "FAIL"} ${density}: ${name} ${base[name].toFixed(1)}px -> ${h.toFixed(1)}px`);
    if (!ok) failed++;
  }
}
// The button's height is exactly the token times the multiplier.
const compact = await heights("compact");
if (Math.abs(compact.button - base.button * 0.8) > 0.5) {
  console.log(`FAIL compact button should be 80% of ${base.button}px, got ${compact.button}px`);
  failed++;
}
await browser.close();
process.exit(failed ? 1 : 0);
