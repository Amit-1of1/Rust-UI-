// Behaviour of the `use_form` hook through the Orbit signup form (apps/site).
// Usage: serve the built site, then `GALLERY_URL=http://localhost:8767/ node form.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const browser = await launch();
const page = await browser.newPage();
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8767/");
await page.waitForSelector("#signup-form");
await page.waitForTimeout(1000);
const form = page.locator("#signup-form");
const name = form.getByLabel("Name");
const email = form.getByLabel("Work email");

await form.getByRole("button", { name: "Join waitlist" }).click();
await page.waitForTimeout(200);
assert.equal(await form.getByRole("alert").count(), 2, "both fields report an error");
assert.equal(await name.getAttribute("aria-invalid"), "true");
assert.ok((await name.getAttribute("aria-describedby")).length > 0, "error is linked");
assert.equal(await page.evaluate(() => document.activeElement?.id), await name.getAttribute("id"), "focus moves to the first invalid field");

await name.fill("Ada");
await page.waitForTimeout(100);
assert.equal(await name.getAttribute("aria-invalid"), null, "error clears as soon as the value is valid");

await email.fill("nope");
await email.blur();
await page.waitForTimeout(100);
assert.match(await form.getByRole("alert").innerText(), /valid email/i);
const described = await email.getAttribute("aria-describedby");
assert.equal(described.split(" ").length, 2, "description and error are both linked");

await email.fill("ada@example.com");
await page.waitForTimeout(100);
assert.equal(await form.getByRole("alert").count(), 0);
await form.getByRole("button", { name: "Join waitlist" }).click();
await page.waitForSelector("#welcome-close", { timeout: 5000 });
assert.equal(await name.inputValue(), "", "form resets after a valid submit");
console.log("form: ok");
await browser.close();
