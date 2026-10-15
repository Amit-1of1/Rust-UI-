// Springs, gestures and layout animation in the gallery: a thrown card keeps coasting after
// release and stays in bounds, snap points, pinch / Ctrl+wheel / keyboard zoom, the shared
// highlight morphing between options, and reduced motion turning all of it into jumps.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node motion.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const URL = process.env.GALLERY_URL ?? "http://localhost:8765/";
const browser = await launch();

async function open(options = {}) {
  const page = await browser.newPage({ viewport: { width: 900, height: 800 }, ...options });
  await page.goto(URL);
  await page.waitForSelector(".gallery-main > .gallery-section:last-child");
  await page.waitForTimeout(800);
  return page;
}

/** `[x, y]` of an element's `translate(...)` style, or `[0, 0]`. */
const offset = (page, sel) =>
  page.$eval(sel, (el) => {
    const m = /translate\(([-\d.e]+)px, ([-\d.e]+)px\)/.exec(el.style.transform);
    return m ? [Number(m[1]), Number(m[2])] : [0, 0];
  });
const scale = (page) => page.$eval("#pinch", (el) => Number(el.dataset.scale));
const near = (a, b, tol = 0.6) => Math.abs(a - b) <= tol;

const page = await open();

// --- Throw: the card keeps moving after the pointer lets go, then rests inside its area.
const card = page.locator("#drag-card");
await card.scrollIntoViewIfNeeded();
let box = await card.boundingBox();
let p = { x: box.x + 20, y: box.y + 20 };
await page.mouse.move(p.x, p.y);
await page.mouse.down();
for (let i = 1; i <= 6; i++) {
  await page.mouse.move(p.x + i * 12, p.y + i * 4);
  await page.waitForTimeout(16);
}
await page.mouse.up();
const atRelease = await offset(page, "#drag-card");
await page.waitForTimeout(120);
const coasting = await offset(page, "#drag-card");
assert.ok(coasting[0] > atRelease[0] + 2, `keeps coasting after release: ${atRelease} -> ${coasting}`);
await page.waitForTimeout(2500);
const rest = await offset(page, "#drag-card");
assert.ok(rest[0] >= 0 && rest[0] <= 240.01 && rest[1] >= 0 && rest[1] <= 112.01, `rests in bounds: ${rest}`);
const reported = (await page.textContent("#drag-status")).match(/-?\d+/g).map(Number);
assert.ok(near(reported[0], rest[0], 1) && near(reported[1], rest[1], 1), `on_drag_end reports where it rests: ${reported} vs ${rest}`);

// Hard throw to the right edge: overshoots a little (rubber band and bounce), then rests on it.
box = await card.boundingBox();
p = { x: box.x + 40, y: box.y + 40 };
await page.mouse.move(p.x, p.y);
await page.mouse.down();
for (let i = 1; i <= 5; i++) {
  await page.mouse.move(p.x + i * 40, p.y);
  await page.waitForTimeout(16);
}
await page.mouse.up();
let peak = 0;
for (let i = 0; i < 40; i++) {
  peak = Math.max(peak, (await offset(page, "#drag-card"))[0]);
  await page.waitForTimeout(16);
}
await page.waitForTimeout(1200);
assert.ok(near((await offset(page, "#drag-card"))[0], 240), "a hard throw rests on the edge");
assert.ok(peak > 240, `and bounces past it first (peak ${peak})`);

// Keyboard: Home springs back to the start, arrows step.
await card.focus();
await page.keyboard.press("Home");
await page.waitForTimeout(900);
assert.deepEqual((await offset(page, "#drag-card")).map(Math.round), [0, 0], "Home returns to the start");
await page.keyboard.press("ArrowRight");
await page.keyboard.press("ArrowDown");
await page.waitForTimeout(900);
assert.deepEqual((await offset(page, "#drag-card")).map(Math.round), [16, 16], "arrows move 16 px");

// --- Snap points: a short flick lands on the next stop, exactly.
const knob = page.locator("#snap-knob");
await knob.scrollIntoViewIfNeeded();
box = await knob.boundingBox();
p = { x: box.x + 16, y: box.y + 16 };
await page.mouse.move(p.x, p.y);
await page.mouse.down();
for (let i = 1; i <= 4; i++) {
  await page.mouse.move(p.x + i * 10, p.y + i * 30);
  await page.waitForTimeout(16);
}
await page.mouse.up();
await page.waitForTimeout(1500);
const snapped = await offset(page, "#snap-knob");
assert.ok(near(snapped[0], 120, 0.05) && snapped[1] === 0, `flick snaps to the middle stop on x only: ${snapped}`);
assert.equal(await page.textContent("#snap-status"), "Level: Half");
await knob.focus();
await page.keyboard.press("ArrowRight");
await page.waitForTimeout(1200);
assert.ok(near((await offset(page, "#snap-knob"))[0], 240, 0.05), "ArrowRight jumps to the next stop");
assert.equal(await page.textContent("#snap-status"), "Level: Full");

