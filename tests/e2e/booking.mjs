// apps/booking end to end: every route and overlay passes axe in light and dark, and the
// search, filter, booking, checkout and cancel flows work.
// Build: (cd apps/booking && dx build --platform web); serve target/dx/booking/debug/web/public
// Run:   GALLERY_URL=http://localhost:8773/ node booking.mjs
import { launch } from "./engine.mjs";
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";

const axeSource = readFileSync(createRequire(import.meta.url).resolve("axe-core/axe.min.js"), "utf8");
const url = process.env.GALLERY_URL ?? "http://localhost:8773/";

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

const ready = (page) => page.waitForSelector("body[data-ready='1'] .tr-main");
const h1 = (page) => page.locator("main h1").first().textContent();
const count = (page) => page.locator(".tr-results-bar [role='status']").textContent();
const nav = (page, name) => page.getByRole("navigation", { name: "Main" }).getByRole("link", { name }).click();

// Type a date into a picker's segments. `nth` 0 is a range's start, 1 its end.
async function typeDate(scope, date, nth = 0) {
  const [y, m, d] = date.split("-");
  await scope.getByRole("spinbutton", { name: "year" }).nth(nth).focus();
  await scope.page().keyboard.type(y);
  await scope.getByRole("spinbutton", { name: "month" }).nth(nth).focus();
  await scope.page().keyboard.type(m);
  await scope.getByRole("spinbutton", { name: "day" }).nth(nth).focus();
  await scope.page().keyboard.type(d);
}

const iso = (days) => {
  const t = new Date();
  t.setUTCDate(t.getUTCDate() + days);
  return t.toISOString().slice(0, 10);
};

const browser = await launch();
for (const scheme of ["light", "dark"]) {
  const page = await browser.newPage({ colorScheme: scheme, viewport: { width: 1280, height: 900 } });
  page.on("pageerror", (e) => check(false, `page error: ${e.message}`));
  await page.goto(url);
  await ready(page);
  // The page follows the system scheme; the switch reflects it. (Clicking it here would turn
  // dark mode *off* and scan the light theme twice.)
  const dark = await page.getByRole("switch", { name: "Dark mode" }).getAttribute("aria-checked");
  check(dark === String(scheme === "dark"), `${scheme}: the dark mode switch follows the system (${dark})`);

  await axe(page, `${scheme} explore`);
  await page.locator(".tr-grid a").first().click();
  await page.locator(".tr-stay").waitFor();
  await axe(page, `${scheme} stay`);
  for (const tab of ["Amenities", "Reviews"]) {
    await page.getByRole("tab", { name: tab }).click();
    await axe(page, `${scheme} stay ${tab} tab`);
  }
  await page.getByRole("button", { name: "Cancellation" }).click();
  await axe(page, `${scheme} stay accordion open`);

  await nav(page, "My trips");
  await axe(page, `${scheme} trips`);
  await page.getByRole("button", { name: /^Cancel trip to / }).first().click();
  await page.getByRole("alertdialog", { name: "Cancel this trip?" }).waitFor();
  await axe(page, `${scheme} cancel dialog`);
  await page.keyboard.press("Escape");

  await page.keyboard.press("Control+k");
  await page.getByRole("combobox", { name: /search commands/i }).waitFor();
  await axe(page, `${scheme} command palette`);
  await page.keyboard.press("Escape");
  await page.close();
}

// ---- flows (light) ----
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
page.on("pageerror", (e) => check(false, `page error: ${e.message}`));
await page.goto(url);
await ready(page);
check((await page.title()) === "Explore stays · Trailhead", `document title per route (${await page.title()})`);

// Search and filters.
const all = await count(page);
check(all === "24 stays", `24 stays listed (${all})`);
await page.getByRole("searchbox", { name: "Where" }).fill("lake");
await page.waitForTimeout(200);
const lake = await count(page);
check(lake !== all, `text search narrows the list (${lake})`);
await page.getByRole("searchbox", { name: "Where" }).fill("");
await page.getByRole("button", { name: "Treehouse" }).click();
await page.waitForTimeout(200);
const titles = await page.locator(".tr-card-title").allTextContents();
check(titles.length > 0 && titles.every((t) => t.includes("Treehouse")), `type filter keeps treehouses (${titles.length})`);
await page.getByRole("checkbox", { name: "Hot tub" }).click();
// Guests is the kit's NumberInput: a spin button named by its label, -/+ buttons, arrow keys.
const guests = page.getByRole("spinbutton", { name: "Guests" }).first();
check((await guests.getAttribute("aria-valuenow")) === "2", "guests is a spin button at 2");
await guests.focus();
await page.keyboard.press("ArrowUp");
check((await guests.getAttribute("aria-valuenow")) === "3", "ArrowUp adds a guest");
await page.getByRole("button", { name: "Increase guests" }).first().click();
await page.getByRole("button", { name: "Increase guests" }).first().click();
await page.getByRole("button", { name: "Increase guests" }).first().click();
check((await guests.getAttribute("aria-valuenow")) === "6", "+ adds guests up to the maximum");
check(await page.getByRole("button", { name: "Increase guests" }).first().isDisabled(), "+ is disabled at the maximum");
await page.waitForTimeout(200);
const narrowed = await count(page);
check(/^\d+ stays?$/.test(narrowed), `filters combine (${narrowed})`);
if (narrowed === "0 stays") {
  check(await page.getByText("No stays match").isVisible(), "empty state when nothing matches");
}
await page.getByRole("button", { name: "Clear filters" }).click();
await page.waitForTimeout(200);
check((await count(page)) !== narrowed || narrowed === all, "clear filters widens the list again");
// Back to 2 guests for the booking below.
await guests.focus();
await page.keyboard.press("Home");
await page.getByRole("button", { name: "Increase guests" }).first().click();
check((await guests.getAttribute("aria-valuenow")) === "2", "Home goes to 1, + back to 2");

