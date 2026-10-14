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

// Drag the body down past the threshold (0.4 x 320 px), from the top of the drawer so the
// pointer stays inside the window: Firefox's automation drops the release outside the viewport.
await trigger.click();
await dialog.waitFor();
await page.waitForTimeout(500);
await page.evaluate(() => {
  window.__events = [];
  let moves = 0;
  document.addEventListener("pointermove", () => moves++ === 0 && window.__events.push("pointermove…"), true);
  for (const type of ["pointerdown", "pointerup", "pointercancel", "lostpointercapture", "gotpointercapture"])
    document.addEventListener(type, (e) => window.__events.push(`${type}@${e.target.tagName ?? "?"}`), true);
});
const box = await dialog.boundingBox();
const start = { x: box.x + box.width / 2, y: box.y + 12 };
const viewport = page.viewportSize();
assert.ok(start.y + 160 < viewport.height, "the drag fits inside the window");
await page.mouse.move(start.x, start.y);
await page.mouse.down();
for (let i = 1; i <= 10; i++) {
  await page.mouse.move(start.x, start.y + i * 16);
  await page.waitForTimeout(30);
}
await page.waitForTimeout(150);
await page.mouse.up();
await page.waitForTimeout(900);
if (await page.getByRole("dialog", { name: "Move goal" }).count())
  console.log("pointer events during the drag:", (await page.evaluate(() => window.__events)).join(" "));
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
