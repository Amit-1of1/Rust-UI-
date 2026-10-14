// Behaviour of InputOtp and Carousel in the gallery.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node otp.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const browser = await launch();
const page = await browser.newPage();
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8765/");
await page.waitForSelector(".gallery-main > .gallery-section:last-child");
await page.waitForTimeout(1200);

const group = page.getByRole("group", { name: "Verification code", exact: true });
await group.scrollIntoViewIfNeeded();
const boxes = group.locator("input");
assert.equal(await boxes.count(), 6);
const code = async () => (await boxes.evaluateAll((els) => els.map((e) => e.value))).join("");
const status = page.getByRole("status").filter({ hasText: /code/i });

await boxes.nth(0).click();
await page.keyboard.type("123");
await page.waitForTimeout(150);
assert.equal(await code(), "123", "typing fills boxes in order");
assert.equal(await page.evaluate(() => document.activeElement.getAttribute("aria-label")), "Verification code, character 4 of 6", "focus advances");

await page.keyboard.press("Backspace"); // empty box 4: clears 3 and goes back
await page.waitForTimeout(150);
assert.equal(await code(), "12");

await page.keyboard.type("abc"); // letters are ignored for numeric codes
await page.keyboard.type("3456");
await page.waitForTimeout(200);
assert.equal(await code(), "123456");
assert.match(await status.first().innerText(), /123456 entered/);

// Paste/autofill: put several characters into one box.
await boxes.nth(0).click();
await page.keyboard.press("Control+A");
await page.keyboard.press("Backspace");
await boxes.nth(0).evaluate((el) => {
  el.value = "987654";
  el.dispatchEvent(new Event("input", { bubbles: true }));
});
await page.waitForTimeout(200);
assert.equal(await code(), "987654", "pasted code spreads across boxes");

const carousel = page.getByRole("region", { name: "Featured plans" });
await carousel.scrollIntoViewIfNeeded();
const track = carousel.getByRole("group", { name: "Featured plans slides" });
const before = await track.evaluate((e) => e.scrollLeft);
await carousel.getByRole("button", { name: "Next" }).click();
await page.waitForTimeout(800);
assert.ok((await track.evaluate((e) => e.scrollLeft)) > before, "Next scrolls the track");
console.log("otp+carousel: ok");
await browser.close();
