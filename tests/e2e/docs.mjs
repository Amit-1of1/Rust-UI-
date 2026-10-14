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
assert.equal(await page.getByRole("heading", { level: 1 }).innerText(), "Rust UI");

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

// A deep link and the browser's Back button.
await page.goto(base + "#theming");
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Theming");
await page.goBack();
await page.waitForFunction(() => document.getElementById("page-title")?.textContent === "Dialog");

// Filter narrows the component list.
await page.getByLabel("Filter components").fill("date");
const names = await nav.getByRole("list").nth(1).getByRole("link").allInnerTexts();
assert.ok(names.length > 0 && names.every((n) => /date/i.test(n)), `filter result: ${names}`);

// Code blocks that can scroll are keyboard reachable.
await page.goto(base + "#button");
await page.waitForSelector(".docs-prose pre");
assert.equal(await page.locator(".docs-prose pre").first().getAttribute("tabindex"), "0");

assert.deepEqual(errors, []);
console.log("docs: ok");
await browser.close();
