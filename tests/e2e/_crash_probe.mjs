// Temporary diagnostic: how often WebKit traps while starting the gallery, per JSC wasm tier.
import { webkit } from "playwright";
const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const runs = Number(process.env.RUNS ?? 15);
const variants = {
  default: {},
  "no fast memory": { JSC_useWebAssemblyFastMemory: "false" },
  "no OMG": { JSC_useOMGJIT: "false" },
  "no BBQ": { JSC_useBBQJIT: "false" },
};
for (const [name, extra] of Object.entries(variants)) {
  const browser = await webkit.launch({ env: { ...process.env, ...extra } });
  let crashed = 0;
  const first = new Set();
  for (let i = 0; i < runs; i++) {
    const page = await browser.newPage();
    const errors = [];
    page.on("pageerror", (e) => errors.push(e));
    await page.goto(url);
    await page.waitForSelector(".gallery-main", { timeout: 20000 }).catch(() => {});
    await page.waitForTimeout(1500);
    if (errors.length) {
      crashed++;
      first.add(`${errors[0].message}\n${(errors[0].stack ?? "").split("\n").slice(0, 6).join("\n")}`);
    }
    await page.close();
  }
  console.log(`${name}: ${crashed}/${runs} loads trapped`);
  for (const f of first) console.log(`  ${f.replace(/\n/g, "\n  ")}`);
  await browser.close();
}
