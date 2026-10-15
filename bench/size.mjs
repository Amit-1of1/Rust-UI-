// Download size and time to the first screen, for split and unsplit release builds.
//
//   cd tests/e2e && node ../../bench/size.mjs gallery=../../target/dx/gallery/release/web/public ...
//
// Each `name=dir` is served from a built-in static server that sends brotli (what a real host
// does; `python3 -m http.server` sends nothing compressed). Per target, over RUNS runs:
//   - first4gMs: time to a styled first screen with the network at slow 4G
//     (1.6 Mbps down, 750 kbps up, 150 ms RTT)
//   - firstKB / firstRequests: brotli bytes and requests finished by then (on fast networks
//     the later files often arrive before the check runs, so this is read from the slow runs)
//   - totalKB / totalRequests: everything, after the page goes idle and every screen in
//     SCREENS[name] has been opened once
// "Ready" is READY[name] (a selector) plus every stylesheet loaded. Prints JSON.
import { createRequire } from "node:module";
import { createServer } from "node:http";
import { readFileSync, existsSync, statSync } from "node:fs";
import { join, extname, resolve } from "node:path";
import { brotliCompressSync, constants } from "node:zlib";

// Playwright comes from tests/e2e, the directory this runs in.
const { chromium } = createRequire(process.cwd() + "/")("playwright");
const RUNS = Number(process.env.RUNS ?? 5);

const READY = {
  gallery: ".gallery-main .gallery-section button",
  dashboard: "body[data-ready='1'] .db-main h1",
  trial: "body[data-ready='1']",
  react: "body[data-ready='1']",
  svelte: "body[data-ready='1']",
};

// Opened one after another after the first load, so their code is counted in the total.
const SCREENS = {
  dashboard: async (page) => {
    for (const name of ["Customers", "Orders", "Settings"]) {
      await page.locator("[data-sidebar='menu-button']", { hasText: name }).click();
      await page.waitForFunction((n) => document.querySelector("h1.db-title")?.textContent === n
        && !document.querySelector(".db-main > [role='status']"), name);
    }
    await page.keyboard.press("Control+k");
    await page.waitForSelector("[role='dialog'] input");
  },
  gallery: async (page) => {
    // Everything renders on load; wait for the sections below the first screen.
    await page.waitForFunction(() => !document.querySelector(".gallery-loading"));
  },
};

const TYPES = {
  ".html": "text/html", ".js": "text/javascript", ".css": "text/css", ".wasm": "application/wasm",
  ".svg": "image/svg+xml", ".png": "image/png", ".ico": "image/x-icon", ".json": "application/json",
};

function serve(root) {
  const cache = new Map();
  const server = createServer((req, res) => {
    let path = decodeURIComponent(new URL(req.url, "http://x").pathname);
    let file = join(root, path);
    if (!existsSync(file) || statSync(file).isDirectory()) file = join(root, "index.html");
    if (!cache.has(file)) {
      cache.set(file, brotliCompressSync(readFileSync(file), {
        params: { [constants.BROTLI_PARAM_QUALITY]: 11 },
      }));
    }
    res.writeHead(200, {
      "content-type": TYPES[extname(file)] ?? "application/octet-stream",
      "content-encoding": "br",
      "cache-control": "no-store",
    });
    res.end(cache.get(file));
  });
  return new Promise((ok) => server.listen(0, () => ok(server)));
}

const median = (xs) => {
  const s = [...xs].sort((a, b) => a - b);
  return s[Math.floor(s.length / 2)];
};
const kb = (n) => Math.round(n / 102.4) / 10;

async function ready(page, name) {
  await page.waitForSelector(READY[name], { timeout: 120000 });
  await page.waitForFunction(() =>
    [...document.querySelectorAll("link[rel=stylesheet]")].every((l) => l.sheet));
}

async function once(browser, name, url, slow) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const cdp = await context.newCDPSession(page);
  await cdp.send("Network.enable");
  await cdp.send("Network.setCacheDisabled", { cacheDisabled: true });
  if (slow) {
    await cdp.send("Network.emulateNetworkConditions", {
      offline: false, latency: 150, downloadThroughput: 1.6e6 / 8, uploadThroughput: 750e3 / 8,
    });
  }
  const done = []; // { url, bytes }
  const urls = new Map();
  cdp.on("Network.requestWillBeSent", (e) => urls.set(e.requestId, e.request.url));
  cdp.on("Network.loadingFinished", (e) => {
    const u = urls.get(e.requestId) ?? "";
    if (!u.endsWith("favicon.ico")) done.push({ url: u, bytes: e.encodedDataLength });
  });
  await page.goto(url);
  await ready(page, name);
  const ms = await page.evaluate(() => performance.now());
  const first = done.slice();
  let total = null;
  if (!slow) {
    await page.waitForLoadState("networkidle");
    if (SCREENS[name]) {
      await SCREENS[name](page);
      await page.waitForLoadState("networkidle");
    }
    await page.waitForTimeout(500);
    total = done.slice();
  }
  await context.close();
  return { ms, first, total };
}

const targets = process.argv.slice(2).map((a) => a.split("="));
const browser = await chromium.launch();
const out = { date: new Date().toISOString(), runs: RUNS, targets: {} };
for (const [name, dir] of targets) {
  const key = name.split(":")[0];
  if (!READY[key]) throw new Error(`no READY selector for ${key}`);
  const server = await serve(resolve(dir));
  const url = `http://localhost:${server.address().port}/`;
  const fast = await once(browser, key, url, false);
  const slow = [];
  for (let i = 0; i < RUNS; i++) slow.push(await once(browser, key, url, true));
  const sum = (xs) => xs.reduce((n, x) => n + x.bytes, 0);
  out.targets[name] = {
    firstRequests: median(slow.map((r) => r.first.length)),
    firstKB: kb(median(slow.map((r) => sum(r.first)))),
    first4gMs: Math.round(median(slow.map((r) => r.ms))),
    totalRequests: fast.total.length,
    totalKB: kb(sum(fast.total)),
    files: Object.fromEntries(fast.total.map((f) => [f.url.replace(url, ""), kb(f.bytes)])),
  };
  server.close();
  console.error(name, JSON.stringify({ ...out.targets[name], files: undefined }));
}
await browser.close();
console.log(JSON.stringify(out, null, 2));
