// Runs every component contract (crates/ui/src/components/<name>/contract.toml) in the browser,
// against that component's docs page, where its demo is the only thing in `.docs-preview`. Each
// row of a contract's keyboard table is one test here, so the docs can't list a key these
// tests don't press. The same rows run on native in apps/native/tests/contracts.rs.
// Format: docs/CONTRACTS.md.
//   GALLERY_URL=<docs site> node contracts.mjs [component]
// The steps come from `atoll contract --json` (run through cargo), or CONTRACTS_JSON=<file>.
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { launch } from "./engine.mjs";

const url = process.env.GALLERY_URL ?? "http://localhost:8769/";
const only = process.argv[2];
const json = process.env.CONTRACTS_JSON
  ? readFileSync(process.env.CONTRACTS_JSON, "utf8")
  : execFileSync("cargo", ["run", "-q", "-p", "atoll-cli", "--", "contract", "--json"], {
      cwd: new URL("../..", import.meta.url).pathname,
      encoding: "utf8",
    });
const contracts = JSON.parse(json).filter((c) => !only || c.name === only);

const browser = await launch();
const page = await browser.newPage({ viewport: { width: 1100, height: 900 } });

let failed = 0;
let passed = 0;
// Assertions retry for a while: overlays animate in and out, and focus moves after a render.
const WAIT_MS = 1500;
async function until(probe) {
  const end = Date.now() + WAIT_MS;
  for (;;) {
    if (await probe().catch(() => false)) return true;
    if (Date.now() > end) return false;
    await page.waitForTimeout(50);
  }
}

const describe = (at) => (at.nth === 1 ? at.part : `${at.part}@${at.nth}`);
const scope = () => page.locator(".docs-preview");
const locate = (at) => scope().locator(at.selector).nth(at.nth - 1);

async function step(s) {
  switch (s.op) {
    case "focus":
      await locate(s.at).focus();
      return;
    case "click":
      await locate(s.at).click();
      return;
    case "press":
      for (let i = 0; i < s.times; i++) {
        await page.keyboard.press(s.key);
        await page.waitForTimeout(60);
      }
      return;
    case "type":
      await page.keyboard.type(s.text);
      return;
  }
  const loc = locate(s.at);
  const checks = {
    visible: [() => loc.isVisible(), `${describe(s.at)} is not visible`],
    hidden: [async () => !(await loc.isVisible()), `${describe(s.at)} is still visible`],
    focused: [
      () => loc.evaluate((el) => el === document.activeElement),
      `${describe(s.at)} does not have focus`,
    ],
    "focus-in": [
      () => loc.evaluate((el) => el.contains(document.activeElement)),
      `focus is not inside ${describe(s.at)}`,
    ],
    text: [
      async () => (await loc.textContent()).includes(s.text),
      `${describe(s.at)} does not say "${s.text}"`,
    ],
    attr: [
      async () => ((await loc.getAttribute(s.name)) === s.value) !== s.negate,
      `${describe(s.at)}: ${s.name} is not ${s.negate ? "different from " : ""}${s.value}`,
    ],
  };
  const [probe, message] = checks[s.op] ?? [];
  if (!probe) throw new Error(`unknown step ${s.op}`);
  if (!(await until(probe))) {
    let actual = "";
    if (s.op === "text") actual = ` (it says "${(await loc.textContent().catch(() => "")).trim()}")`;
    if (s.op === "attr") actual = ` (it is ${await loc.getAttribute(s.name).catch(() => "missing")})`;
    if (s.op === "focused" || s.op === "focus-in") {
      actual = ` (focus is on ${await page.evaluate(() => {
        const el = document.activeElement;
        return el ? `<${el.tagName.toLowerCase()}> "${el.textContent.trim().slice(0, 30)}"` : "nothing";
      })})`;
    }
    throw new Error(message + actual);
  }
}

let fresh = 0;
for (const c of contracts) {
  for (const row of c.rows) {
    // A fresh page per row, so each row starts from the demo's initial state.
    await page.goto(`${url}?row=${fresh++}#${c.name}`);
    await page.waitForSelector(".docs-preview");
    await page.waitForTimeout(150);
    try {
      for (const s of row.steps) await step(s);
      passed++;
      console.log(`ok   ${c.name} ${row.key}: ${row.does}`);
    } catch (e) {
      failed++;
      console.log(`FAIL ${c.name} ${row.key}: ${e.message.split("\n")[0]}`);
    }
  }
}

await browser.close();
console.log(`${passed} contract row(s) passed, ${failed} failed`);
if (failed) process.exit(1);
