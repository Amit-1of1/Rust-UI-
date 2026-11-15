// Behaviour of DataTable in the gallery: filter, sort, paginate, select.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node table.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const browser = await launch();
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

// Data table v2. Export: 5 rows are selected above, so the file has those 5 and a header.
const [download] = await Promise.all([
  page.waitForEvent("download"),
  root.getByRole("button", { name: "Export CSV" }).click(),
]);
assert.equal(download.suggestedFilename(), "team-directory.csv");
const csv = (await import("node:fs")).readFileSync(await download.path(), "utf8");
const lines = csv.replace(/^﻿/, "").trimEnd().split("\r\n");
assert.equal(lines[0], "Name,Team,Office,Salary");
assert.equal(lines.length, 6, csv);
assert.equal(lines[1], 'Donald Knuth,Research,Stanford,"$170,000"', "in the table's sort order, commas quoted");

// Column menu: hide Office, then show it again.
const heads = () => table.locator("thead th").count();
const before = await heads();
await root.getByRole("button", { name: "Columns" }).click();
const office = page.getByRole("checkbox", { name: "Office" });
await office.uncheck();
await page.waitForTimeout(100);
assert.equal(await heads(), before - 1, "Office is hidden");
assert.equal(await table.getByRole("columnheader", { name: "Office" }).count(), 0);
assert.equal(await page.getByRole("checkbox", { name: "Name" }).count(), 0, "Name can't be hidden");
await office.check();
await page.waitForTimeout(100);
assert.equal(await heads(), before, "Office is back");
await page.keyboard.press("Escape");

// Resize from the keyboard: two steps of 16 px.
const nameHead = table.getByRole("columnheader", { name: /^Name/ });
const width = async (head) => (await head.boundingBox()).width;
const nameBefore = await width(nameHead);
const nameHandle = table.getByRole("separator", { name: "Resize Name" });
await nameHandle.focus();
await page.keyboard.press("ArrowRight");
await page.waitForTimeout(100);
await page.keyboard.press("ArrowRight");
await page.waitForTimeout(150);
const nameAfter = await width(nameHead);
assert.ok(Math.abs(nameAfter - nameBefore - 32) <= 2, `Name ${nameBefore} -> ${nameAfter}`);
assert.equal(await nameHandle.getAttribute("aria-valuenow"), String(Math.round(nameAfter)));
await page.keyboard.press("Home");
await page.waitForTimeout(150);
assert.ok(Math.abs((await width(nameHead)) - 48) <= 2, "Home goes to the narrowest");

// Resize with the pointer: drag Team's edge 60 px.
const teamHead = table.getByRole("columnheader", { name: /^Team/ });
const teamBefore = await width(teamHead);
const handle = await table.getByRole("separator", { name: "Resize Team" }).boundingBox();
const y = handle.y + handle.height / 2;
await page.mouse.move(handle.x + handle.width / 2, y);
await page.mouse.down();
await page.waitForTimeout(100);
await page.mouse.move(handle.x + 30, y, { steps: 3 });
await page.mouse.move(handle.x + 60, y, { steps: 3 });
await page.mouse.up();
await page.waitForTimeout(150);
const teamAfter = await width(teamHead);
assert.ok(Math.abs(teamAfter - teamBefore - 60) <= 4, `Team ${teamBefore} -> ${teamAfter}`);

// Virtualised: 10,000 rows, a few dozen in the DOM, and scrolling brings the end in.
const orders = page.locator("table", { has: page.locator("caption", { hasText: "10,000 orders" }) });
const viewport = page.getByRole("region", { name: "10,000 orders" });
await viewport.scrollIntoViewIfNeeded();
assert.equal(await orders.getAttribute("aria-rowcount"), "10001");
const rendered = () => orders.locator("tbody tr[aria-rowindex]").count();
assert.ok((await rendered()) < 40, `${await rendered()} rows in the DOM`);
const timeTo = async (top, text) => {
  const start = Date.now();
  await viewport.evaluate((el, top) => { el.scrollTop = top; }, top);
  await orders.locator("td", { hasText: text }).first().waitFor({ state: "visible", timeout: 5000 });
  return Date.now() - start;
};
const middle = await timeTo(5000 * 40, "#05001");
const end = await timeTo(10_000 * 40, "#10000");
assert.ok((await rendered()) < 40, "still a few dozen rows after scrolling");
const row = await orders.locator("tbody tr[aria-rowindex]").last().getAttribute("aria-rowindex");
assert.equal(row, "10001", "the last row says it is row 10,001 (the header is row 1)");
console.log(`table: virtual scroll to the middle ${middle} ms, to the end ${end} ms`);
console.log("table: ok");
await browser.close();
