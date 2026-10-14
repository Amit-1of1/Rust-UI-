// Temporary diagnostic: which part of the drawer stops Firefox delivering a captured drag.
import { launch } from "./engine.mjs";
const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const within = (ms, work) => Promise.race([work, new Promise((d) => setTimeout(() => d("STUCK"), ms))]);
const LOG = `
  window.__ev = {};
  for (const t of ["pointerdown","pointermove","pointerup","pointercancel","gotpointercapture","lostpointercapture","mouseup","mousemove"])
    document.addEventListener(t, () => { window.__ev[t] = (window.__ev[t] ?? 0) + 1; }, true);
  window.__seen = () => Object.entries(window.__ev).map(([k, v]) => k + "×" + v).join(" ");
`;
async function drag(page, selector, { pause = 30 } = {}) {
  const b = await page.locator(selector).first().boundingBox();
  await page.evaluate(LOG);
  await page.evaluate(() => { for (const k in window.__ev) delete window.__ev[k]; });
  const done = await within(15000, (async () => {
    await page.mouse.move(b.x + 10, b.y + 5);
    await page.evaluate(() => { for (const k in window.__ev) delete window.__ev[k]; });
    await page.mouse.down();
    for (let i = 1; i <= 6; i++) {
      await page.mouse.move(b.x + 10, b.y + 5 + i * 25);
      if (pause) await page.waitForTimeout(pause);
    }
    await page.mouse.up();
    return "done";
  })());
  return `${done === "STUCK" ? "(mouse stuck) " : ""}${await within(5000, page.evaluate(() => window.__seen()))}`;
}
const browser = await launch();
const bare = async (name, html, script) => {
  const p = await browser.newPage();
  await p.setContent(html);
  await p.evaluate(script ?? "0");
  console.log(`bare ${name}: ${await drag(p, "#p")}`);
  await p.close();
};
const CAP = `document.getElementById("d").addEventListener("pointerdown", (e) => e.target.setPointerCapture(e.pointerId));`;
const FOLLOW = `${CAP}
  let y0 = null; const d = document.getElementById("d");
  d.addEventListener("pointerdown", (e) => { y0 = e.clientY; });
  d.addEventListener("pointermove", (e) => { if (y0 !== null) d.style.transform = "translateY(" + (e.clientY - y0) + "px)"; });
  d.addEventListener("pointerup", () => { y0 = null; });`;
await bare("capture", `<div id=d style="padding:40px"><p id=p>Pick a number</p></div>`, CAP);
await bare("capture + follow", `<div id=d style="padding:40px"><p id=p>Pick a number</p></div>`, FOLLOW);
await bare("capture + follow + drawer css",
  `<div role=dialog aria-modal=true style="position:fixed;inset:auto 0 0 0"><div id=d tabindex=0 style="padding:40px;touch-action:pan-x;cursor:grab;user-select:none;transform:translateY(0px)"><p id=p>Pick a number</p></div></div>`, FOLLOW);

// On the gallery page itself.
for (const [name, pause] of [["drawer drag, 30 ms pauses", 30], ["drawer drag, no pauses", 0]]) {
  const page = await browser.newPage({ viewport: { width: 900, height: 800 } });
  await page.goto(url);
  await page.waitForSelector(".gallery-main");
  await page.waitForTimeout(1200);
  await page.getByRole("button", { name: "Open drawer" }).click();
  await page.getByRole("dialog", { name: "Move goal" }).waitFor();
  await page.waitForTimeout(600);
  console.log(`gallery ${name}: ${await drag(page, "[role=dialog] p", { pause })}`);
  await page.close();
}
{
  // A plain element with its own capture, inside the open drawer: the page without the kit's handler.
  const page = await browser.newPage({ viewport: { width: 900, height: 800 } });
  await page.goto(url);
  await page.waitForSelector(".gallery-main");
  await page.waitForTimeout(1200);
  await page.getByRole("button", { name: "Open drawer" }).click();
  await page.getByRole("dialog", { name: "Move goal" }).waitFor();
  await page.waitForTimeout(600);
  await page.evaluate(() => {
    const d = document.createElement("div");
    d.id = "probe";
    d.style.cssText = "padding:20px;background:#eee";
    d.innerHTML = "<span id=q>probe</span>";
    d.addEventListener("pointerdown", (e) => { e.stopPropagation(); e.target.setPointerCapture(e.pointerId); });
    document.querySelector("[role=dialog]").prepend(d);
  });
  console.log(`gallery plain element in the drawer: ${await drag(page, "#q")}`);
  await page.evaluate(() => {
    const d = document.createElement("div");
    d.innerHTML = "<span id=r>outside</span>";
    d.style.cssText = "position:fixed;top:10px;left:10px;padding:20px;z-index:99999;background:#eee";
    d.addEventListener("pointerdown", (e) => e.target.setPointerCapture(e.pointerId));
    document.body.append(d);
  });
  console.log(`gallery plain element outside the modal: ${await drag(page, "#r")}`);
  await page.close();
}
await browser.close();
