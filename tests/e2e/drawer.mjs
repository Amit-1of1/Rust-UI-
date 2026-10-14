// Behaviour of Drawer in the gallery: open, labelled dialog, Escape, drag down to dismiss.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node drawer.mjs`.
import { ENGINE, launch } from "./engine.mjs";
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
  // Tell a bug in the kit apart from how this engine's automation delivers input: the same
  // press-and-drag on a bare page in a fresh browser, with and without pointer capture.
  const lost = await captureLosesEvents();
  if (!lost) throw new Error("dragging down dismisses: this engine delivers captured drags on a bare page, so the drawer is at fault");
  // The engine's synthesized mouse stops delivering moves once a pointer is captured (the bare
  // page shows the same), so drive the drawer's own pointer handlers with DOM PointerEvents.
  console.log(`${ENGINE}: captured mouse drags are not delivered by automation; dragging with dispatched PointerEvents`);
  await page.evaluate(async () => {
    const el = [...document.querySelectorAll("[role=dialog] p")].find((p) => p.textContent.includes("Pick a number"));
    const r = el.getBoundingClientRect();
    const at = (type, y) =>
      el.dispatchEvent(new PointerEvent(type, { bubbles: true, cancelable: true, composed: true, pointerId: 1, pointerType: "mouse", isPrimary: true, buttons: type === "pointerup" ? 0 : 1, clientX: r.x + 20, clientY: y }));
    at("pointerdown", r.y + 5);
    for (let i = 1; i <= 10; i++) {
      at("pointermove", r.y + 5 + i * 25);
      await new Promise((f) => setTimeout(f, 30));
    }
    at("pointerup", r.y + 255);
  });
  await page.waitForTimeout(900);
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

// True when a fresh browser of this engine gets moves and the release on a bare page without
// pointer capture but not with it. Mouse actions take no timeout, so each try gets 20 s.
async function captureLosesEvents() {
  const within = (ms, work) => Promise.race([work, new Promise((done) => setTimeout(() => done(null), ms))]);
  const probe = await launch();
  const seen = {};
  for (const capture of [false, true]) {
    const p = await probe.newPage();
    await p.setContent(`<div id=d style="padding:40px;user-select:none"><p id=p>Pick a number</p></div>`);
    await p.evaluate((capture) => {
      window.__ev = [];
      for (const t of ["pointerdown", "pointermove", "pointerup", "gotpointercapture", "lostpointercapture"])
        document.addEventListener(t, () => (t !== "pointermove" || !window.__ev.includes(t)) && window.__ev.push(t), true);
      if (capture) document.getElementById("d").addEventListener("pointerdown", (e) => e.target.setPointerCapture(e.pointerId));
    }, capture);
    const b = await p.locator("#p").boundingBox();
    const done = await within(20000, (async () => {
      await p.mouse.move(b.x + 10, b.y + 5);
      await p.mouse.down();
      for (let i = 1; i <= 5; i++) await p.mouse.move(b.x + 10, b.y + 5 + i * 25);
      await p.mouse.up();
      return true;
    })());
    seen[capture] = (await within(5000, p.evaluate(() => window.__ev.join(" ")))) ?? "(page not answering)";
    console.log(`bare page, capture ${capture}${done ? "" : " (mouse stuck)"}: ${seen[capture]}`);
  }
  await within(5000, probe.close());
  const full = (s) => s.includes("pointermove") && s.includes("pointerup");
  return full(seen[false]) && !full(seen[true]);
}