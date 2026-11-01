// Behaviour of the docs site: hash routes render, the nav marks the current page, the filter
// works, a demo is live, and navigation moves focus to the page heading.
// Usage: serve the built docs app, then `GALLERY_URL=http://localhost:8769/ node docs.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const base = process.env.GALLERY_URL ?? "http://localhost:8769/";
const browser = await launch();
const page = await browser.newPage({ viewport: { width: 1200, height: 900 } });
const errors = [];
page.on("pageerror", (e) => errors.push(String(e)));
await page.goto(base);
await page.waitForSelector(".docs-main");
await page.waitForTimeout(1000);

// Home page.
assert.equal(await page.getByRole("heading", { level: 1 }).innerText(), "Atoll");

// Clicking a nav link opens that page, marks it current and focuses its heading.
const nav = page.getByRole("navigation", { name: "Documentation" });
await nav.getByRole("link", { name: "Dialog", exact: true }).click();
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Dialog");
await page.waitForFunction(() => document.activeElement?.id === "page-title", null, { timeout: 3000 });
assert.equal(
  await nav.getByRole("link", { name: "Dialog", exact: true }).getAttribute("aria-current"),
  "page",
);

// The live demo works: it opens a modal dialog.
await page.getByRole("button", { name: /open dialog/i }).click();
await page.getByRole("dialog").waitFor();
await page.keyboard.press("Escape");
await page.getByRole("dialog").waitFor({ state: "detached" });

// The props table is generated from the source: `title` is a required prop.
await page.getByRole("heading", { name: "Properties", exact: true }).waitFor();
const titleRow = page.locator(".docs-prose table tr", { hasText: "The dialog's heading" }).first();
assert.match(await titleRow.innerText(), /required/);

// Guides link to each other on the site, and to GitHub for files that aren't guides.
await page.goto(base + "#composition");
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Composing components");
assert.equal(
  await page.locator(".docs-prose a", { hasText: "BEST_PRACTICES.md" }).first().getAttribute("href"),
  "#best-practices",
);
await page.goBack();

// The getting-started guide is a page of its own.
await page.goto(base + "#start");
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Getting started");
await page.getByText("atoll create my-app", { exact: false }).first().waitFor();
await page.goBack();

// So is the page for skeptics.
await page.goto(base + "#hard-questions");
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Hard questions");
await page.getByRole("heading", { name: "Am I locking my app to Dioxus?" }).waitFor();
await page.goBack();

// The theme builder: a typed brand colour recolours the preview, and the proof table says
// both modes pass AA. #4f46e5 is already readable with white, so light mode keeps it as is.
await page.goto(base + "#theme-builder");
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Theme builder");
await page.getByLabel("Hex").fill("#4f46e5");
const cta = page.getByRole("button", { name: "Start free trial" });
await page.waitForFunction(
  (el) => getComputedStyle(el).backgroundColor === "rgb(79, 70, 229)",
  await cta.elementHandle(),
  { timeout: 3000 },
);
assert.equal(await page.getByText("Passes AA").count(), 2);
// A pale brand gets a darker fill in light mode, still with the same hue.
await page.getByRole("button", { name: "Sunflower" }).click();
assert.equal(await page.getByRole("button", { name: "Sunflower" }).getAttribute("aria-pressed"), "true");
assert.equal(await page.getByLabel("Hex").inputValue(), "#facc15");
assert.ok((await page.getByText("lightness moved from #facc15").count()) >= 1);
assert.equal(await page.getByText("Passes AA").count(), 2);
// The CSS to copy is the preset `atoll theme --brand` prints.
assert.match(await page.locator("pre code").last().innerText(), /\[data-accent="brand"\]/);
await page.goBack();

// The example apps page offers every template.
await page.goto(base + "#examples");
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Example apps");
for (const t of ["dashboard", "booking", "marketing", "showcase"]) {
  await page.getByText(`atoll create my-app --git --template ${t}`, { exact: true }).waitFor();
}
assert.equal(await page.getByRole("link", { name: /^Open the live app/ }).count(), 4);
await page.goBack();

// A deep link and the browser's Back button.
await page.goto(base + "#theming");
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Theming");
await page.goBack();
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Dialog");

// Filter narrows the component list.
const filterBox = page.getByLabel("Filter components");
await filterBox.fill("date");
// The list re-renders after the input event; Firefox gets there a frame later than Chromium.
// On main (2026-11-01) WebKit once left the list unfiltered for 2 s after fill() right after a
// Back navigation; if it hasn't narrowed after a second, the text is typed again key by key.
// The failure message keeps the box's value, so a filter that really broke still shows.
const list = nav.getByRole("list").nth(1).getByRole("link");
let names = [];
for (let i = 0; i < 50; i++) {
  names = await list.allInnerTexts();
  if (names.length > 0 && names.every((n) => /date/i.test(n))) break;
  if (i === 10 || i === 30) {
    await filterBox.fill("");
    await filterBox.pressSequentially("date");
  }
  await page.waitForTimeout(100);
}
assert.ok(
  names.length > 0 && names.every((n) => /date/i.test(n)),
  `filter "${await filterBox.inputValue()}" gave: ${names}`,
);

// Code blocks that can scroll are keyboard reachable.
await page.goto(base + "#button");
await page.waitForSelector(".docs-prose pre");
assert.equal(await page.locator(".docs-prose pre").first().getAttribute("tabindex"), "0");

assert.deepEqual(errors, []);
console.log("docs: ok");
await browser.close();
