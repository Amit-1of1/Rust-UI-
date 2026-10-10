// Behaviour of MultiSelect and Dropzone in the gallery.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node pickers.mjs`.
import { chromium } from "playwright";
import assert from "node:assert/strict";

const browser = await chromium.launch();
const page = await browser.newPage();
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8765/");
await page.waitForSelector(".gallery-main");
await page.waitForTimeout(1200);

// MultiSelect
const button = page.getByRole("button", { name: /^Teams:/ });
await button.scrollIntoViewIfNeeded();
assert.equal(await button.getAttribute("aria-expanded"), "false");
assert.match(await button.getAttribute("aria-label"), /Teams: Design$/);
await button.click();
await page.waitForTimeout(150);
assert.equal(await button.getAttribute("aria-expanded"), "true");
const panel = page.getByRole("group", { name: "Teams options" });
await panel.getByLabel("Filter Teams").fill("eng");
await page.waitForTimeout(100);
assert.equal(await panel.getByRole("checkbox").count(), 1, "filter narrows the list");
await panel.getByRole("checkbox").check();
await page.waitForTimeout(100);
assert.match(await panel.getByRole("status").innerText(), /2 selected/);
await page.keyboard.press("Escape");
await page.waitForTimeout(150);
assert.equal(await button.getAttribute("aria-expanded"), "false", "Escape closes");
assert.equal(await page.evaluate(() => document.activeElement?.getAttribute("aria-expanded")), "false", "focus returns to the button");
assert.match(await button.getAttribute("aria-label"), /Design, Engineering$/);

// Dropzone: a wrong type is rejected with an alert, a right one is accepted.
const zone = page.locator("label", { hasText: "Drop files here" });
await zone.scrollIntoViewIfNeeded();
const input = zone.locator("input[type=file]");
await input.setInputFiles({ name: "notes.txt", mimeType: "text/plain", buffer: Buffer.from("hi") });
await page.waitForTimeout(200);
const alert = page.getByRole("alert").filter({ hasText: "notes.txt" });
assert.match(await alert.innerText(), /not an accepted type/);
await input.setInputFiles({ name: "report.pdf", mimeType: "application/pdf", buffer: Buffer.from("%PDF") });
await page.waitForTimeout(200);
assert.equal(await alert.count(), 0, "error clears after a valid file");
assert.match(await page.getByRole("status").filter({ hasText: "Files received" }).innerText(), /Files received: 1/);
assert.ok(await page.getByRole("list", { name: "Selected files" }).innerText().then((t) => t.includes("report.pdf")));
console.log("pickers: ok");
await browser.close();
