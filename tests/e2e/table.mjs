// Behaviour of DataTable in the gallery: filter, sort, paginate, select.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node table.mjs`.
import { chromium } from "playwright";
import assert from "node:assert/strict";

const browser = await chromium.launch();
const page = await browser.newPage();
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8765/");
await page.waitForSelector(".gallery-main > .gallery-section:last-child");
await page.waitForTimeout(1200);
const table = page.locator("table", { has: page.locator("caption", { hasText: "Team directory" }) });
const root = page.locator("div").filter({ has: table }).filter({ has: page.getByLabel("Filter rows") }).last();
await table.scrollIntoViewIfNeeded();
const status = root.getByRole("status");
const firstName = () => table.locator("tbody tr").first().locator("td").nth(1).innerText();

assert.match(await status.innerText(), /Showing 1–5 of 14 rows/);

await root.getByLabel("Filter rows").fill("research");
await page.waitForTimeout(150);
assert.match(await status.innerText(), /Showing 1–5 of 5 rows|of 5 rows/);
await root.getByLabel("Filter rows").fill("");

const salary = table.getByRole("columnheader", { name: /Salary/ });
await salary.getByRole("button").click();
await page.waitForTimeout(100);
assert.equal(await salary.getAttribute("aria-sort"), "ascending");
assert.equal(await firstName(), "Hedy Lamarr", "lowest salary first");
await salary.getByRole("button").click();
assert.equal(await salary.getAttribute("aria-sort"), "descending");
assert.equal(await firstName(), "Donald Knuth", "highest salary first");

await root.getByRole("button", { name: "Next" }).click();
await page.waitForTimeout(100);
assert.match(await status.innerText(), /Showing 6–10 of 14 rows/);
await root.getByRole("button", { name: "Previous" }).click();

await table.locator("tbody tr").first().getByRole("checkbox").check();
await page.waitForTimeout(100);
assert.match(await status.innerText(), /1 selected/);
await table.getByLabel("Select all rows on this page").check();
await page.waitForTimeout(100);
assert.match(await status.innerText(), /5 selected/);

// Sortable headers line up with their cells: start edges for text, end edges for numbers.
const edges = await table.evaluate((t) => {
  const textBox = (node) => {
    const r = document.createRange();
    r.selectNodeContents(node);
    return r.getBoundingClientRect();
  };
  const heads = [...t.querySelectorAll("thead th")];
  const cells = [...t.querySelectorAll("tbody tr:first-child td")];
  const at = (name) => heads.findIndex((th) => th.innerText.startsWith(name));
  const label = (i) => textBox(heads[i].querySelector("button").firstChild);
  const name = at("Name");
  const salary = at("Salary");
  return {
    nameHead: label(name).left,
    nameCell: textBox(cells[name]).left,
    salaryHead: label(salary).right,
    salaryCell: textBox(cells[salary]).right,
  };
});
assert.ok(Math.abs(edges.nameHead - edges.nameCell) <= 1, `Name header starts with its cells ${JSON.stringify(edges)}`);
assert.ok(Math.abs(edges.salaryHead - edges.salaryCell) <= 1, `Salary header ends with its cells ${JSON.stringify(edges)}`);
console.log("table: ok");
await browser.close();