// Sort by price: the first card is the cheapest.
await page.getByRole("button", { name: /^Sort by/ }).click();
await page.getByRole("option", { name: "Price: low to high" }).click();
await page.waitForTimeout(200);
const prices = (await page.locator(".tr-price strong").allTextContents()).map((p) => Number(p.replace(/[$,]/g, "")));
check(prices.every((p, i) => i === 0 || prices[i - 1] <= p), `sorted by price (${prices.slice(0, 4).join(", ")}…)`);

// Paging: the kit's Pager renders buttons; Previous does nothing on page 1.
const pages = page.getByRole("navigation", { name: "Pages of stays" });
check(
  (await pages.getByRole("button", { name: "Go to previous page" }).getAttribute("aria-disabled")) === "true",
  "Previous is disabled on page 1",
);
await pages.getByRole("button", { name: "2", exact: true }).click();
await page.waitForTimeout(200);
check((await pages.getByRole("button", { name: "2", exact: true }).getAttribute("aria-current")) === "page", "page 2 is current");

// Open the cheapest stay through the command palette.
await page.keyboard.press("Control+k");
await page.keyboard.type("Fern Hollow");
await page.keyboard.press("Enter");
await page.locator(".tr-stay").waitFor();
const stayName = await h1(page);
check(stayName.startsWith("Fern Hollow"), `palette opens a stay (${stayName})`);
check((await page.title()).startsWith("Fern Hollow"), "stay page sets the title");

// Booking box: no dates, no reserve.
const reserve = page.getByRole("button", { name: "Reserve" });
check(await reserve.isDisabled(), "Reserve waits for dates");
const box = page.locator(".tr-book");
// The range picker is named by its visible label (no hard-coded "Date Range"), and its
// segments have no dangling aria-labelledby.
check((await box.getByRole("group", { name: "Dates" }).count()) === 1, "the date range picker is named Dates");
check((await box.locator("span[role='spinbutton'][aria-labelledby]").count()) === 0, "date segments have no dangling aria-labelledby");
// "0" then "1" fills a month: focus moves on to the day.
await box.getByRole("spinbutton", { name: "month" }).nth(0).focus();
await page.keyboard.type("01");
check(
  (await page.evaluate(() => document.activeElement?.getAttribute("aria-label"))) === "day",
  "a full two-digit month moves on to the day",
);
await typeDate(box, iso(20), 0);
await typeDate(box, iso(23), 1);
await page.waitForTimeout(200);
check((await box.locator(".tr-quote-total dd").textContent()).startsWith("$"), "a total appears once dates are set");
await axe(page, "stay with a price breakdown");
await reserve.click();
await page.locator(".tr-checkout").waitFor();
check((await page.title()).startsWith("Checkout"), "Reserve opens checkout");

// Step 1 carries the dates over; non-refundable lowers the total.
const summaryTotal = () => page.locator(".tr-summary .tr-quote-total dd").textContent();
const flexibleTotal = await summaryTotal();
await page.getByRole("radio", { name: /^Non-refundable/ }).click();
await page.waitForTimeout(100);
check((await summaryTotal()) !== flexibleTotal, `rate changes the total (${flexibleTotal} -> ${await summaryTotal()})`);
check(
  (await page.locator("[aria-current='step']").textContent()).includes("Trip"),
  "stepper marks Trip as the current step",
);
check(await page.getByRole("button", { name: /^Payment/ }).isDisabled(), "later steps can't be skipped to");
await axe(page, "checkout trip step");
await page.getByRole("button", { name: "Continue" }).click();

