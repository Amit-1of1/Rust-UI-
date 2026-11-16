// Opening and closing a component must leave nothing behind. For every component contract
// (crates/ui/src/components/<name>/contract.toml) with an Escape row that closes what it opened,
// this runs that row again and again on one page and compares Chromium's live DOM node and
// event listener counts after a warm-up with the counts after many more cycles. Both are exact
// counts, so the allowed growth is zero. Chromium only (it reads the counts over CDP).
//   GALLERY_URL=<docs site> node leaks.mjs [component]
//
// Measuring leaks has one trap: a Playwright ElementHandle (`page.$`, `waitForSelector`) keeps
// its element alive until it is disposed, which looks exactly like a leak. Only locators are
// used here.
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { ENGINE, launch } from "./engine.mjs";

if (ENGINE !== "chromium") {
  console.log(`leaks: skipped on ${ENGINE} (needs Chromium's DevTools protocol)`);
  process.exit(0);
}
const url = process.env.GALLERY_URL ?? "http://localhost:8769/";
const only = process.argv[2];
const WARMUP = 5;
const CYCLES = Number(process.env.LEAK_CYCLES ?? 20);
const json = process.env.CONTRACTS_JSON
  ? readFileSync(process.env.CONTRACTS_JSON, "utf8")
  : execFileSync("cargo", ["run", "-q", "-p", "atoll-cli", "--", "contract", "--json"], {
      cwd: new URL("../..", import.meta.url).pathname,
      encoding: "utf8",
    });
const contracts = JSON.parse(json).filter((c) => !only || c.name === only);

// Components that still grow, with the measured growth per cycle and why. The run fails when
// one of these grows by a different amount, so a fix (or a worse leak) updates this list.
const KNOWN = {};

const browser = await launch();
const page = await browser.newPage({ viewport: { width: 1100, height: 900 } });
const cdp = await page.context().newCDPSession(page);
await cdp.send("Performance.enable");

async function counts() {
  // Several rounds: one collection can leave nodes that the next one frees.
  for (let i = 0; i < 3; i++) {
    await cdp.send("HeapProfiler.collectGarbage");
    await page.waitForTimeout(100);
  }
  const m = Object.fromEntries((await cdp.send("Performance.getMetrics")).metrics.map((x) => [x.name, x.value]));
  return { nodes: m.Nodes, listeners: m.JSEventListeners };
}

const scope = () => page.locator(".docs-preview");
const locate = (at) => scope().locator(at.selector).nth(at.nth - 1);
let canary = false;
async function run(steps) {
  for (const s of steps) {
    switch (s.op) {
      case "focus":
        await locate(s.at).focus();
        break;
      case "click":
        await locate(s.at).click();
        break;
      case "press":
        for (let i = 0; i < s.times; i++) await page.keyboard.press(s.key);
        break;
      case "type":
        await page.keyboard.type(s.text);
        break;
      case "visible":
        await locate(s.at).waitFor({ state: "visible" });
        break;
      case "hidden":
        await locate(s.at).waitFor({ state: "hidden" });
        break;
      default:
        // focus, text and attribute checks are the contract runner's job.
        break;
    }
  }
  // Let exit animations finish and their elements leave the DOM.
  await page.waitForTimeout(250);
  // The canary: a deliberate leak of one element with one listener per cycle, to show the
  // measurement sees it.
  if (canary)
    await page.evaluate(() => {
      const el = document.createElement("i");
      el.addEventListener("click", () => {});
      (window.__leak ??= []).push(el);
    });
}

async function measure(c, row) {
  await page.goto(`${url}?leaks#${c.name}`);
  await scope().waitFor();
  await page.waitForTimeout(300);
  for (let i = 0; i < WARMUP; i++) await run(row.steps);
  // Two equal windows. A leak grows in both; a listener or node that is merely still
  // waiting for a timer at the moment of counting shows up in one window only.
  const a = await counts();
  for (let i = 0; i < CYCLES; i++) await run(row.steps);
  const b = await counts();
  for (let i = 0; i < CYCLES; i++) await run(row.steps);
  const c2 = await counts();
  const growth = (k) => Math.min(b[k] - a[k], c2[k] - b[k]) / CYCLES;
  return { nodes: Math.max(0, growth("nodes")), listeners: Math.max(0, growth("listeners")) };
}

let failed = 0;
let checked = 0;
for (const c of contracts) {
  // The Escape row that ends with the thing closed: one full open/close cycle.
  const row = c.rows.find((r) => r.key === "Escape");
  if (!row) continue;
  let result;
  try {
    result = await measure(c, row);
  } catch (e) {
    failed++;
    console.log(`FAIL ${c.name}: the Escape row did not run (${e.message.split("\n")[0]})`);
    continue;
  }
  checked++;
  const known = KNOWN[c.name];
  const expect = known ? { nodes: known.nodes, listeners: known.listeners } : { nodes: 0, listeners: 0 };
  const same = result.nodes === expect.nodes && result.listeners === expect.listeners;
  const line = `${c.name}: ${result.nodes} node(s) and ${result.listeners} listener(s) per open/close`;
  if (same) console.log(`ok   ${line}${known ? ` (known: ${known.why})` : ""}`);
  else {
    failed++;
    console.log(`FAIL ${line}, expected ${expect.nodes} and ${expect.listeners}`);
  }
}

// The gate must see a leak when there is one.
const dialog = JSON.parse(json).find((c) => c.name === "dialog");
if (!only && dialog) {
  canary = true;
  const r = await measure(dialog, dialog.rows.find((x) => x.key === "Escape"));
  canary = false;
  if (r.nodes === 1 && r.listeners === 1) console.log("ok   canary: a leak of 1 node and 1 listener per cycle is seen");
  else {
    failed++;
    console.log(`FAIL canary: a deliberate leak of 1 node and 1 listener measured ${r.nodes} and ${r.listeners}`);
  }
}

await browser.close();
console.log(`${checked} component(s) opened and closed ${WARMUP + 2 * CYCLES} times, ${failed} failed`);
if (failed || checked === 0) process.exit(1);
