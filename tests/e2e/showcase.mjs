// apps/showcase end to end: the same pages at a desktop and a phone size, light and dark. Each
// page passes axe, the layout swaps the side rail for a bottom tab bar on the phone, the form
// validates and saves, and the overlays open. apps/native/tests/showcase.rs checks the same
// things on the native renderer.
// Build: (cd apps/showcase && dx build --platform web); serve target/dx/showcase/debug/web/public
// Run:   GALLERY_URL=http://localhost:8775/ node showcase.mjs   (SHOTS=dir saves screenshots)
import { launch } from "./engine.mjs";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";

const axeSource = readFileSync(createRequire(import.meta.url).resolve("axe-core/axe.min.js"), "utf8");
const url = process.env.GALLERY_URL ?? "http://localhost:8775/";
const shots = process.env.SHOTS;

let failed = 0;
const check = (ok, msg) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${msg}`);
  if (!ok) failed++;
};

async function axe(page, label) {
  await page.waitForTimeout(400);
  await page.evaluate(axeSource);
  const result = await page.evaluate(() =>
    axe.run(document, { runOnly: ["wcag2a", "wcag2aa", "wcag21aa", "best-practice"] }),
  );
  for (const v of result.violations) {
    console.log(`     ${v.impact}\t${v.id}\t${v.help}`);
    for (const n of v.nodes.slice(0, 3)) console.log(`        ${n.target.join(" ")}`);
  }
  check(result.violations.length === 0, `axe ${label}: ${result.violations.length} violation type(s)`);
}

const shot = async (page, name) => {
  if (!shots) return;
  await page.waitForTimeout(600); // let enter animations finish
  await page.screenshot({ path: `${shots}/${name}.png` });
};

const SIZES = {
  desktop: { width: 1100, height: 780 },
  phone: { width: 390, height: 844 },
};

const browser = await launch();
for (const [size, viewport] of Object.entries(SIZES)) {
  for (const scheme of ["light", "dark"]) {
    const tag = `${size} ${scheme}`;
    const page = await browser.newPage({ colorScheme: scheme, viewport });
    page.on("pageerror", (e) => check(false, `${tag}: page error: ${e.message}`));
    await page.goto(url);
    await page.waitForSelector("body[data-ready='1'] .sc-main");
    check((await page.locator("#renderer").textContent()) === "Web (WebAssembly)", `${tag}: says it runs on the web`);

    const rail = await page.locator(".sc-rail").isVisible();
    const bar = await page.locator(".sc-bottom").isVisible();
    check(size === "desktop" ? rail && !bar : !rail && bar, `${tag}: ${size === "desktop" ? "side rail" : "bottom tab bar"} shown`);
    const nav = size === "desktop" ? page.locator(".sc-rail") : page.locator(".sc-bottom nav");

    for (const name of ["Forms", "Overlays", "Settings", "Home"]) {
      await nav.getByRole("button", { name }).click();
      check((await page.locator("h1.sc-title").textContent()) === name, `${tag}: opens ${name}`);
      check(
        (await nav.locator("[aria-current='page']").textContent()).includes(name),
        `${tag}: ${name} is aria-current`,
      );
      await shot(page, `web-${size}-${scheme}-${name.toLowerCase()}`);
      await axe(page, `${tag} ${name}`);
    }
    const width = await page.evaluate(() => document.documentElement.scrollWidth);
    check(width <= viewport.width, `${tag}: no sideways scrolling (${width}px)`);

    // Form: errors first, then a saved summary and a toast.
    await nav.getByRole("button", { name: "Forms" }).click();
    await page.locator("#save").click();
    check((await page.locator("input[aria-invalid='true']").count()) === 2, `${tag}: empty form shows two errors`);
    await page.getByLabel("Workspace name").fill("Acme");
    await page.getByLabel("Billing email").fill("ops@acme.dev");
    await page.locator("#save").click();
    check((await page.locator("#saved").textContent()).includes("Acme on Team, 5 seats"), `${tag}: form saves`);
    check(await page.getByText("Saved", { exact: true }).isVisible(), `${tag}: success toast shown`);

    // Overlays, each scanned while open.
    await nav.getByRole("button", { name: "Overlays" }).click();
    for (const [trigger, label] of [
      ["#open-dialog", "dialog"],
      ["#open-sheet", "sheet"],
    ]) {
      await page.locator(trigger).click();
      await page.getByRole("dialog").waitFor();
      await shot(page, `web-${size}-${scheme}-${label}`);
      await axe(page, `${tag} ${label} open`);
      await page.keyboard.press("Escape");
      await page.getByRole("dialog").waitFor({ state: "detached" }).catch(() => {});
    }
    await page.close();
  }
}
await browser.close();

if (failed) {
  console.log(`\n${failed} check(s) failed`);
  process.exit(1);
}
console.log("\nshowcase checks passed");
