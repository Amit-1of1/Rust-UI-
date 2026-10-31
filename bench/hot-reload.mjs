// Time from saving a file to the change showing in the browser, under `dx serve`.
//
//   (cd apps/gallery && dx serve --platform web --port 8099)      # wait for the first build
//   cd tests/e2e && node ../../bench/hot-reload.mjs               # URL=... to change the address
//
// Three edits, each undone afterwards: an rsx text change in the gallery, a CSS change in a kit
// component, and a Rust change dx cannot hot-reload (a full rebuild). Prints seconds per edit.
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
// Playwright comes from tests/e2e, the directory this runs in.
const { chromium } = createRequire(process.cwd() + "/")("playwright");
const ROOT = resolve(import.meta.dirname, "..");
const browser = await chromium.launch();
const page = await browser.newPage();
await page.goto(process.env.URL ?? "http://localhost:8099/");
await page.waitForSelector("h1", { timeout: 120000 });
async function trial(label, file, from, to, check, timeout = 300000) {
  const path = ROOT + "/" + file;
  const src = readFileSync(path, "utf8");
  if (!src.includes(from)) throw new Error("missing " + from);
  const t0 = Date.now();
  writeFileSync(path, src.replace(from, to));
  try {
    await page.waitForFunction(check, null, { timeout, polling: 50 });
    console.log(label, ((Date.now() - t0) / 1000).toFixed(2), "s");
  } catch (e) { console.log(label, "timeout"); }
  return () => writeFileSync(path, src);
}
// 1. rsx text edit in the app
let undo = await trial("rsx text, app", "apps/gallery/src/main.rs", 'h1 { "Atoll kit" }', 'h1 { "Atoll kit 2" }',
  () => document.querySelector("h1")?.textContent === "Atoll kit 2");
undo();
await page.waitForFunction(() => document.querySelector("h1")?.textContent === "Atoll kit", null, { timeout: 300000 }).catch(() => {});
// 2. CSS edit in a kit component
undo = await trial("css, kit component", "crates/ui/src/components/kbd/style.css", ".dx-kbd {", ".dx-kbd {\n  outline: 3px solid rgb(1, 2, 3);",
  () => [...document.querySelectorAll("kbd")].some(e => getComputedStyle(e).outlineColor === "rgb(1, 2, 3)"), 120000);
undo();
// 3. Rust logic edit in the app (string built by code, not a literal in rsx)
undo = await trial("rust logic, app (full rebuild)", "apps/gallery/src/main.rs", 'h1 { "Atoll kit" }', 'h1 { {format!("Atoll kit {}", 1 + 2)} }',
  () => document.querySelector("h1")?.textContent === "Atoll kit 3", 600000);
undo();
await browser.close();
