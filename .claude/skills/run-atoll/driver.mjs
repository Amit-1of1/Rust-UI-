// Drive one of the kit's web apps in headless Chromium: serve its dx build, open it, run steps.
//
//   node .claude/skills/run-atoll/driver.mjs [--app gallery] [--build] [--release] [--dark]
//        [--reduced-motion] [--port N] [step ...]
//
// --app      gallery (default), site, docs, dashboard, booking, showcase, trial
// --build    run `dx build --platform web` first (also done when there is no build yet)
// --release  use target/dx/<app>/release (build it with scripts/release-web.sh <app>)
// --dark     start with prefers-color-scheme: dark; --reduced-motion the same for motion
//
// With no steps on the command line, steps are read from stdin one per line (a heredoc, or
// type them live under tmux). Lines starting with # are comments. Steps:
//
//   goto:<#hash or /path>     open a page of the app fresh (#dialog = gallery section,
//                             #block/login_01 = block page, /trips = booking route)
//   reload                    reload, then wait for the ready selector
//   theme:dark|light|os       set data-theme on <html> (os removes it)
//   accent:<blue|green|...>   set data-accent on <html> (none removes it)
//   viewport:<W>x<H>          resize the page (e.g. 390x844 for a phone)
//   click:<selector>          Playwright selector: text=Open, role=button[name="Save"], #id, css
//   fill:<selector>=<value>   type into an input
//   press:<key>               keyboard key on the focused element (Tab, Escape, Control+k)
//   hover:<selector>
//   scroll:<selector>         scroll an element into view
//   wait:<ms | selector>      sleep, or wait for a selector to appear
//   text:<selector>           print the element's innerText
//   aria:<selector>           print the accessibility tree under the element (body for all)
//   eval:<js expression>      print the result of a JS expression in the page
//   count:<selector>          print how many elements match
//   settle                    wait for finite animations/transitions to end (ss does this itself)
//   ss:<name>[:full]          screenshot to target/run-atoll/<name>.png (:full = whole page)
//   ss:<name>@<selector>      screenshot of one element
//   quit                      stop (end of input also stops)
//
// A failing step prints FAIL and the run continues; the exit status is 1 if any step failed.
// Page errors (a wasm panic shows up as one) and console errors are printed as they happen.
import { createServer } from "node:http";
import { readFile, stat, mkdir } from "node:fs/promises";
import { existsSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { createInterface } from "node:readline";
import { createRequire } from "node:module";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
// Playwright is the one tests/e2e pins (npm ci there); resolve it from that folder.
const require = createRequire(path.join(ROOT, "tests/e2e/package.json"));
const { chromium } = require("playwright");

// name -> selector that exists once the wasm has started and rendered the page.
const APPS = {
  // The second selector is a block page (#block/<name>).
  gallery: ".gallery-main > .gallery-section:last-child, [data-block] h1",
  site: "#pricing",
  docs: ".docs-main",
  dashboard: "body[data-ready='1'] .db-main",
  booking: "body[data-ready='1'] .tr-main",
  showcase: "body[data-ready='1'] .sc-main",
  trial: "body",
};

const args = process.argv.slice(2);
let app = "gallery", build = false, port = 0, release = false, scheme = "light", reducedMotion = "no-preference";
const steps = [];
for (let i = 0; i < args.length; i++) {
  const a = args[i];
  if (a === "--app") app = args[++i];
  else if (a === "--build") build = true;
  else if (a === "--release") release = true;
  else if (a === "--port") port = Number(args[++i]);
  else if (a === "--dark") scheme = "dark";
  else if (a === "--reduced-motion") reducedMotion = "reduce";
  else if (a === "-h" || a === "--help") {
    const src = await readFile(fileURLToPath(import.meta.url), "utf8");
    console.log(src.split("\n").filter((l) => l.startsWith("//")).map((l) => l.slice(3)).join("\n"));
    process.exit(0);
  } else steps.push(a);
}
if (!(app in APPS)) {
  console.error(`--app ${app}: expected one of ${Object.keys(APPS).join(", ")}`);
  process.exit(2);
}

const profile = release ? "release" : "debug";
const publicDir = path.join(ROOT, `target/dx/${app}/${profile}/web/public`);
if (build || !existsSync(path.join(publicDir, "index.html"))) {
  console.log(`building ${app} (dx build --platform web${release ? " --release" : ""})...`);
  execFileSync("dx", ["build", "--platform", "web", ...(release ? ["--release"] : [])], {
    cwd: path.join(ROOT, "apps", app),
    stdio: "inherit",
  });
}

// Static server. SPA fallback to index.html so router apps (booking) open on deep paths.
const TYPES = {
  ".html": "text/html", ".js": "text/javascript", ".mjs": "text/javascript", ".css": "text/css",
  ".wasm": "application/wasm", ".svg": "image/svg+xml", ".png": "image/png", ".json": "application/json",
  ".ico": "image/x-icon", ".woff2": "font/woff2", ".txt": "text/plain",
};
const server = createServer(async (req, res) => {
  let file = path.join(publicDir, decodeURIComponent(new URL(req.url, "http://x").pathname));
  try {
    if ((await stat(file)).isDirectory()) file = path.join(file, "index.html");
  } catch {
    file = path.join(publicDir, "index.html");
  }
  try {
    const body = await readFile(file);
    res.writeHead(200, { "content-type": TYPES[path.extname(file)] ?? "application/octet-stream" });
    res.end(body);
  } catch {
    res.writeHead(404).end();
  }
});
await new Promise((r) => server.listen(port, "127.0.0.1", r));
const base = `http://127.0.0.1:${server.address().port}/`;
console.log(`serving ${path.relative(ROOT, publicDir)} at ${base}`);

const browser = await chromium.launch(
  existsSync(chromium.executablePath()) ? {} : { executablePath: "/opt/pw-browsers/chromium" },
);
const page = await browser.newPage({ colorScheme: scheme, reducedMotion, viewport: { width: 1280, height: 900 } });
page.setDefaultTimeout(30000);
page.on("pageerror", (e) => console.log(`page error: ${e.message.split("\n")[0]}`));
// Debug builds try to open dx's hot-reload socket; with no `dx serve` behind it that always
// fails, so that one error is dropped.
page.on("console", (m) => {
  if (m.type() === "error" && !m.text().includes("/_dioxus")) console.log(`console error: ${m.text().slice(0, 300)}`);
});

// Wait for running enter/exit animations and transitions; infinite ones (spinner, marquee,
// shimmer) are skipped or this would never return.
const settle = () =>
  page.evaluate(() =>
    Promise.race([
      Promise.all(
        document
          .getAnimations()
          .filter((a) => a.effect?.getComputedTiming().iterations !== Infinity)
          .map((a) => a.finished.catch(() => {})),
      ),
      new Promise((r) => setTimeout(r, 3000)),
    ]),
  );

const ready = () => page.waitForSelector(APPS[app]);
const shots = path.join(ROOT, "target/run-atoll");
await mkdir(shots, { recursive: true });

const t0 = Date.now();
await page.goto(base);
await ready();
console.log(`ready: ${app} in ${Date.now() - t0} ms`);

let failed = 0;
async function run(line) {
  line = line.trim();
  if (!line || line.startsWith("#")) return;
  const i = line.indexOf(":");
  const [cmd, arg] = i < 0 ? [line, ""] : [line.slice(0, i), line.slice(i + 1)];
  const root = (name, value) =>
    page.evaluate(([n, v]) => (v ? document.documentElement.setAttribute(n, v) : document.documentElement.removeAttribute(n)), [name, value]);
  switch (cmd) {
    case "goto":
      await page.goto(new URL(arg, base).href);
      // A hash change is a same-document navigation, and the gallery reads #block/<name> only
      // on load: reload so a hash always opens like a fresh visit.
      if (arg.includes("#")) await page.reload();
      await ready();
      // A hash route only scrolls on the first load; scroll to the section by hand too.
      if (arg.startsWith("#")) await page.evaluate((id) => document.getElementById(id)?.scrollIntoView(), arg.slice(1));
      break;
    case "reload": await page.reload(); await ready(); break;
    case "theme": await root("data-theme", arg === "os" ? null : arg); break;
    case "accent": await root("data-accent", arg === "none" ? null : arg); break;
    case "viewport": {
      const [w, h] = arg.split("x").map(Number);
      await page.setViewportSize({ width: w, height: h });
      break;
    }
    case "click": await page.locator(arg).first().click(); break;
    case "hover": await page.locator(arg).first().hover(); break;
    case "scroll": await page.locator(arg).first().scrollIntoViewIfNeeded(); break;
    case "fill": {
      const j = arg.lastIndexOf("=");
      await page.locator(arg.slice(0, j)).first().fill(arg.slice(j + 1));
      break;
    }
    case "press": await page.keyboard.press(arg); break;
    case "wait":
      if (/^\d+$/.test(arg)) await page.waitForTimeout(Number(arg));
      else await page.waitForSelector(arg);
      break;
    case "text": console.log(await page.locator(arg).first().innerText()); break;
    case "aria": console.log(await page.locator(arg || "body").first().ariaSnapshot()); break;
    case "settle": await settle(); break;
    case "count": console.log(await page.locator(arg).count()); break;
    case "eval": console.log(JSON.stringify(await page.evaluate(arg))); break;
    case "ss": {
      const [name, sel] = arg.split("@");
      const [file, mode] = name.split(":");
      const out = path.join(shots, `${file || "shot"}.png`);
      await settle();
      if (sel) await page.locator(sel).first().screenshot({ path: out });
      else await page.screenshot({ path: out, fullPage: mode === "full" });
      console.log(`screenshot: ${path.relative(ROOT, out)}`);
      break;
    }
    case "quit": return "quit";
    default: throw new Error(`unknown step "${cmd}" (see --help)`);
  }
}

async function step(line) {
  try {
    const r = await run(line);
    if (line.trim() && !line.trim().startsWith("#")) console.log(`ok   ${line.trim()}`);
    return r;
  } catch (e) {
    failed++;
    console.log(`FAIL ${line.trim()}: ${e.message.split("\n")[0]}`);
  }
}

if (steps.length) {
  for (const s of steps) if ((await step(s)) === "quit") break;
} else {
  const rl = createInterface({ input: process.stdin, terminal: false });
  for await (const line of rl) if ((await step(line)) === "quit") break;
}

await browser.close();
server.close();
process.exit(failed ? 1 : 0);
