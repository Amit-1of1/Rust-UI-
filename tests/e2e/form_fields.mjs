// SelectField and ComboboxField (the gallery's form demo) join use_form like a TextField:
// labelled, marked invalid with the error linked, focused on a failed submit, cleared on a pick.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node form_fields.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const browser = await launch();
const page = await browser.newPage();
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8765/");
await page.waitForSelector(".gallery-main");
await page.waitForTimeout(1000);
const form = page.locator("form", { has: page.getByRole("button", { name: /^Role/ }) });

const role = form.getByRole("button", { name: /^Role/ });
const zone = form.getByRole("combobox", { name: "Time zone" });
assert.equal(await role.count(), 1, "the select trigger is named by its label");
assert.equal(await zone.count(), 1, "the combobox input is named by its label");
assert.ok((await zone.getAttribute("aria-describedby")).length > 0, "the combobox description is linked");

await form.getByLabel("Email").fill("ada@example.com");
await form.getByLabel("Password").fill("correct horse");
await form.getByRole("button", { name: "Create account" }).click();
await page.waitForTimeout(200);
assert.equal(await role.getAttribute("aria-invalid"), "true", "an empty required select is invalid");
assert.equal(
  await page.evaluate(() => document.activeElement?.id),
  await role.getAttribute("id"),
  "focus moves to the select's trigger",
);
const errorId = (await role.getAttribute("aria-describedby")).split(" ").pop();
assert.match(await page.locator(`#${errorId}`).innerText(), /Pick a role/, "the error is linked");

await role.click();
await page.getByRole("option", { name: "Designer" }).click();
await page.waitForTimeout(200);
assert.equal(await role.getAttribute("aria-invalid"), null, "picking an option clears the error");

await form.getByRole("button", { name: "Create account" }).click();
await page.getByText("Account created.").waitFor({ timeout: 5000 });
console.log("form fields: ok");
await browser.close();
