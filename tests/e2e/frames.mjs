// What actually animates when you use the kit, and what it costs the browser. Runs every row of
// every component contract (the same key presses as contracts.mjs) on the docs site and records
// each CSS animation and transition that starts: which properties it moves, how long it runs,
// and the frame times while anything is moving.
//
// Checks, on every engine:
//   - every animated property is composite (transform, opacity, filter) or paint (colours,
//     shadows), or a layout property whose rule explains it with an `atoll-motion: layout`
//     comment (the static side is `atoll motion`, crates/cli/src/motion.rs);
//   - with prefers-reduced-motion, nothing runs longer than 1 ms.
// On Chromium it also reads the browser's own verdict from a performance trace: every CSS
// animation whose keyframes are composite-only must run on the compositor thread (Chromium
// reports `compositeFailed` otherwise), and it measures frame times with the CPU slowed 4x.
//   GALLERY_URL=<docs site> CONTRACTS_JSON=target/contracts.json node frames.mjs [component]
// FRAMES_JSON=<file> writes the measurements; FRAME_BUDGET=1 also fails on slow frames (local
// runs only: shared CI machines are too noisy for frame times).
import { readFileSync, readdirSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { launch, ENGINE } from "./engine.mjs";

const url = process.env.GALLERY_URL ?? "http://localhost:8769/";
const only = process.argv[2];
const root = new URL("../..", import.meta.url).pathname;
const contracts = JSON.parse(readFileSync(process.env.CONTRACTS_JSON ?? join(root, "target/contracts.json"), "utf8"))
  .filter((c) => !only || c.name === only);

// The same classes as crates/cli/src/motion.rs.
const COMPOSITE = new Set(["transform", "translate", "scale", "rotate", "opacity", "filter", "backdrop-filter"]);
const PAINT = new Set([
  "color", "background", "background-color", "background-position", "border-color", "border-top-color",
  "border-right-color", "border-bottom-color", "border-left-color", "border-inline-color", "border-block-color",
  "outline-color", "outline-offset", "text-decoration-color", "box-shadow", "fill", "stroke", "stroke-dashoffset",
  "stroke-dasharray", "visibility", "clip-path", "caret-color", "accent-color", "text-shadow", "border-radius",
  // Longhands the engines report for the shorthands above.
  "border-top-left-radius", "border-top-right-radius", "border-bottom-left-radius", "border-bottom-right-radius",
  "outline-style", "text-decoration-line", "background-image",
]);
const cost = (p) => (COMPOSITE.has(p) ? "composite" : PAINT.has(p) ? "paint" : "layout");

// Layout animations the stylesheets explain: class or keyframes name -> properties.
const explained = new Map();
function scan(dir) {
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name);
    if (e.isDirectory()) scan(p);
    else if (e.name.endsWith(".css")) {
      const css = readFileSync(p, "utf8");
      // A comment explains the rule (or @keyframes block) it sits in.
      const re = /([^{};]*)\{[^{}]*?\/\*\s*atoll-motion:\s*layout([^(*]*)/g;
      for (const m of css.matchAll(re)) {
        const props = m[2].trim().split(/[\s,]+/).filter(Boolean);
        const prelude = m[1].trim();
        const names = prelude.startsWith("@keyframes")
          ? [prelude.split(/\s+/)[1]]
          : [...prelude.matchAll(/\.(dx-[\w-]+)/g)].map((x) => x[1]);
        for (const n of names) explained.set(n, new Set([...(explained.get(n) ?? []), ...props]));
      }
    }
  }
}
scan(join(root, "crates/ui/src"));

// css_module appends a hash to class and keyframes names; match on the name before it.
function isExplained(names, property) {
  for (const n of names) {
    for (const [base, props] of explained) {
      if ((n === base || n.startsWith(base + "-")) && props.has(property)) return true;
    }
  }
  return false;
}

// Records, inside the page, every animation and transition that starts and the frames drawn
// while anything runs.
const RECORDER = () => {
  window.__motion = { runs: [], frames: [] };
  const describe = (el) => [...(el.classList ?? [])];
  const add = (kind, e) => {
    const target = e.target;
    const anims = target.getAnimations ? target.getAnimations() : [];
    const anim = anims.find((a) =>
      kind === "animation" ? a.animationName === e.animationName : a.transitionProperty === e.propertyName,
    );
    let properties = [e.propertyName];
    if (kind === "animation") {
      const frames = anim?.effect?.getKeyframes?.() ?? [];
      const skip = new Set(["offset", "computedOffset", "easing", "composite"]);
      properties = [...new Set(frames.flatMap((f) => Object.keys(f).filter((k) => !skip.has(k))))].map((k) =>
        k.replace(/[A-Z]/g, (c) => "-" + c.toLowerCase()),
      );
    }
    const timing = anim?.effect?.getComputedTiming?.();
    window.__motion.runs.push({
      kind,
      name: kind === "animation" ? e.animationName : e.propertyName,
      classes: describe(target).concat(e.pseudoElement ? [e.pseudoElement] : []),
      properties,
      duration: timing ? Number(timing.duration) || 0 : 0,
      infinite: timing ? timing.iterations === Infinity : false,
    });
  };
  document.addEventListener("animationstart", (e) => add("animation", e), true);
  document.addEventListener("transitionrun", (e) => add("transition", e), true);
  const tick = (t) => {
    const moving = document.getAnimations().some((a) => a.playState === "running" && a.effect?.getComputedTiming().iterations !== Infinity);
    if (moving) window.__motion.frames.push(t);
    else if (window.__motion.frames.length && window.__motion.frames.at(-1) !== null) window.__motion.frames.push(null);
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
};

async function step(page, s) {
  const loc = () => page.locator(".docs-preview").locator(s.at.selector).nth(s.at.nth - 1);
  switch (s.op) {
    case "focus":
      return loc().focus().catch(() => {});
    case "click":
      return loc().click({ timeout: 3000 }).catch(() => {});
    case "press":
      for (let i = 0; i < s.times; i++) {
        await page.keyboard.press(s.key);
        await page.waitForTimeout(60);
      }
      return;
    case "type":
      return page.keyboard.type(s.text);
    default:
      // A check: give the animation it waits for time to run.
      return page.waitForTimeout(120);
  }
}

/** Frame intervals (ms) between consecutive frames that both had something moving. */
function intervals(frames) {
  const out = [];
  for (let i = 1; i < frames.length; i++) {
    if (frames[i] !== null && frames[i - 1] !== null) out.push(frames[i] - frames[i - 1]);
  }
  return out;
}
const pct = (xs, q) => (xs.length ? [...xs].sort((a, b) => a - b)[Math.min(xs.length - 1, Math.floor(q * xs.length))] : 0);

async function sweep({ reduced }) {
  const browser = await launch();
  const context = await browser.newContext({
    viewport: { width: 1100, height: 900 },
    reducedMotion: reduced ? "reduce" : "no-preference",
  });
  await context.addInitScript(RECORDER);
  const page = await context.newPage();
  const chromium = ENGINE === "chromium" && !reduced;
  const cdp = chromium ? await context.newCDPSession(page) : null;
  if (cdp) await cdp.send("Emulation.setCPUThrottlingRate", { rate: 4 });

  const result = { runs: [], frames: [], trace: [], perComponent: {} };
  let fresh = 0;
  for (const c of contracts) {
    if (cdp) {
      await cdp.send("Tracing.start", {
        categories: "blink.animations,devtools.timeline",
        transferMode: "ReturnAsStream",
      });
    }
    const runs = [];
    let frames = [];
    for (const row of c.rows) {
      await page.goto(`${url}?frames=${fresh++}#${c.name}`);
      await page.waitForSelector(".docs-preview");
      await page.waitForTimeout(150);
      await page.evaluate(() => {
        window.__motion.runs.length = 0;
        window.__motion.frames.length = 0;
      });
      for (const s of row.steps) await step(page, s);
      await page.waitForTimeout(400);
      const m = await page.evaluate(() => window.__motion);
      runs.push(...m.runs.map((r) => ({ ...r, component: c.name, row: row.key })));
      frames = frames.concat(intervals(m.frames));
    }
    if (cdp) {
      const done = new Promise((resolve) => cdp.once("Tracing.tracingComplete", resolve));
      await cdp.send("Tracing.end");
      const { stream } = await done;
      let text = "";
      for (;;) {
        const chunk = await cdp.send("IO.read", { handle: stream });
        text += chunk.base64Encoded ? Buffer.from(chunk.data, "base64").toString() : chunk.data;
        if (chunk.eof) break;
      }
      await cdp.send("IO.close", { handle: stream });
      const events = JSON.parse(text).traceEvents ?? JSON.parse(text);
      // Chromium reuses an animation's trace id once it ends: a "b" event starts a new record.
      const current = new Map();
      const records = [];
      let layouts = 0;
      for (const e of events) {
        if (e.name === "Layout") layouts++;
        if (e.name !== "Animation") continue;
        const id = `${e.id2?.local ?? e.id ?? ""}`;
        const data = e.args?.data ?? {};
        if (e.ph === "b") {
          const a = { name: data.displayName ?? data.name ?? "", node: data.nodeName ?? "", failed: 0, unsupported: [] };
          records.push(a);
          current.set(id, a);
        } else if (data.compositeFailed && current.has(id)) {
          const a = current.get(id);
          a.failed |= data.compositeFailed;
          a.unsupported.push(...(data.unsupportedProperties ?? []));
        }
      }
      const byId = new Map(records.map((a, i) => [i, a]));
      result.trace.push(...[...byId.values()].map((a) => ({ ...a, component: c.name })));
      result.perComponent[c.name] = { layouts };
    }
    result.runs.push(...runs);
    result.frames.push(...frames);
    result.perComponent[c.name] = {
      ...(result.perComponent[c.name] ?? {}),
      animations: runs.length,
      frames: frames.length,
      p95: pct(frames, 0.95),
      max: frames.length ? Math.max(...frames) : 0,
    };
  }
  await browser.close();
  return result;
}

const failures = [];
const normal = await sweep({ reduced: false });

// 1. Every property that moved is composite or paint, or an explained layout property.
const byCost = { composite: 0, paint: 0, layout: 0 };
const seen = new Set();
for (const r of normal.runs) {
  for (const p of r.properties) {
    const k = cost(p);
    byCost[k]++;
    const names = r.kind === "animation" ? [r.name, ...r.classes] : r.classes;
    if (k === "layout" && !isExplained(names, p)) {
      const key = `${r.component} ${r.kind} of ${p} on .${r.classes[0] ?? "?"}`;
      if (!seen.has(key)) failures.push(`${key} (${r.row}): lays the page out every frame, and no atoll-motion comment explains it`);
      seen.add(key);
    }
  }
}
if (process.env.FRAMES_VERBOSE) {
  const props = new Map();
  for (const r of normal.runs) for (const p of r.properties) props.set(p, (props.get(p) ?? 0) + 1);
  console.log([...props].map(([p, n]) => `${p}=${n} (${cost(p)})`).join(", "));
}
console.log(
  `${ENGINE}: ${normal.runs.length} animations and transitions in ${contracts.reduce((n, c) => n + c.rows.length, 0)} contract rows: ` +
    `${byCost.composite} composite, ${byCost.paint} paint, ${byCost.layout} layout property runs`,
);

// 2. Chromium's own verdict: composite-only keyframes ran on the compositor.
if (normal.trace.length) {
  const cssAnimations = normal.runs.filter((r) => r.kind === "animation" && r.properties.every((p) => COMPOSITE.has(p)));
  const names = new Set(cssAnimations.map((r) => r.name));
  const traced = normal.trace.filter((a) => names.has(a.name));
  const off = traced.filter((a) => a.failed);
  console.log(`chromium trace: ${traced.length} composite-only CSS animations seen, ${traced.length - off.length} ran on the compositor`);
  for (const a of off) {
    failures.push(`${a.component}: animation ${a.name} fell back to the main thread (compositeFailed ${a.failed}${a.unsupported.length ? `, ${[...new Set(a.unsupported)].join(", ")}` : ""})`);
  }
  const layouts = Object.values(normal.perComponent).reduce((n, c) => n + (c.layouts ?? 0), 0);
  console.log(`chromium trace: ${layouts} layouts in total over all rows (page loads included)`);
}

// 3. Frame times while something moves.
const all = normal.frames;
const long = all.filter((d) => d > 1000 / 60 * 1.5).length;
console.log(
  `frames while animating${normal.trace.length ? " (CPU slowed 4x)" : ""}: ${all.length}, ` +
    `median ${pct(all, 0.5).toFixed(1)} ms, p95 ${pct(all, 0.95).toFixed(1)} ms, ` +
    `max ${all.length ? Math.max(...all).toFixed(1) : 0} ms, ${(all.length ? (100 * long) / all.length : 0).toFixed(1)}% over 25 ms`,
);
if (process.env.FRAME_BUDGET === "1" && pct(all, 0.95) > 34) failures.push(`p95 frame ${pct(all, 0.95).toFixed(1)} ms is over two frames`);

// 4. Reduced motion: nothing runs longer than 1 ms.
const reduced = await sweep({ reduced: true });
const slow = reduced.runs.filter((r) => r.duration > 1 && !r.infinite);
console.log(`reduced motion: ${reduced.runs.length} animations and transitions started, ${slow.length} longer than 1 ms`);
for (const r of slow.slice(0, 20)) failures.push(`reduced motion: ${r.component} ${r.kind} ${r.name} runs ${r.duration} ms`);

if (process.env.FRAMES_JSON) {
  writeFileSync(
    process.env.FRAMES_JSON,
    JSON.stringify({ engine: ENGINE, byCost, perComponent: normal.perComponent, trace: normal.trace.length, reducedRuns: reduced.runs.length }, null, 2),
  );
}
for (const f of failures) console.log(`FAIL ${f}`);
console.log(failures.length ? `${failures.length} failure(s)` : "ok: every animation is composite, paint or explained");
if (failures.length) process.exit(1);
