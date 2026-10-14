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

// Drag the body down past the threshold. The pointer events seen are logged if it fails, since
// engines differ in capture and boundary events.
await trigger.click();
await dialog.waitFor();
await page.waitForTimeout(500);
await page.evaluate(() => {
  window.__events = [];
  let moves = 0;
  document.addEventListener("pointermove", () => moves++ === 0 && window.__events.push("pointermove…"), true);
  for (const type of ["pointerdown", "pointerup", "pointercancel", "pointerleave", "pointerout", "lostpointercapture", "gotpointercapture"])
    document.addEventListener(type, (e) => window.__events.push(`${type}@${e.target.tagName ?? "?"}`), true);
});
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
if (await page.getByRole("dialog", { name: "Move goal" }).count()) {
  console.log("pointer events during the drag:", (await page.evaluate(() => window.__events)).join(" "));
  // The same press-and-drag on a bare page, with and without pointer capture, tells a bug in
  // the kit apart from how this engine's automation delivers events.
  for (const capture of [false, true]) {
    const probe = await browser.newPage();
    await probe.setContent(`<div id=d style="padding:40px;user-select:none"><p id=p>Pick a number</p></div>`);
    await probe.evaluate((capture) => {
      window.__ev = [];
      for (const t of ["pointerdown", "pointermove", "pointerup", "pointercancel", "gotpointercapture", "lostpointercapture"])
        document.addEventListener(t, (e) => (t !== "pointermove" || !window.__ev.includes("pointermove")) && window.__ev.push(t), true);
      if (capture) document.getElementById("d").addEventListener("pointerdown", (e) => e.target.setPointerCapture(e.pointerId));
    }, capture);
    const b = await probe.locator("#p").boundingBox();
    await probe.mouse.move(b.x + 10, b.y + 5);
    await probe.mouse.down();
    for (let i = 1; i <= 5; i++) await probe.mouse.move(b.x + 10, b.y + 5 + i * 25);
    await probe.mouse.up();
    console.log(`bare page, capture ${capture}:`, (await probe.evaluate(() => window.__ev)).join(" "));
    await probe.close();
  }
}
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
