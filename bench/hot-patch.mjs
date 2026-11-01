// Time from saving a Rust file to the change showing in the browser, under
// `dx serve --hot-patch` (Subsecond). Needs the fork dx from scripts/install-dx.sh.
//
//   (cd apps/gallery && dx serve --platform web --port 8099 --hot-patch)   # wait for the first build
//   node bench/hot-patch.mjs                                               # the gallery
//
//   # any other app: the file to edit and a string literal it renders as text
//   FILE=~/my-app/src/main.rs TEXT="My app" node bench/hot-patch.mjs
//
// Each edit is a Rust change that `rsx!` hot reload cannot apply, so plain `dx serve` rebuilds
// and reloads the page for it (bench/hot-reload.mjs, 62 s on the gallery). Under --hot-patch the
// running page keeps its state. Edits:
//   1. the literal becomes `{format!("<text> {}", 1 + 2)}` (an expression),
//   2. the literal becomes a call to a new helper function added at the end of the file,
//   3. (gallery only, KIT=0 to skip) a kit component in another workspace crate, Kbd in
//      crates/ui, gets an attribute computed by `format!`. Patching another crate recompiles that
//      crate; atoll-ui holds every component and demo, so this one is slow.
// Every edit is undone afterwards; the undo is timed too (an undo is often plain hot reload).
// URL=... changes the address, RUNS=n repeats edits 1 and 2.
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const require = createRequire(import.meta.url);
let playwright;
try {
  playwright = require("playwright");
} catch {
  playwright = createRequire(resolve(import.meta.dirname, "../tests/e2e") + "/")("playwright");
}
const ROOT = resolve(import.meta.dirname, "..");
const FILE = resolve(ROOT, process.env.FILE ?? "apps/gallery/src/main.rs");
const TEXT = process.env.TEXT ?? "Atoll kit";
const KIT = process.env.KIT ?? (process.env.FILE ? "0" : "1");
const RUNS = Number(process.env.RUNS ?? 1);
const WAIT = { timeout: 180000, polling: 25 };

const browser = await playwright.chromium.launch();
const page = await browser.newPage();
page.on("pageerror", (e) => console.log("page error:", e.message));
await page.goto(process.env.URL ?? "http://localhost:8099/");
await page.waitForFunction((t) => document.body?.innerText.includes(t), TEXT, { timeout: 300000 });

// `check(true)` waits for the edit to show, `check(false)` for the undo.
const times = {};
async function timed(label, path, edit, check) {
  const src = readFileSync(path, "utf8");
  const next = edit(src);
  if (next === src) throw new Error(`edit "${label}" changed nothing in ${path}`);
  let t0 = Date.now();
  writeFileSync(path, next);
  let s = null;
  try {
    await check(true);
    s = (Date.now() - t0) / 1000;
  } catch {}
  (times[label] ??= []).push(s);
  console.log(label, s === null ? "timeout" : s.toFixed(2) + " s");
  t0 = Date.now();
  writeFileSync(path, src);
  try {
    await check(false);
    console.log("  undo", ((Date.now() - t0) / 1000).toFixed(2), "s");
  } catch {
    console.log("  undo timeout");
  }
}
// Plain `dx serve` reloads the page instead of patching it; keep waiting across the reload.
async function until(fn, arg) {
  const end = Date.now() + WAIT.timeout;
  for (;;) {
    try {
      return await page.waitForFunction(fn, arg, { ...WAIT, timeout: Math.max(1, end - Date.now()) });
    } catch (e) {
      if (Date.now() >= end || !/context was destroyed|navigation/i.test(String(e))) throw e;
    }
  }
}
const hasText = (t, on) => until(([t, on]) => document.body?.innerText.includes(t) === on, [t, on]);

const lit = `"${TEXT}"`;
for (let run = 0; run < RUNS; run++) {
  await timed("expression in the app", FILE,
    (src) => src.replace(lit, `{format!("${TEXT} {}", 1 + 2)}`),
    (on) => hasText(`${TEXT} 3`, on));
  await timed("new helper fn in the app", FILE,
    (src) => src.replace(lit, "{hp_bench_text()}") +
      `\nfn hp_bench_text() -> String {\n    format!("patched {}", 40 + 2)\n}\n`,
    (on) => hasText("patched 42", on));
}
if (KIT === "1") {
  await timed("kit crate (atoll-ui)", ROOT + "/crates/ui/src/components/kbd/component.rs",
    (src) => src.replace("kbd { ..merge_attributes(", 'kbd { "data-hp": format!("{}", 40 + 2), ..merge_attributes('),
    (on) => until((on) => (document.querySelector("kbd")?.dataset.hp === "42") === on, on));
}
const median = (xs) => {
  const v = xs.filter((x) => x !== null).sort((a, b) => a - b);
  return v.length ? v[Math.floor(v.length / 2)] : null;
};
console.log("median s:", JSON.stringify(Object.fromEntries(Object.entries(times).map(([k, v]) => [k, median(v)]))));
await browser.close();
