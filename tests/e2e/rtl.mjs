// Right-to-left layout (docs/RTL.md): with dir="rtl" on <html> the start-side sheet opens
// from the right, a switch's thumb moves to the left when checked, and the carousel's
// "Next" scrolls towards the left, and mirror_rtl icons flip. Usage: `GALLERY_URL=http://localhost:8765/ node rtl.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const browser = await launch();
const page = await browser.newPage({ viewport: { width: 1000, height: 800 } });
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8765/");
await page.waitForSelector(".gallery-main > .gallery-section:last-child");
await page.evaluate(() => document.documentElement.setAttribute("dir", "rtl"));
await page.waitForTimeout(800);
const width = page.viewportSize().width;

// Sheet: "Left" is the start edge, which is the right edge in RTL.
for (const [button, edge] of [["Left", "right"], ["Right", "left"]]) {
  await page.getByRole("button", { name: button, exact: true }).first().evaluate((el) => el.click());
  const sheet = page.getByRole("dialog", { name: "Edit profile" });
  await sheet.waitFor();
  await page.waitForTimeout(700); // let the slide-in finish
  const box = await sheet.boundingBox();
  if (edge === "right") assert.ok(Math.abs(box.x + box.width - width) < 2, `${button} sheet sits on the right edge in RTL (x=${box.x}, w=${box.width})`);
  else assert.ok(box.x < 2, `${button} sheet sits on the left edge in RTL (x=${box.x})`);
  console.log(`ok   "${button}" sheet opens from the ${edge} in RTL`);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(500);
}

// Switch: the thumb travels towards the start (left) when it is turned on.
const toggle = page.getByRole("switch").first();
await toggle.scrollIntoViewIfNeeded();
const thumbX = () => toggle.evaluate((el) => el.querySelector("[class*=dx-switch-thumb]").getBoundingClientRect().x);
const before = await thumbX();
const wasOn = (await toggle.getAttribute("aria-checked")) === "true";
await toggle.evaluate((el) => el.click());
await page.waitForTimeout(400);
const after = await thumbX();
if (wasOn) assert.ok(after > before + 4, `turning off moves the thumb right in RTL (${before} -> ${after})`);
else assert.ok(after < before - 4, `turning on moves the thumb left in RTL (${before} -> ${after})`);
const box = await toggle.boundingBox();
assert.ok(after >= box.x - 1 && after <= box.x + box.width, "the thumb stays inside the switch");
console.log("ok   switch thumb moves towards the start in RTL");

// Carousel: "Next" reveals the next slide, which is to the left in RTL.
const carousel = page.locator("[aria-roledescription=carousel]").first();
const next = carousel.getByRole("button", { name: "Next" });
if (await next.count()) {
  const track = carousel.locator("[id^=dx-carousel-]").first();
  await track.scrollIntoViewIfNeeded();
  const start = await track.evaluate((el) => el.scrollLeft);
  await next.evaluate((el) => el.click());
  await page.waitForTimeout(900);
  const end = await track.evaluate((el) => el.scrollLeft);
  assert.ok(end < start, `Next scrolls towards the left in RTL (scrollLeft ${start} -> ${end})`);
  console.log("ok   carousel Next scrolls towards the start in RTL");
}

// Icons that point along the reading direction flip, with dir set and no RTL lang.
const mirrored = await page.evaluate(() => {
  const icon = document.querySelector("svg[data-mirror-rtl]");
  return icon ? getComputedStyle(icon).scale : null;
});
if (mirrored !== null) {
  assert.equal(mirrored, "-1 1", "mirror_rtl icons flip on a dir=rtl page");
  console.log("ok   mirror_rtl icons flip");
}

await browser.close();
