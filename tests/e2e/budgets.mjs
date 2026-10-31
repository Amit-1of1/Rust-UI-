// Size and startup budgets for the release web apps (bench/budgets.json). A PR that makes an
// app's first screen, its main wasm file or its whole download more than 10% bigger fails here,
// with the files that grew; one that makes it 10% smaller is told to lower the budget.
//
//   node budgets.mjs                    check every app built by scripts/release-web.sh
//   BUDGETS_UPDATE=1 node budgets.mjs   write the current numbers as the new budgets
//
// Sizes are brotli (quality 11) of the files on disk, so they are the same on every machine:
// - first: the files that finished loading before the first screen was ready (READY below and
//   every stylesheet loaded), from a load in Chromium at slow 4G (1.6 Mbps, 150 ms RTT);
// - mainWasm: the largest wasm file among those;
// - total: every .wasm, .js and .css file the build ships, lazy parts included.
// Startup is the time to ready with the CPU slowed 4x (median of 5 loads, served locally with
// brotli). Shared CI machines are noisy, so startup only fails past 2x its budget.
import { createServer } from "node:http";
import { readFileSync, writeFileSync, existsSync, statSync, readdirSync } from "node:fs";
import { join, extname, relative } from "node:path";
import { brotliCompressSync, constants } from "node:zlib";
import { chromium } from "playwright";

const root = new URL("../..", import.meta.url).pathname;
const BUDGETS = join(root, "bench/budgets.json");
const TOLERANCE = 0.1;
const STARTUP_TOLERANCE = 1.0;
const RUNS = Number(process.env.RUNS ?? 5);

// What "ready" means per app (as in bench/size.mjs and the run-atoll driver).
const READY = {
  gallery: ".gallery-main .gallery-section button",
  site: "#pricing",
  docs: ".docs-main",
  dashboard: "body[data-ready='1'] .db-main h1",
  booking: "body[data-ready='1'] .tr-main",
  showcase: "body[data-ready='1'] .sc-main",
};

const TYPES = {
  ".html": "text/html", ".js": "text/javascript", ".css": "text/css", ".wasm": "application/wasm",
  ".svg": "image/svg+xml", ".png": "image/png", ".ico": "image/x-icon", ".json": "application/json",
};

const brotli = new Map();
const br = (file) => {
  if (!brotli.has(file)) {
    brotli.set(file, brotliCompressSync(readFileSync(file), { params: { [constants.BROTLI_PARAM_QUALITY]: 11 } }));
  }
  return brotli.get(file);
};
const kb = (n) => Math.round(n / 102.4) / 10;
const median = (xs) => [...xs].sort((a, b) => a - b)[Math.floor(xs.length / 2)];

function serve(dir) {
  const server = createServer((req, res) => {
    const path = decodeURIComponent(new URL(req.url, "http://x").pathname);
    let file = join(dir, path);
    if (!existsSync(file) || statSync(file).isDirectory()) file = join(dir, "index.html");
    res.writeHead(200, {
      "content-type": TYPES[extname(file)] ?? "application/octet-stream",
      "content-encoding": "br",
      "cache-control": "no-store",
    });
    res.end(br(file));
  });
  return new Promise((ok) => server.listen(0, () => ok(server)));
}

function shipped(dir) {
  const out = [];
  const walk = (d) => {
    for (const e of readdirSync(d, { withFileTypes: true })) {
      const p = join(d, e.name);
      if (e.isDirectory()) walk(p);
      else if ([".wasm", ".js", ".css"].includes(extname(e.name))) out.push(p);
    }
  };
  walk(dir);
  return out;
}

// Hashed names change with every build; compare files by name without the hash.
const stable = (name) => name.replace(/[-_.](?:dxh)?[0-9a-f]{8,}(?=\.)/g, "");

