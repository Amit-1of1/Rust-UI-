// Usage: serve the built gallery (dx build --platform web; serve target/dx/gallery/debug/web/public),
// (set ACCENT=blue|green|... to scan an accent preset), then `GALLERY_URL=http://localhost:8765/ node axe.mjs [light|dark]`.
// For apps/site: `GALLERY_URL=http://localhost:8767/ READY_SELECTOR=#pricing node axe.mjs`.
import { launch } from "./engine.mjs";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";

const axeSource = readFileSync(createRequire(import.meta.url).resolve("axe-core/axe.min.js"), "utf8");
const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const scheme = process.argv[2] ?? "light";
// Known, accepted findings. Every entry needs a reason; keep this list short.
const ACCEPTED = {
  // The sidebar demo renders its own <main>; inside the gallery it is nested in a labelled region.
  "landmark-main-is-top-level": "sidebar demo is embedded in the gallery page",
};

const browser = await launch();
const page = await browser.newPage({ colorScheme: scheme, viewport: { width: 1100, height: 900 } });
const logs = [];
page.on("pageerror", (e) => logs.push(`pageerror: ${e.message}`));
page.on("console", (m) => m.type() === "error" && logs.push(`console: ${m.text()}`));
await page.goto(url);
try {
  await page.waitForSelector(process.env.READY_SELECTOR ?? ".gallery-main > .gallery-section:last-child");
} catch (e) {
  console.log(`${url} did not render:\n  ${logs.slice(0, 10).join("\n  ") || "(no errors logged)"}`);
  throw e;
}
if (process.env.ACCENT) {
  await page.evaluate((a) => document.documentElement.setAttribute("data-accent", a), process.env.ACCENT);
}
if (process.env.DIR) {
  await page.evaluate((d) => document.documentElement.setAttribute("dir", d), process.env.DIR);
}
if (process.env.SHOT) await page.screenshot({ path: process.env.SHOT, fullPage: true });
await page.waitForTimeout(1500);
await page.evaluate(axeSource);
const result = await page.evaluate(() =>
  axe.run(document, { runOnly: ["wcag2a", "wcag2aa", "wcag21aa", "best-practice"] }),
);
const bad = result.violations.filter((v) => !(v.id in ACCEPTED));
for (const v of bad) {
  console.log(`${v.impact}\t${v.id}\t${v.help}`);
  for (const n of v.nodes.slice(0, 3)) console.log(`   ${n.target.join(" ")}`);
}
console.log(`${scheme}${process.env.ACCENT ? " / " + process.env.ACCENT : ""}${process.env.DIR ? " / " + process.env.DIR : ""}: ${bad.length} violation type(s), ${result.passes.length} rules passed`);
await browser.close();
process.exit(bad.length ? 1 : 0);
