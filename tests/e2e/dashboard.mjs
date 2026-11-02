// apps/dashboard end to end: every page and every overlay passes axe in light and dark, and
// the main flows work by keyboard and mouse.
// Build: (cd apps/dashboard && dx build --platform web); serve target/dx/dashboard/debug/web/public
// Run:   GALLERY_URL=http://localhost:8771/ node dashboard.mjs
import { launch } from "./engine.mjs";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";

const axeSource = readFileSync(createRequire(import.meta.url).resolve("axe-core/axe.min.js"), "utf8");
const url = process.env.GALLERY_URL ?? "http://localhost:8771/";

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

const nav = (page, name) => page.locator("[data-sidebar='menu-button']", { hasText: name }).click();
const heading = (page) => page.locator("h1.db-title").textContent();

const browser = await launch();
for (const scheme of ["light", "dark"]) {
  const page = await browser.newPage({ colorScheme: scheme, viewport: { width: 1280, height: 900 } });
  page.on("pageerror", (e) => check(false, `page error: ${e.message}`));
  await page.goto(url);
  await page.waitForSelector("body[data-ready='1'] .db-main");
  // The page follows the system scheme and the switch reflects it; clicking it here would
  // turn dark mode off and scan the light theme twice.
  const dark = await page.getByRole("switch", { name: "Dark mode" }).getAttribute("aria-checked");
  check(dark === String(scheme === "dark"), `${scheme}: the dark mode switch follows the system (${dark})`);

  for (const name of ["Overview", "Customers", "Orders", "Settings"]) {
    await nav(page, name);
    check((await heading(page)) === name, `${scheme}: sidebar opens ${name}`);
    check(
      (await page.locator("[aria-current='page'][data-sidebar='menu-button']").textContent()).includes(name),
      `${scheme}: ${name} is aria-current in the sidebar`,
    );
    await axe(page, `${scheme} ${name}`);
  }

  // Charts: named images with a keyboard-reachable data table.
  await nav(page, "Overview");
  for (const name of [/^Revenue per month/, /^Customers by plan$/, /^Q3 revenue against target$/]) {
    check((await page.getByRole("img", { name }).count()) === 1, `${scheme}: chart ${name} is a named image`);
  }
  check((await page.locator("[data-slot='chart-legend'] li").count()) >= 5, `${scheme}: charts show legends`);
  const toggle = page.locator("[data-slot='chart-data'] summary").first();
  await toggle.focus();
  await page.keyboard.press("Enter");
  const table = page.getByRole("table", { name: /^Revenue per month/ });
  check(await table.isVisible(), `${scheme}: Enter on "Show data" opens the revenue table`);
  check((await table.locator("tbody tr").count()) === 9, `${scheme}: the revenue table has a row per month`);
  await axe(page, `${scheme} Overview with a chart table open`);
  await page.keyboard.press("Enter");

  // Overlays: each one is scanned while open.
  await nav(page, "Customers");
  await page.getByRole("button", { name: "Add customer" }).click();
  await page.getByRole("dialog").waitFor();
  await axe(page, `${scheme} add-customer dialog`);
  await page.keyboard.press("Escape");

  await page.getByRole("button", { name: /^Actions for / }).first().click();
  await page.waitForTimeout(200);
  await axe(page, `${scheme} row menu`);
  await page.keyboard.press("Escape");

  await page.keyboard.press("Control+k");
  await page.getByRole("combobox", { name: /search commands/i }).waitFor();
  await axe(page, `${scheme} command palette`);
  await page.keyboard.press("Escape");
  await page.close();
}

// ---- flows (light) ----
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
await page.goto(url);
await page.waitForSelector("body[data-ready='1'] .db-main");
const status = () => page.locator(".db-table-footer [role='status']").textContent();

// Ctrl+K navigates.
// In a split build the palette and the Customers page load on first use: wait for each.
await page.keyboard.press("Control+k");
await page.getByRole("combobox", { name: /search commands/i }).waitFor();
// Focus moves in after the open dialog mounts; keys typed before that are lost.
const focused = await page.waitForFunction(() => document.activeElement?.getAttribute("role") === "combobox",
  null, { timeout: 10000 }).then(() => true, () => false);
check(focused, "palette: focus moves to the search box");
await page.keyboard.type("customers");
await page.keyboard.press("Enter");
await page.waitForFunction(() => document.querySelector("h1.db-title")?.textContent === "Customers",
  null, { timeout: 10000 }).catch(() => {});
const after = await heading(page);
check(after === "Customers", `palette: 'customers' + Enter opens Customers (${after})`);
const before = await status();
check(/of 48 customers/.test(before), `48 customers listed (${before})`);

