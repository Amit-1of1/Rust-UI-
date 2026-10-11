// Drives the three Taskboard builds (bench/SPEC.md) the same way and writes the numbers.
//
//   TARGETS="dioxus=http://localhost:8801/,react=http://localhost:8802/,svelte=http://localhost:8803/" \
//   RUNS=7 node bench/run.mjs > bench/results/latest.json
//
// Run from tests/e2e (it uses that folder's playwright and axe-core):
//   cd tests/e2e && node ../../bench/run.mjs
//
// Every number is measured in a fresh headless Chromium, on release builds served as static files.
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { execSync } from "node:child_process";
import { gzipSync, brotliCompressSync } from "node:zlib";
import os from "node:os";

const require = createRequire(process.cwd() + "/");
const { chromium } = require("playwright");
const axeSource = readFileSync(require.resolve("axe-core/axe.min.js"), "utf8");

const TARGETS = Object.fromEntries(
  (process.env.TARGETS ?? "dioxus=http://localhost:8801/,react=http://localhost:8802/,svelte=http://localhost:8803/")
    .split(",")
    .map((kv) => kv.split("=")),
);
const RUNS = Number(process.env.RUNS ?? 7);
const median = (xs) => {
  const s = [...xs].sort((a, b) => a - b);
  return s.length % 2 ? s[(s.length - 1) / 2] : (s[s.length / 2 - 1] + s[s.length / 2]) / 2;
};
const round = (x, d = 1) => Math.round(x * 10 ** d) / 10 ** d;
const log = (...a) => console.error(...a);

async function freshPage(browser, url) {
  const context = await browser.newContext({ viewport: { width: 1100, height: 900 } });
  const page = await context.newPage();
  await page.goto(url);
  await page.waitForFunction(() => document.body.dataset.ready === "1", null, { timeout: 30000 });
  return { context, page };
}

// ----- load: bytes, requests, time to ready ---------------------------------------------------

async function load(browser, url) {
  const times = [];
  const slow = [];
  let bytes = null;
  for (let i = 0; i < RUNS; i++) {
    const context = await browser.newContext({ viewport: { width: 1100, height: 900 } });
    const page = await context.newPage();
    const bodies = [];
    page.on("response", async (r) => {
      try {
        bodies.push({ url: r.url(), body: await r.body() });
      } catch {}
    });
    await page.goto(url);
    await page.waitForFunction(() => document.body.dataset.ready === "1", null, { timeout: 30000 });
    // Ready = when the app set the flag, measured from navigation start in the page's own clock.
    const t = await page.evaluate(() => {
      const nav = performance.getEntriesByType("navigation")[0];
      return { ready: performance.now() - nav.startTime };
    });
    times.push(t.ready);
    await page.waitForTimeout(300);
    if (i === 0) {
      const files = bodies.filter((b) => !b.url.endsWith("favicon.ico"));
      const raw = files.reduce((n, f) => n + f.body.length, 0);
      const gz = files.reduce((n, f) => n + gzipSync(f.body, { level: 9 }).length, 0);
      const br = files.reduce((n, f) => n + brotliCompressSync(f.body).length, 0);
      const byKind = {};
      for (const f of files) {
        const kind = (f.url.match(/\.(wasm|js|css|html)(\?|$)/) ?? [, "html"])[1];
        byKind[kind] = (byKind[kind] ?? 0) + gzipSync(f.body, { level: 9 }).length;
      }
      bytes = { requests: files.length, raw, gzip: gz, brotli: br, gzipByKind: byKind };
    }
    await context.close();
  }
  // The same, with the CPU slowed 4x (a mid-range phone): parsing and compiling the bundle
  // is where wasm and JS differ, and a fast desktop hides it.
  for (let i = 0; i < RUNS; i++) {
    const context = await browser.newContext({ viewport: { width: 1100, height: 900 } });
    const page = await context.newPage();
    const cdp = await context.newCDPSession(page);
    await cdp.send("Emulation.setCPUThrottlingRate", { rate: 4 });
    await page.goto(url);
    await page.waitForFunction(() => document.body.dataset.ready === "1", null, { timeout: 60000 });
    slow.push(await page.evaluate(() => performance.now() - performance.getEntriesByType("navigation")[0].startTime));
    await context.close();
  }
  return { readyMs: round(median(times)), readyCpu4xMs: round(median(slow)), ...bytes };
}

// ----- memory: RSS of the whole headless browser after load -----------------------------------