// Step 2: empty submit focuses the first invalid field; terms are required too.
await page.getByLabel("Full name").waitFor();
await axe(page, "checkout details step");
await page.getByRole("button", { name: "Continue" }).click();
check(
  await page.getByLabel("Full name").evaluate((el) => el === document.activeElement && el.getAttribute("aria-invalid") === "true"),
  "empty submit focuses the invalid name field",
);
await axe(page, "checkout details errors");
await page.getByLabel("Full name").fill("Grace Hopper");
await page.getByLabel("Email").fill("grace@example.com");
await page.getByLabel("Phone").fill("+1 555 0100");
await page.getByRole("button", { name: /^Country/ }).click();
await page.getByRole("option", { name: "Norway" }).click();
await page.getByLabel("Requests for the host (optional)").fill("Arriving late, around 9 pm.");
await page.getByRole("button", { name: "Continue" }).click();
check(await page.getByText("Accept the house rules to continue.").isVisible(), "terms must be accepted");
const terms = page.getByRole("checkbox", { name: /house rules/ });
check(
  await terms.evaluate((el) => el === document.activeElement && el.getAttribute("aria-invalid") === "true"),
  "the unticked terms box is focused and invalid",
);
await terms.click();
check(!(await page.getByText("Accept the house rules to continue.").isVisible()), "ticking it clears the error");
await page.getByRole("button", { name: "Continue" }).click();

// Step 3: card checks, then confirm.
await page.getByLabel("Card number").waitFor();
check((await page.getByLabel("Card number").getAttribute("inputmode")) === "numeric", "card field passes inputmode through");
await page.getByLabel("Name on card").fill("Grace Hopper");
await page.getByLabel("Card number").fill("4242");
await page.getByLabel("Expiry").fill("13/40");
await page.getByLabel("Security code").fill("12");
await page.getByRole("button", { name: /^Confirm and pay/ }).click();
check(
  await page.getByLabel("Card number").evaluate((el) => el === document.activeElement),
  "a bad card number is focused first",
);
await axe(page, "checkout payment errors");
await page.getByLabel("Card number").fill("4242 4242 4242 4242");
await page.getByLabel("Expiry").fill("04/29");
await page.getByLabel("Security code").fill("123");
await page.getByRole("button", { name: /^Confirm and pay/ }).click();
await page.waitForTimeout(300);
check((await h1(page)) === "My trips", "confirming opens My trips");
check(await page.getByText(`Booked ${stayName}`).isVisible(), "toast confirms the booking");
// A toast is a status message, not a dialog.
check(
  (await page.getByRole("status").filter({ hasText: `Booked ${stayName}` }).count()) === 1,
  "the toast is announced as a status",
);
check((await page.getByRole("alertdialog").count()) === 0, "no alertdialog while the toast shows");
check(
  (await page.getByRole("tabpanel").first().textContent()).includes(stayName),
  "the new trip is listed under Upcoming",
);

// Cancel it.
await page.getByRole("button", { name: `Cancel trip to ${stayName}` }).click();
const dialog = page.getByRole("alertdialog", { name: "Cancel this trip?" });
await dialog.waitFor();
check((await dialog.textContent()).includes(stayName), "the dialog names the stay");
await dialog.getByRole("button", { name: "Cancel trip" }).click();
await page.waitForTimeout(300);
check(
  (await page.getByRole("button", { name: `Cancel trip to ${stayName}` }).count()) === 0,
  "the cancelled trip leaves Upcoming",
);
await page.getByRole("tab", { name: /^Past/ }).click();
check(await page.getByText("Cancelled").first().isVisible(), "it shows as Cancelled under Past");

// After in-app navigation focus moves to <main> (the kit's MainContent); on a fresh load
// the first Tab reaches the skip link.
await nav(page, "Explore");
await page.waitForTimeout(200);
check(await page.evaluate(() => document.activeElement?.id === "main-content"), "navigation moves focus to main");

// Kit links go through the router: the breadcrumb's plain href does not reload the page.
await page.locator(".tr-grid a").first().click();
await page.locator(".tr-stay").waitFor();
await page.evaluate(() => (window.__noReload = true));
await page.getByRole("navigation", { name: "Breadcrumb" }).getByRole("link", { name: "Explore" }).click();
await page.locator(".tr-grid").waitFor();
check(await page.evaluate(() => window.__noReload === true), "the breadcrumb link navigates in the app (no reload)");
check(await page.evaluate(() => document.activeElement?.id === "main-content"), "and focus moves to main");
await page.goto(url);
await ready(page);
await page.keyboard.press("Tab");
const skip = page.getByRole("link", { name: "Skip to content" });
check(await skip.evaluate((el) => el === document.activeElement), "first Tab reaches the skip link");

await browser.close();
if (failed) {
  console.error(`${failed} booking check(s) failed`);
  process.exit(1);
}
console.log("booking: all checks passed");
