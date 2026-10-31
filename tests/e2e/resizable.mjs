// Resizable panels in the gallery: the divider is a window splitter (keyboard and mouse).
// Usage: `GALLERY_URL=http://localhost:8765/ node resizable.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const browser = await launch();
const page = await browser.newPage({ viewport: { width: 1000, height: 800 } });
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8765/");
await page.waitForSelector(".gallery-main > .gallery-section:last-child");

const handle = page.getByRole("separator", { name: "Resize folders" });
await handle.scrollIntoViewIfNeeded();
const value = async () => Number(await handle.getAttribute("aria-valuenow"));
const width = () => page.locator("#resizable-folders").evaluate((el) => el.getBoundingClientRect().width);

assert.equal(await value(), 30, "starts at the default size");
assert.equal(await handle.getAttribute("aria-controls"), "resizable-folders");
assert.equal(await handle.getAttribute("aria-orientation"), "vertical");

await handle.focus();
const before = await width();
await page.keyboard.press("ArrowRight");
assert.equal(await value(), 35, "ArrowRight grows the first panel by 5%");
assert.ok((await width()) > before, "and the panel gets wider");
await page.keyboard.press("Shift+ArrowLeft");
assert.equal(await value(), 25, "Shift+ArrowLeft shrinks it by 10%");
await page.keyboard.press("Home");
assert.equal(await value(), 15, "Home goes to the minimum");
await page.keyboard.press("End");
assert.equal(await value(), 60, "End goes to the maximum");
await page.keyboard.press("ArrowRight");
assert.equal(await value(), 60, "it stops at the maximum");
console.log("ok   keyboard: arrows, Shift, Home, End, limits");

// Drag the divider 10% of the group to the left.
await page.keyboard.press("Home");
await page.keyboard.press("ArrowRight"); // 20%
const group = await page.locator("[class*=dx-resizable-group]").first().boundingBox();
const box = await handle.boundingBox();
const x = box.x + box.width / 2;
const y = box.y + box.height / 2;
await page.mouse.move(x, y);
await page.mouse.down();
await page.waitForTimeout(100); // the group's size is measured on pointer down
await page.mouse.move(x + group.width * 0.1, y, { steps: 5 });
await page.mouse.up();
const dragged = await value();
assert.ok(Math.abs(dragged - 30) <= 1, `dragging by 10% of the group moves the divider to ~30% (got ${dragged})`);
console.log("ok   mouse drag");

// The nested vertical group uses Up and Down.
const inner = page.getByRole("separator", { name: "Resize preview" });
assert.equal(await inner.getAttribute("aria-orientation"), "horizontal");
await inner.focus();
const start = Number(await inner.getAttribute("aria-valuenow"));
await page.keyboard.press("ArrowDown");
assert.equal(Number(await inner.getAttribute("aria-valuenow")), start + 5, "ArrowDown grows the top panel");
console.log("ok   vertical group: Up and Down");

await browser.close();
