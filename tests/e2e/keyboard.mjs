// Keyboard behaviour per WAI-ARIA APG pattern, driven only with the keyboard against the
// built gallery. One block per pattern; each checks the keys the APG lists for it, the
// ARIA state those keys change, and where focus ends up. Modelled on React Aria's
// test-utils "testers" (Apache-2.0), written from scratch.
// GALLERY_URL=... node keyboard.mjs
import { chromium } from "playwright";

const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1100, height: 900 } });
await page.goto(url);
await page.waitForSelector(".gallery-main > .gallery-section:last-child");

let failed = 0;
const check = (ok, msg) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${msg}`);
  if (!ok) failed++;
};
const section = (title) =>
  page.locator("section.gallery-section").filter({ has: page.locator(`h2:text-is("${title}")`) });
const focused = () =>
  page.evaluate(() => {
    const el = document.activeElement;
    return { text: el?.textContent?.trim() ?? "", role: el?.getAttribute("role") ?? el?.tagName.toLowerCase() };
  });
const attr = (loc, name) => loc.getAttribute(name);
const press = async (key) => {
  await page.keyboard.press(key);
  await page.waitForTimeout(120);
};
async function pattern(name, body) {
  try {
    await body();
  } catch (e) {
    check(false, `${name}: ${e.message.split("\n")[0]}`);
  }
  await page.keyboard.press("Escape");
  await page.evaluate(() => document.activeElement?.blur());
  await page.waitForTimeout(200);
}

// Tabs: arrows move focus between tabs, Enter or Space activates (manual activation).
await pattern("tabs", async () => {
  const tabs = section("Tabs").getByRole("tab");
  await tabs.first().focus();
  await press("ArrowRight");
  check((await focused()).text === (await tabs.nth(1).innerText()).trim(), "tabs: ArrowRight moves focus to the next tab");
  await press("Enter");
  check((await attr(tabs.nth(1), "aria-selected")) === "true", "tabs: Enter selects the focused tab");
  await press("ArrowLeft");
  await press("Space");
  check((await attr(tabs.first(), "aria-selected")) === "true", "tabs: Space selects after ArrowLeft");
});

// Accordion: Enter or Space toggles the header's panel and aria-expanded.
await pattern("accordion", async () => {
  const header = section("Accordion").locator("button[aria-expanded]").first();
  const before = await attr(header, "aria-expanded");
  await header.focus();
  await press("Enter");
  check((await attr(header, "aria-expanded")) !== before, "accordion: Enter toggles aria-expanded");
  await press("Space");
  check((await attr(header, "aria-expanded")) === before, "accordion: Space toggles it back");
});

// Checkbox and switch: Space toggles.
await pattern("checkbox", async () => {
  const box = section("Checkbox").getByRole("checkbox").first();
  const before = await attr(box, "aria-checked") ?? String(await box.isChecked());
  await box.focus();
  await press("Space");
  const after = await attr(box, "aria-checked") ?? String(await box.isChecked());
  check(after !== before, "checkbox: Space toggles");
});
await pattern("switch", async () => {
  const sw = section("Switch").getByRole("switch").first();
  const before = await attr(sw, "aria-checked");
  await sw.focus();
  await press("Space");
  check((await attr(sw, "aria-checked")) !== before, "switch: Space toggles aria-checked");
});

// Radio group: arrows move focus and check, skipping disabled radios.
await pattern("radio group", async () => {
  const radios = section("Radio group").getByRole("radio");
  await radios.first().focus();
  await press("Space");
  check((await attr(radios.first(), "aria-checked")) === "true", "radio group: Space checks the focused radio");
  await press("ArrowDown");
  check((await focused()).text === (await radios.nth(1).innerText()).trim(), "radio group: ArrowDown moves focus");
  await press("ArrowDown");
  const f = await focused();
  check(f.text !== "Green", "radio group: arrows skip the disabled radio");
});

// Slider: arrows step, Home and End jump to the ends.
await pattern("slider", async () => {
  const thumb = section("Slider").getByRole("slider").first();
  await thumb.focus();
  const start = Number(await attr(thumb, "aria-valuenow"));
  await press("ArrowRight");
  check(Number(await attr(thumb, "aria-valuenow")) > start, "slider: ArrowRight increases the value");
  await press("Home");
  check(Number(await attr(thumb, "aria-valuenow")) === Number(await attr(thumb, "aria-valuemin")), "slider: Home goes to the minimum");
  await press("End");
  check(Number(await attr(thumb, "aria-valuenow")) === Number(await attr(thumb, "aria-valuemax")), "slider: End goes to the maximum");
});

// Dialog: opens from the keyboard, traps focus, Escape closes and returns focus.
await pattern("dialog", async () => {
  const trigger = section("Dialog").getByRole("button").first();
  await trigger.focus();
  await press("Enter");
  await page.waitForTimeout(300);
  const dialog = page.getByRole("dialog");
  check(await dialog.isVisible(), "dialog: Enter on the trigger opens it");
  check(await dialog.evaluate((d) => d.contains(document.activeElement)), "dialog: focus moves inside");
  for (let i = 0; i < 6; i++) await press("Tab");
  check(await dialog.evaluate((d) => d.contains(document.activeElement)), "dialog: Tab keeps focus inside");
  await press("Escape");
  await page.waitForTimeout(300);
  check(!(await dialog.isVisible()), "dialog: Escape closes it");
  check(await trigger.evaluate((t) => t === document.activeElement), "dialog: focus returns to the trigger");
});

// Menu button: Enter opens, arrows move between items, Escape closes and returns focus.
await pattern("dropdown menu", async () => {
  const trigger = section("Dropdown menu").getByRole("button", { name: "Actions" });
  await trigger.focus();
  await press("Enter");
  await page.waitForTimeout(250);
  const first = await focused();
  await press("ArrowDown");
  const second = await focused();
  check(first.text !== "" && second.text !== first.text, "dropdown menu: ArrowDown moves between items");
  await press("Escape");
  await page.waitForTimeout(250);
  check(await trigger.evaluate((t) => t === document.activeElement), "dropdown menu: Escape returns focus to the trigger");
});

// Select (listbox popup): Enter opens, arrows move, Enter picks and closes.
await pattern("select", async () => {
  const trigger = section("Select").locator("button[aria-haspopup=listbox]").first();
  await trigger.focus();
  await press("Enter");
  await page.waitForTimeout(250);
  check((await attr(trigger, "aria-expanded")) === "true", "select: Enter opens the list");
  await press("ArrowDown");
  const option = (await focused()).text;
  await press("Enter");
  await page.waitForTimeout(250);
  check((await attr(trigger, "aria-expanded")) === "false", "select: Enter picks and closes");
  // Options can carry an icon; the trigger shows the option's text value.
  const shown = (await trigger.innerText()).trim();
  check(shown !== "" && option.includes(shown), `select: the trigger shows the picked option (${shown})`);
});

// Toggle button: Space and Enter flip aria-pressed.
await pattern("toggle", async () => {
  const toggle = section("Toggle").locator("[aria-pressed]").first();
  const before = await attr(toggle, "aria-pressed");
  await toggle.focus();
  await press("Space");
  check((await attr(toggle, "aria-pressed")) !== before, "toggle: Space flips aria-pressed");
  await press("Enter");
  check((await attr(toggle, "aria-pressed")) === before, "toggle: Enter flips it back");
});

// Textarea autosize: grows with its content where field-sizing is supported.
await pattern("textarea autosize", async () => {
  const area = page.locator("#autosize");
  const h0 = (await area.boundingBox()).height;
  await area.focus();
  await page.keyboard.type("one\ntwo\nthree\nfour\nfive\nsix");
  const h1 = (await area.boundingBox()).height;
  check(h1 > h0, `textarea: autosize grows with the text (${Math.round(h0)}px -> ${Math.round(h1)}px)`);
});

await browser.close();
if (failed) {
  console.error(`${failed} keyboard check(s) failed`);
  process.exit(1);
}
