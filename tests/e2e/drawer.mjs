// Behaviour of Drawer in the gallery: open, labelled dialog, Escape, drag down to dismiss.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node drawer.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const browser = await launch();
const page = await browser.newPage({ viewport: { width: 900, height: 800 } });
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8765/");
await page.waitForSelector(".gallery-main");
await page.waitForTimeout(1200);

const trigger = page.getByRole("button", { name: "Open drawer" });
await trigger.scrollIntoViewIfNeeded();

await trigger.click();
const dialog = page.getByRole("dialog", { name: "Move goal" });
await dialog.waitFor();
assert.ok(await dialog.getByRole("button", { name: "Close" }).isVisible());

await page.keyboard.press("Escape");
await page.waitForTimeout(400);
assert.equal(await page.getByRole("dialog", { name: "Move goal" }).count(), 0, "Escape closes");

// Drag the body down past the threshold.
await trigger.click();
await dialog.waitFor();
await page.waitForTimeout(500);
const handle = dialog.locator("div").filter({ hasText: /^$/ }).first();
const box = await dialog.getByText("Pick a number").boundingBox();
await page.mouse.move(box.x + 20, box.y + 5);
await page.mouse.down();
for (let i = 1; i <= 10; i++) {
  await page.mouse.move(box.x + 20, box.y + 5 + i * 25);
  await page.waitForTimeout(30);
}
await page.waitForTimeout(150);
await page.mouse.up();
await page.waitForTimeout(900);
assert.equal(await page.getByRole("dialog", { name: "Move goal" }).count(), 0, "dragging down dismisses");

// A short drag springs back and keeps it open.
await trigger.click();
await dialog.waitFor();
await page.waitForTimeout(500);
const box2 = await dialog.getByText("Pick a number").boundingBox();
await page.mouse.move(box2.x + 20, box2.y + 5);
await page.mouse.down();
await page.mouse.move(box2.x + 20, box2.y + 25, { steps: 4 });
await page.waitForTimeout(250);
await page.mouse.up();
await page.waitForTimeout(700);
assert.equal(await page.getByRole("dialog", { name: "Move goal" }).count(), 1, "short drag springs back");
console.log("drawer: ok");
await browser.close();
