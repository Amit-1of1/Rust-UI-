// Blocks (ui::blocks) on their own gallery pages (#block/<name>): each passes axe in light and
// dark (settings on every tab, with its dialog open), and the main flows work by keyboard.
// Build: (cd apps/gallery && dx build --platform web); serve target/dx/gallery/debug/web/public
// Run:   GALLERY_URL=http://localhost:8765/ node blocks.mjs
import { launch } from "./engine.mjs";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";

const axeSource = readFileSync(createRequire(import.meta.url).resolve("axe-core/axe.min.js"), "utf8");
const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const BLOCKS = ["login_01", "signup_01", "dashboard_01", "settings_01"];

let failed = 0;
const check = (ok, msg) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${msg}`);
  if (!ok) failed++;
};

async function axe(page, label) {
  await page.waitForTimeout(400);
  // Scan the settled page: on a slow runner an overlay can still be fading in after the
  // fixed wait, and axe reads half-transparent text as low contrast.
  await page
    .waitForFunction(
      () =>
        !document
          .getAnimations()
          .some((a) => a.playState === "running" && a.effect?.getComputedTiming().iterations !== Infinity),
      null,
      { timeout: 3000 },
    )
    .catch(() => {});
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

// A fresh page per block: the gallery reads the hash once, on load.
async function open(browser, name, scheme = "light", viewport = { width: 1280, height: 900 }) {
  const page = await browser.newPage({ colorScheme: scheme, viewport });
  page.on("pageerror", (e) => check(false, `${name}: page error: ${e.message}`));
  await page.goto(`${url}#block/${name}`);
  await page.waitForSelector(`[data-block='${name}'] h1`);
  return page;
}

const focused = (page) =>
  page.evaluate(() => {
    const el = document.activeElement;
    return el ? `${el.tagName.toLowerCase()}:${el.getAttribute("autocomplete") ?? el.textContent.trim()}` : "";
  });

const browser = await launch();

for (const scheme of ["light", "dark"]) {
  for (const name of BLOCKS) {
    const page = await open(browser, name, scheme);
    await axe(page, `${scheme} ${name}`);
    if (name === "settings_01") {
      for (const tab of ["Notifications", "Appearance", "Account"]) {
        await page.getByRole("tab", { name: tab }).click();
        await page.keyboard.press("Enter");
        await axe(page, `${scheme} settings ${tab}`);
      }
      await page.getByRole("button", { name: "Delete account" }).click();
      await page.getByRole("alertdialog").waitFor();
      await axe(page, `${scheme} settings delete dialog`);
    }
    await page.close();
  }
}

// Phone width: nothing scrolls sideways.
for (const name of BLOCKS) {
  const page = await open(browser, name, "light", { width: 320, height: 800 });
  const wide = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
  check(wide <= 0, `${name}: no sideways scroll at 320px (${wide}px over)`);
  await page.close();
}

// ---- login ----
{
  const page = await open(browser, "login_01");
  await page.getByRole("button", { name: "Sign in" }).click();
  check((await focused(page)) === "input:email", `login: an empty submit focuses the email field (${await focused(page)})`);
  check(
    (await page.getByLabel("Email").getAttribute("aria-invalid")) === "true",
    "login: the email field is marked invalid",
  );
  await page.getByLabel("Email").fill("ada@example.com");
  await page.getByRole("button", { name: "Sign in" }).click();
  check((await focused(page)) === "input:current-password", "login: then the password field");
  await page.close();
}

// ---- sign-up ----
{
  const page = await open(browser, "signup_01");
  await page.getByLabel("Name").fill("Ada");
  await page.getByLabel("Email").fill("ada@example.com");
  await page.getByLabel("Password", { exact: true }).fill("correct horse");
  // Tick the box before typing the mismatch: leaving a field with an error shows its message,
  // which moves everything below it between mousedown and click.
  await page.getByLabel("I agree to the terms of service").click();
  await page.getByLabel("Confirm password").fill("correct hose");
  await page.getByLabel("Confirm password").press("Enter");
  check((await focused(page)) === "input:new-password", "sign-up: mismatched passwords focus the confirm field");
  check(
    (await page.locator("[data-block='signup_01']").textContent()).includes("The passwords don't match."),
    "sign-up: the mismatch is explained",
  );
  await page.getByLabel("Confirm password").fill("correct horse");
  await page.getByLabel("Confirm password").press("Enter");
  await page.getByRole("heading", { name: "Check your email" }).waitFor();
  check((await focused(page)) === "h1:Check your email", `sign-up: the code step's heading takes focus (${await focused(page)})`);
  await axe(page, "light sign-up code step");
  await page.getByRole("button", { name: "Verify" }).click();
  check(
    (await page.locator("[data-block='signup_01'] [role='alert']").textContent()).includes("Enter all 6 digits."),
    "sign-up: a short code is reported",
  );
  await page.getByRole("button", { name: "Use a different email" }).click();
  await page.getByRole("heading", { name: "Create an account" }).waitFor();
  check(
    (await page.getByLabel("Email").inputValue()) === "ada@example.com",
    "sign-up: going back keeps what was typed",
  );
  await page.close();
}

// ---- dashboard ----
{
  const page = await open(browser, "dashboard_01");
  const h1 = () => page.locator("[data-block='dashboard_01'] h1").textContent();
  check((await h1()) === "Overview", "dashboard: starts on Overview");
  await page.locator("[data-sidebar='menu-button']", { hasText: "Customers" }).click();
  check((await h1()) === "Customers", "dashboard: the sidebar changes the page and its h1");
  check(
    (await page.locator("[aria-current='page']").textContent()).includes("Customers"),
    "dashboard: the current page is aria-current",
  );
  await page.locator("[data-sidebar='menu-button']", { hasText: "Overview" }).click();
  await page.getByRole("button", { name: "View all" }).click();
  check((await h1()) === "Orders", "dashboard: View all opens Orders");
  await page.close();
}

// ---- settings ----
{
  const page = await open(browser, "settings_01");
  await page.getByLabel("Name").fill("Ada");
  await page.getByLabel("Email").fill("ada@example.com");
  await page.getByRole("button", { name: "Save" }).click();
  check((await page.getByRole("status").first().textContent()) === "Saved.", "settings: Save says Saved.");
  await page.getByLabel("Name").fill("Ada L");
  check((await page.getByRole("status").first().textContent()) === "", "settings: editing clears Saved.");
  await page.getByRole("tab", { name: "Account" }).click();
  await page.keyboard.press("Enter");
  const trigger = page.getByRole("button", { name: "Delete account" });
  await trigger.click();
  await page.getByRole("alertdialog").waitFor();
  await page.keyboard.press("Escape");
  await page.waitForTimeout(300);
  check((await page.getByRole("alertdialog").count()) === 0, "settings: Escape closes the confirmation");
  await page.close();
}

await browser.close();
console.log(failed ? `${failed} check(s) failed` : "all block checks passed");
process.exit(failed ? 1 : 0);