// Add customer: an empty submit shows errors and focuses the first invalid field.
await page.getByRole("button", { name: "Add customer" }).click();
const dialog = page.getByRole("dialog");
await dialog.waitFor();
await dialog.getByRole("button", { name: "Add customer" }).click();
check(
  await dialog.getByLabel("Name").evaluate((el) => el === document.activeElement && el.getAttribute("aria-invalid") === "true"),
  "empty submit focuses the invalid Name field",
);
await dialog.getByLabel("Name").fill("Edsger Dijkstra");
await dialog.getByLabel("Email").fill("edsger@example.com");
await dialog.getByRole("button", { name: "Add customer" }).click();
await page.waitForTimeout(300);
check(!(await dialog.isVisible()), "valid submit closes the dialog");
check(/of 49 customers/.test(await status()), "customer count goes to 49");
check(await page.getByText("Customer added").isVisible(), "toast confirms the add");

// Filter, then delete via the row menu and the confirmation dialog.
await page.getByRole("searchbox", { name: "Filter customers" }).fill("dijkstra");
await page.waitForTimeout(200);
check(/of 1 customers/.test(await status()), "filter narrows to the new customer");
await page.getByRole("button", { name: "Actions for Edsger Dijkstra" }).click();
await page.getByRole("option", { name: "Delete" }).click();
const alert = page.getByRole("alertdialog", { name: "Delete customer?" });
await alert.waitFor();
check((await alert.textContent()).includes("Edsger Dijkstra"), "confirmation names the customer");
await alert.getByRole("button", { name: "Delete" }).click();
await page.waitForTimeout(300);
check(await page.getByText("No customers match").isVisible(), "empty state after deleting the only match");
// The "deleted" toast is a status message, not a second alertdialog (GAPS.md 13).
check((await page.getByRole("alertdialog").count()) === 0, "the toast is not an alertdialog");
await page.getByRole("button", { name: "Clear filters" }).click();
check(/of 48 customers/.test(await status()), "clear filters shows all 48 again");

// Edit in the sheet.
await page.getByRole("button", { name: /^Actions for / }).first().click();
await page.getByRole("option", { name: "Edit" }).click();
const sheet = page.getByRole("dialog");
await sheet.waitFor();
check((await sheet.getByLabel("Name").inputValue()).length > 0, "sheet loads the customer's name");
await axe(page, "edit sheet");
check((await sheet.getByRole("button", { name: /^Plan / }).count()) === 1, "the plan select is named by its label and value");
await sheet.getByLabel("Name").fill("Renamed Person");
await sheet.getByRole("button", { name: "Save changes" }).click();
await page.waitForTimeout(300);
check(await page.getByRole("cell", { name: "Renamed Person", exact: true }).isVisible(), "edit is saved to the table");

// Orders: the status toggles filter the DataTable.
await nav(page, "Orders");
const ordersStatus = () => page.locator("[role='status']").filter({ hasText: /rows/ }).first().textContent();
const all = await ordersStatus();
await page.getByRole("button", { name: "Refunded" }).click();
await page.waitForTimeout(200);
const fewer = await ordersStatus();
check(all !== fewer, `turning off Refunded changes the rows (${all} -> ${fewer})`);

// Settings: the time zone is a labelled form field with its initial value.
await nav(page, "Settings");
const zone = page.getByRole("combobox", { name: "Time zone" });
// Split release builds load the Settings page on first open.
await zone.first().waitFor({ timeout: 10000 }).catch(() => {});
check((await zone.count()) === 1, "time zone combobox is named by its label");

// One theme state: the header switch and the customizer agree both ways.
const darkSwitch = page.getByRole("switch", { name: "Dark mode" });
await darkSwitch.click();
await page.waitForTimeout(200);
check(await page.getByRole("radio", { name: "Dark" }).isChecked(), "header switch on: customizer shows Dark");
check((await page.evaluate(() => document.documentElement.dataset.theme)) === "dark", "data-theme is dark");
// From the keyboard: the radio input is visually hidden, and a forced click on it does
// nothing in WebKit. Space checks the focused radio.
await page.getByRole("radio", { name: "Light" }).focus();
await page.keyboard.press("Space");
await page.waitForTimeout(200);
check((await darkSwitch.getAttribute("aria-checked")) === "false", "customizer Light: header switch turns off");

// Settings: the reset confirmation is an alertdialog and passes axe.
await page.getByRole("button", { name: "Reset workspace" }).click();
await page.getByRole("alertdialog", { name: "Reset the workspace?" }).waitFor();
await axe(page, "reset confirmation");
await page.keyboard.press("Escape");

// Reduced motion: chart marks don't animate in.
{
  const page = await browser.newPage({ reducedMotion: "reduce" });
  await page.goto(url);
  await page.waitForSelector("body[data-ready='1'] .db-main");
  const motion = await page.evaluate(() => {
    const slice = document.querySelector("[data-slot='chart'] path[data-series]");
    return slice ? getComputedStyle(slice).animationName : "missing";
  });
  check(motion === "none", `reduced motion: chart slices don't animate (${motion})`);
  await page.close();
}

await browser.close();
if (failed) {
  console.error(`${failed} dashboard check(s) failed`);
  process.exit(1);
}
console.log("dashboard: all checks passed");