async function measure(browser, name, dir) {
  const server = await serve(dir);
  const url = `http://localhost:${server.address().port}/`;
  const loads = [];
  // Load 0 runs at slow 4G, as bench/size.mjs does: on a fast local network the lazy parts
  // finish before the check runs and would count as first screen. The rest time startup.
  for (let i = 0; i <= RUNS; i++) {
    const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
    const page = await context.newPage();
    const cdp = await context.newCDPSession(page);
    await cdp.send("Network.setCacheDisabled", { cacheDisabled: true });
    if (i === 0) {
      await cdp.send("Network.enable");
      await cdp.send("Network.emulateNetworkConditions", {
        offline: false, latency: 150, downloadThroughput: 1.6e6 / 8, uploadThroughput: 750e3 / 8,
      });
    } else {
      await cdp.send("Emulation.setCPUThrottlingRate", { rate: 4 });
    }
    const requested = [];
    page.on("requestfinished", (r) => requested.push(new URL(r.url()).pathname));
    await page.goto(url);
    await page.waitForSelector(READY[name], { timeout: 120000 });
    await page.waitForFunction(() => [...document.querySelectorAll("link[rel=stylesheet]")].every((l) => l.sheet));
    const ms = await page.evaluate(() => performance.now());
    loads.push({ ms, requested: [...new Set(requested)] });
    await context.close();
  }
  server.close();
  const firstFiles = loads[0].requested
    .map((p) => join(dir, decodeURIComponent(p)))
    .filter((f) => existsSync(f) && statSync(f).isFile() && !f.endsWith("favicon.ico"));
  const files = Object.fromEntries(shipped(dir).map((f) => [stable(relative(dir, f)), kb(br(f).length)]));
  const wasm = firstFiles.filter((f) => f.endsWith(".wasm")).map((f) => br(f).length);
  return {
    firstKB: kb(firstFiles.reduce((n, f) => n + br(f).length, 0)),
    firstRequests: firstFiles.length,
    mainWasmKB: kb(wasm.length ? Math.max(...wasm) : 0),
    totalKB: kb(shipped(dir).reduce((n, f) => n + br(f).length, 0)),
    startupMs4x: Math.round(median(loads.slice(1).map((l) => l.ms))),
    files,
  };
}

const budgets = existsSync(BUDGETS) ? JSON.parse(readFileSync(BUDGETS, "utf8")) : { apps: {} };
const browser = await chromium.launch(
  existsSync(chromium.executablePath()) ? {} : { executablePath: process.env.CHROMIUM_PATH ?? "/opt/pw-browsers/chromium" },
);
const current = {};
const failures = [];
const notes = [];
for (const name of Object.keys(READY)) {
  const dir = join(root, `target/dx/${name}/release/web/public`);
  if (!existsSync(join(dir, "index.html"))) {
    console.log(`skip ${name}: no release build (scripts/release-web.sh ${name})`);
    continue;
  }
  const m = await measure(browser, name, dir);
  current[name] = m;
  const b = budgets.apps[name];
  const line = `${name}: first screen ${m.firstKB} KB in ${m.firstRequests} requests, main wasm ${m.mainWasmKB} KB, ` +
    `total ${m.totalKB} KB, ready in ${m.startupMs4x} ms at 4x CPU`;
  if (!b) {
    console.log(`${line} (no budget yet)`);
    continue;
  }
  const over = [];
  for (const key of ["firstKB", "mainWasmKB", "totalKB"]) {
    const limit = b[key] * (1 + TOLERANCE);
    const pct = Math.round((100 * (m[key] - b[key])) / b[key]);
    if (m[key] > limit) over.push(`${key} ${m[key]} KB is ${pct}% over its budget of ${b[key]} KB`);
    else if (m[key] < b[key] * (1 - TOLERANCE)) notes.push(`${name} ${key} is ${-pct}% under budget: lower it (BUDGETS_UPDATE=1)`);
  }
  if (m.firstRequests > b.firstRequests + 1) over.push(`first screen needs ${m.firstRequests} requests, budget ${b.firstRequests}`);
  if (m.startupMs4x > b.startupMs4x * (1 + STARTUP_TOLERANCE)) {
    over.push(`ready in ${m.startupMs4x} ms at 4x CPU, more than twice its budget of ${b.startupMs4x} ms`);
  }
  console.log(`${over.length ? "FAIL" : "ok  "} ${line}`);
  if (over.length) {
    // Which files grew: the ones to look at first.
    const grew = Object.entries(m.files)
      .map(([f, size]) => [f, size, b.files?.[f] ?? 0])
      .filter(([, size, was]) => size - was >= 1)
      .sort((x, y) => y[1] - y[2] - (x[1] - x[2]))
      .slice(0, 5)
      .map(([f, size, was]) => `${f} ${was} -> ${size} KB`);
    failures.push(`${name}: ${over.join("; ")}${grew.length ? `\n    grew: ${grew.join(", ")}` : ""}`);
  }
}
await browser.close();

if (process.env.BUDGETS_UPDATE === "1") {
  const apps = { ...budgets.apps, ...current };
  writeFileSync(
    BUDGETS,
    JSON.stringify(
      {
        note: "Written by BUDGETS_UPDATE=1 node tests/e2e/budgets.mjs. Sizes are brotli KB; startup is ms to ready at 4x CPU slowdown.",
        tolerance: { size: TOLERANCE, startup: STARTUP_TOLERANCE },
        apps,
      },
      null,
      2,
    ) + "\n",
  );
  console.log(`wrote ${relative(root, BUDGETS)}`);
} else {
  for (const n of notes) console.log(`note ${n}`);
  for (const f of failures) console.log(`FAIL ${f}`);
  if (failures.length) {
    console.log("Shrink the change, or raise the budget on purpose: BUDGETS_UPDATE=1 node tests/e2e/budgets.mjs, and say why in the PR.");
    process.exit(1);
  }
}
