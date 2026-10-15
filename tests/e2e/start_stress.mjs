// Load an app many times and report every wasm trap with its whole stack. Used to chase the
// WebKit trap at gallery start (docs/notes/webkit-wasm-trap.md).
//   GALLERY_URL=... LOADS=40 BROWSER=webkit node start_stress.mjs
// READY_SELECTOR: what marks the app as started (default: the gallery's last section).
// SLOW_LAZY_MS: hold back each lazy wasm part (assets/module_*, chunk_*) this long, to widen any
// window between the first screen and the lazy part arriving.
import { launch, ENGINE } from "./engine.mjs";

const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const loads = Number(process.env.LOADS ?? 40);
const ready = process.env.READY_SELECTOR ?? ".gallery-main > .gallery-section:last-child";
const slow = Number(process.env.SLOW_LAZY_MS ?? 0);

const browser = await launch();
let bad = 0;
const seen = new Map();
for (let i = 1; i <= loads; i++) {
  const context = await browser.newContext({ viewport: { width: 1100, height: 900 } });
  const page = await context.newPage();
  page.setDefaultTimeout(60000);
  const errors = [];
  page.on("pageerror", (e) => errors.push(`${e.message}\n${e.stack ?? ""}`));
  page.on("console", (m) => {
    // A missing favicon is not a problem.
    if (/Failed to load resource/.test(m.text())) return;
    if (m.type() === "error" || m.type() === "warning") errors.push(`console.${m.type()}: ${m.text()}`);
  });
  if (slow) {
    await page.route(/\/assets\/(module|chunk)_[^/]*\.wasm/, async (route) => {
      await new Promise((r) => setTimeout(r, slow));
      await route.continue();
    });
  }
  const t0 = Date.now();
  let outcome = "ok";
  try {
    await page.goto(url);
    if (i === 1) console.log(`typeof WebAssembly: ${await page.evaluate(() => typeof WebAssembly)}`);
    await page.waitForSelector(ready);
    // Use the page a little: a click and some scrolling run event closures through wasm.
    await page.mouse.wheel(0, 4000);
    await page.locator("button").first().click({ timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(500);
    // Still alive: one more render through wasm.
    await page.evaluate(() => document.documentElement.setAttribute("data-density", "compact"));
    await page.waitForTimeout(300);
  } catch (e) {
    outcome = `stuck: ${e.message.split("\n")[0]}`;
  }
  if (errors.length) outcome = outcome === "ok" ? "trapped (page went on)" : outcome;
  const ms = Date.now() - t0;
  console.log(`${outcome === "ok" ? "ok  " : "BAD "} load ${i}/${loads} ${ms}ms ${outcome === "ok" ? "" : outcome}`);
  if (outcome !== "ok") {
    bad++;
    for (const e of errors) {
      console.log(`  ${e.split("\n").slice(0, 25).join("\n  ")}`);
      const key = e.split("\n")[0];
      seen.set(key, (seen.get(key) ?? 0) + 1);
    }
  }
  await context.close();
}
await browser.close();
console.log(`\n${ENGINE}: ${bad} of ${loads} loads went wrong`);
for (const [k, n] of seen) console.log(`  ${n}x ${k}`);
process.exit(bad ? 1 : 0);