// --- Pinch zoom: keyboard, double click, Ctrl + wheel and two pointers.
const pinch = page.locator("#pinch");
await pinch.scrollIntoViewIfNeeded();
await pinch.focus();
await page.keyboard.press("+");
await page.waitForTimeout(1000);
assert.ok(near(await scale(page), 1.5, 0.01), "+ zooms to 150%");
assert.equal(await page.textContent("#pinch-status"), "Zoom: 150%");
await page.keyboard.press("0");
await page.waitForTimeout(1000);
assert.ok(near(await scale(page), 1, 0.01), "0 resets");

box = await pinch.boundingBox();
const zoomBefore = await page.evaluate(() => window.visualViewport?.scale ?? 1);
await page.mouse.move(box.x + 60, box.y + 50);
await page.keyboard.down("Control");
await page.mouse.wheel(0, -25);
await page.keyboard.up("Control");
await page.waitForTimeout(300);
const wheeled = await scale(page);
assert.ok(wheeled > 1.2, `Ctrl + wheel zooms (${wheeled})`);
assert.equal(await page.evaluate(() => window.visualViewport?.scale ?? 1), zoomBefore, "and the page itself doesn't zoom");

await pinch.dblclick();
await page.waitForTimeout(1000);
assert.ok(near(await scale(page), 1, 0.01), "double click while zoomed resets");

// Two pointers moving apart: synthetic pointer events, the way a touch screen sends them.
await page.$eval("#pinch", async (el) => {
  const r = el.getBoundingClientRect();
  const cy = r.top + r.height / 2;
  const fire = (type, id, x) =>
    el.dispatchEvent(new PointerEvent(type, { pointerId: id, clientX: x, clientY: cy, bubbles: true, isPrimary: id === 1, pointerType: "touch" }));
  const frame = () => new Promise((resolve) => requestAnimationFrame(resolve));
  const c = r.left + r.width / 2;
  fire("pointerdown", 1, c - 40);
  fire("pointerdown", 2, c + 40);
  for (let i = 1; i <= 10; i++) {
    fire("pointermove", 1, c - 40 - i * 4);
    fire("pointermove", 2, c + 40 + i * 4);
    await frame();
  }
  fire("pointerup", 2, c + 80);
  fire("pointerup", 1, c - 80);
});
await page.waitForTimeout(1000);
const pinched = await scale(page);
assert.ok(near(pinched, 2, 0.02), `pinching 80 -> 160 px apart zooms 2x (${pinched})`);

// --- Shared layout: the highlight starts over the old option and springs to the new one.
const highlightX = () => page.$eval("[data-highlight]", (el) => el.getBoundingClientRect().x);
const week = page.getByRole("button", { name: "Week", exact: true });
await week.scrollIntoViewIfNeeded();
const dayX = await highlightX();
const weekButtonX = (await week.boundingBox()).x;
await week.click();
const samples = [];
for (let i = 0; i < 8; i++) {
  samples.push(await highlightX());
  await page.waitForTimeout(16);
}
await page.waitForTimeout(900);
const settled = await highlightX();
assert.ok(near(settled, weekButtonX, 1), `highlight rests under Week (${settled} vs ${weekButtonX})`);
assert.ok(samples.some((x) => x > dayX + 2 && x < settled - 2), `and travels there: ${samples.map(Math.round)}`);
assert.equal(await page.$eval("[data-highlight]", (el) => el.style.transform), "none", "no transform left at rest");

// Shuffled cards slide: right after the click some card is drawn away from its new slot.
await page.getByRole("button", { name: "Shuffle" }).click();
let moving = false;
for (let i = 0; i < 10 && !moving; i++) {
  moving = await page.$$eval("#layout-cards [data-card]", (els) => els.some((el) => el.style.transform.includes("translate")));
  await page.waitForTimeout(16);
}
assert.ok(moving, "cards animate to their new places");
await page.waitForTimeout(1000);
assert.ok(
  await page.$$eval("#layout-cards [data-card]", (els) => els.every((el) => el.style.transform === "none")),
  "and come to rest in them",
);
await page.close();

// --- Reduced motion: the same actions land at once, with nothing in between.
const still = await open({ reducedMotion: "reduce" });
await still.locator("#drag-card").scrollIntoViewIfNeeded();
await still.locator("#drag-card").focus();
await still.keyboard.press("ArrowRight");
await still.waitForTimeout(50);
assert.deepEqual((await offset(still, "#drag-card")).map(Math.round), [16, 0], "reduced motion: arrow move is instant");
await still.locator("#pinch").focus();
await still.keyboard.press("+");
await still.waitForTimeout(50);
assert.ok(near(await scale(still), 1.5, 0.001), "reduced motion: zoom is instant");
const month = still.getByRole("button", { name: "Month", exact: true });
await month.scrollIntoViewIfNeeded();
await month.click();
const transforms = [];
for (let i = 0; i < 6; i++) {
  transforms.push(await still.$eval("[data-highlight]", (el) => el.style.transform));
  await still.waitForTimeout(16);
}
assert.ok(transforms.every((t) => t === "none"), `reduced motion: highlight jumps (${transforms})`);
await still.close();

console.log("motion: ok");
await browser.close();