function browserRss(pid) {
  // Sum RSS of the browser and all its children (renderer, GPU, utility): wasm linear memory
  // isn't in the JS heap, so the JS heap alone would flatter the wasm build.
  const out = execSync(`ps -o pid=,ppid=,rss= -A`).toString().trim().split("\n");
  const rows = out.map((l) => l.trim().split(/\s+/).map(Number));
  const kids = new Map();
  for (const [p, pp] of rows) kids.set(pp, [...(kids.get(pp) ?? []), p]);
  const rss = new Map(rows.map(([p, , r]) => [p, r]));
  let total = 0;
  const stack = [pid];
  while (stack.length) {
    const p = stack.pop();
    total += rss.get(p) ?? 0;
    stack.push(...(kids.get(p) ?? []));
  }
  return total * 1024;
}

async function memory(url) {
  const samples = { idle: [], rows10k: [], heapIdle: [], heapRows10k: [] };
  for (let i = 0; i < 3; i++) {
    const before = childPids();
    const browser = await chromium.launch();
    const pid = findChromePid(before);
    const page = await browser.newPage();
    await page.goto(url + "?bench");
    await page.waitForFunction(() => document.body.dataset.ready === "1");
    await page.waitForTimeout(1000);
    const cdp = await page.context().newCDPSession(page);
    const heap = async () => {
      await cdp.send("HeapProfiler.collectGarbage");
      await cdp.send("Performance.enable");
      const mm = (await cdp.send("Performance.getMetrics")).metrics;
      return mm.find((x) => x.name === "JSHeapUsedSize").value;
    };
    samples.heapIdle.push(await heap());
    samples.idle.push(browserRss(pid));
    await page.click("#runlots");
    await page.waitForFunction(() => document.querySelectorAll("#bench tbody tr").length === 10000);
    await page.waitForTimeout(1000);
    samples.heapRows10k.push(await heap());
    samples.rows10k.push(browserRss(pid));
    await browser.close();
  }
  const mb = (x) => round(x / 1024 / 1024);
  return {
    browserRssIdleMB: mb(median(samples.idle)),
    browserRss10kRowsMB: mb(median(samples.rows10k)),
    jsHeapIdleMB: mb(median(samples.heapIdle)),
    jsHeap10kRowsMB: mb(median(samples.heapRows10k)),
  };
}

function childPids() {
  // Skip `ps` itself and the shell running it: they are children of this process too.
  const out = execSync(`ps -o pid=,ppid=,comm= -A`).toString().trim().split("\n");
  return new Set(
    out
      .map((l) => l.trim().split(/\s+/))
      .filter(([, pp, comm]) => Number(pp) === process.pid && !["ps", "sh"].includes(comm))
      .map(([p]) => Number(p)),
  );
}

// Playwright starts the browser as a child of this process: the new child is it.
function findChromePid(before) {
  const fresh = [...childPids()].filter((p) => !before.has(p));
  if (fresh.length !== 1) throw new Error(`could not find the browser process id (${fresh})`);
  return fresh[0];
}

// ----- DOM operations (js-framework-benchmark style) ------------------------------------------

// Runs `setup`, then times `act` from the click until the DOM shows `done` and the next frame
// has painted. Everything runs inside the page, on the page's clock.
const OPS = {
  "create 1,000 rows": { act: "#run", done: "n() === 1000" },
  "replace 1,000 rows": { setup: ["#run"], act: "#run", done: "n() === 1000 && id(0) !== before0" },
  "update every 10th row": { setup: ["#run"], act: "#update", done: "title(0).endsWith(' !!!')" },
  "select a row": { setup: ["#run"], act: "row(4) td:nth-child(2)", done: "rows()[4].classList.contains('selected')" },
  "swap two rows": { setup: ["#run"], act: "#swaprows", done: "id(1) === before998" },
  "remove a row": { setup: ["#run"], act: "row(4) button", done: "n() === 999" },
  "create 10,000 rows": { act: "#runlots", done: "n() === 10000" },
  "append 1,000 to 1,000": { setup: ["#run"], act: "#add", done: "n() === 2000" },
  "clear 1,000 rows": { setup: ["#run"], act: "#clear", done: "n() === 0" },
};

async function op(page, spec) {
  return page.evaluate(async ({ setup, act, done }) => {
    const rows = () => document.querySelectorAll("#bench tbody tr");
    const n = () => rows().length;
    const id = (i) => rows()[i]?.children[0].textContent;
    const title = (i) => rows()[i]?.children[1].textContent;
    const row = (sel) => sel.replace(/^row\((\d+)\)/, (_, i) => `#bench tbody tr:nth-child(${Number(i) + 1})`);
    const frame = () => new Promise((r) => requestAnimationFrame(() => setTimeout(r, 0)));
    const until = async (f) => {
      const t = performance.now();
      while (!f()) {
        if (performance.now() - t > 20000) throw new Error("timed out: " + done);
        await new Promise((r) => setTimeout(r, 0));
      }
    };
    for (const s of setup ?? []) {
      const before = n();
      document.querySelector(s).click();
      await until(() => n() !== before || n() === 1000);
      await frame();
      await frame();
    }
    const before0 = id(0);
    const before998 = id(998);
    const check = new Function("n", "id", "title", "rows", "before0", "before998", `return ${done}`);
    const target = document.querySelector(row(act));
    const t0 = performance.now();
    target.click();
    await until(() => check(n, id, title, rows, before0, before998));
    await frame();
    return performance.now() - t0;
  }, spec);
}

async function ops(browser, url) {
  const out = {};
  for (const [name, spec] of Object.entries(OPS)) {
    const xs = [];
    for (let i = 0; i < RUNS + 1; i++) {
      const { context, page } = await freshPage(browser, url + "?bench");
      const ms = await op(page, spec);
      if (i > 0) xs.push(ms); // first run is warm-up
      await context.close();
    }
    out[name] = round(median(xs));
    log(`  ${name}: ${out[name]} ms`);
  }
  return out;
}

// ----- the real app: interactions, accessibility, keyboard ------------------------------------

async function app(browser, url) {
  const result = {};
  // Filter and sort on 1,000 tasks: time from input to the table showing the result.
  const filter = [];
  const sort = [];
  for (let i = 0; i < RUNS; i++) {
    const { context, page } = await freshPage(browser, url + "?rows=1000");
    filter.push(
      await page.evaluate(async () => {
        const input = document.querySelector('input[aria-label="Filter tasks"], input[placeholder="Filter tasks"]')
          ?? [...document.querySelectorAll("input")].find((el) => el.labels?.[0]?.textContent.includes("Filter tasks"));
        const status = () => document.querySelector('[role="status"]')?.textContent ?? "";
        const before = status();
        const t0 = performance.now();
        // The native setter: React ignores a plain `.value =` on a controlled input.
        Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value").set.call(input, "quick");
        input.dispatchEvent(new Event("input", { bubbles: true }));
        while (status() === before) {
          if (performance.now() - t0 > 10000) throw new Error("filter never changed the status line");
          await new Promise((r) => setTimeout(r, 0));
        }
        await new Promise((r) => requestAnimationFrame(() => setTimeout(r, 0)));
        return performance.now() - t0;
      }),
    );
    await context.close();
    const s = await freshPage(browser, url + "?rows=1000");
    sort.push(
      await s.page.evaluate(async () => {
        // Title: ascending puts "Bright audit #..." first, so the first row always changes
        // (the first row already has the smallest estimate, so sorting by Estimate wouldn't).
        const btn = [...document.querySelectorAll("th button")].find((b) => b.textContent.includes("Title"));
        const firstRow = () => document.querySelector("tbody tr")?.textContent;
        const before = firstRow();
        const t0 = performance.now();
        btn.click();
        while (firstRow() === before) {
          if (performance.now() - t0 > 10000) throw new Error("sort never changed the table");
          await new Promise((r) => setTimeout(r, 0));
        }
        await new Promise((r) => requestAnimationFrame(() => setTimeout(r, 0)));
        return performance.now() - t0;
      }),
    );
    await s.context.close();
  }
  result.filter1000Ms = round(median(filter));
  result.sort1000Ms = round(median(sort));

  // Accessibility: axe on the Tasks tab, the open dialog, and the Stats and Settings tabs.
  const { context, page } = await freshPage(browser, url);
  await page.evaluate(axeSource);
  const axe = async () =>
    page.evaluate(async () => {
      const r = await axe.run(document, { runOnly: ["wcag2a", "wcag2aa", "wcag21aa", "best-practice"] });
      return r.violations.map((v) => `${v.id} (${v.impact}, ${v.nodes.length})`);
    });
  const violations = {};
  violations.tasks = await axe();
  await page.getByRole("tab", { name: "Stats" }).click();
  await page.waitForTimeout(300);
  violations.stats = await axe();
  await page.getByRole("tab", { name: "Settings" }).click();
  await page.waitForTimeout(300);
  violations.settings = await axe();
  await page.getByRole("tab", { name: "Tasks" }).click();
  await page.waitForTimeout(300);

  // Keyboard: open the dialog from the keyboard, submit empty, close with Escape.
  const checks = {};
  const newTask = page.getByRole("button", { name: "New task" });
  await newTask.focus();
  await page.keyboard.press("Enter");
  await page.waitForTimeout(400);
  const dialog = page.getByRole("dialog");
  checks.dialogOpensWithEnter = await dialog.isVisible().catch(() => false);
  checks.focusMovesIntoDialog = await dialog.evaluate((d) => d.contains(document.activeElement)).catch(() => false);
  violations.dialog = await axe();
  const save = dialog.getByRole("button", { name: "Save" });
  await save.focus();
  await page.keyboard.press("Enter");
  await page.waitForTimeout(300);
  checks.emptyTitleShowsError = (await dialog.textContent().catch(() => "")).includes("Title is required");
  checks.titleMarkedInvalid = (await dialog.getByRole("textbox", { name: "Title" }).getAttribute("aria-invalid").catch(() => null)) === "true";
  for (let i = 0; i < 8; i++) await page.keyboard.press("Tab");
  checks.tabStaysInDialog = await dialog.evaluate((d) => d.contains(document.activeElement)).catch(() => false);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(400);
  checks.escapeCloses = !(await dialog.isVisible().catch(() => false));
  checks.focusReturnsToButton = await newTask.evaluate((b) => b === document.activeElement);

  // Save a task through the keyboard and see the toast.
  await newTask.focus();
  await page.keyboard.press("Enter");
  await page.waitForTimeout(400);
  await page.keyboard.type("Benchmark me");
  await page.keyboard.press("Enter");
  await page.waitForTimeout(600);
  checks.saveAddsRowAndToast =
    (await page.locator("tbody tr").first().textContent()).includes("Benchmark me") &&
    (await page.getByText("Task added").count()) > 0;

  // Command menu.
  await page.keyboard.press("Control+k");
  await page.waitForTimeout(400);
  const combo = page.getByRole("combobox", { name: "Search commands" });
  checks.ctrlKOpensCommandMenu = await combo.isVisible().catch(() => false);
  await page.keyboard.type("stats");
  await page.keyboard.press("Enter");
  await page.waitForTimeout(400);
  checks.commandRunsWithEnter = (await page.getByRole("tab", { name: "Stats" }).getAttribute("aria-selected")) === "true";

  // Tabs with arrow keys.
  await page.getByRole("tab", { name: "Stats" }).focus();
  await page.keyboard.press("ArrowRight");
  checks.tabsMoveWithArrows = await page.getByRole("tab", { name: "Settings" }).evaluate((t) => t === document.activeElement);

  await context.close();
  result.axeViolations = violations;
  result.axeViolationCount = Object.values(violations).reduce((n, v) => n + v.length, 0);
  result.keyboard = checks;
  result.keyboardPassed = Object.values(checks).filter(Boolean).length;
  result.keyboardTotal = Object.keys(checks).length;
  return result;
}

// ----- main ------------------------------------------------------------------------------------

// ONLY=app,memory runs just those parts (handy when changing the script).
const ONLY = process.env.ONLY?.split(",");
const want = (part) => !ONLY || ONLY.includes(part);

const browser = await chromium.launch();
const results = {
  date: new Date().toISOString(),
  machine: {
    cpu: os.cpus()[0]?.model,
    cores: os.cpus().length,
    memGB: round(os.totalmem() / 2 ** 30),
    chromium: browser.version(),
    node: process.version,
    note: "headless Chromium in a shared cloud VM; compare ratios between columns, not absolute numbers",
  },
  runs: RUNS,
  targets: {},
};
for (const [name, url] of Object.entries(TARGETS)) {
  log(`== ${name} (${url})`);
  const r = {};
  if (want("load")) log("  load", JSON.stringify((r.load = await load(browser, url))));
  if (want("ops")) r.ops = await ops(browser, url);
  if (want("app")) log("  app", JSON.stringify({ ...(r.app = await app(browser, url)), axeViolations: undefined }));
  if (want("memory")) log("  memory", JSON.stringify((r.memory = await memory(url))));
  results.targets[name] = r;
}
await browser.close();
console.log(JSON.stringify(results, null, 2));
