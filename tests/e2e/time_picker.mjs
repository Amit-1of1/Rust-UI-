// TimePicker, TimeField and DateTimeField in the gallery: typing, auto-advance, arrow keys,
// AM/PM, and the form wiring (invalid, error linked, focus on a failed submit).
// Every wait polls for the expected state; there are no fixed pauses (WebKit is slow to
// start the wasm and slow to re-render).
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node time_picker.mjs`.
import { launch, TIMEOUT } from "./engine.mjs";
import assert from "node:assert/strict";

const browser = await launch();
const page = await browser.newPage();
await page.goto(process.env.GALLERY_URL ?? "http://localhost:8765/");

async function until(check, message) {
  const end = Date.now() + TIMEOUT;
  let last;
  while (Date.now() < end) {
    try {
      if (await check()) return;
    } catch (e) {
      last = e;
    }
    await new Promise((r) => setTimeout(r, 50));
  }
  throw new Error(`timed out waiting for: ${message}${last ? ` (${last.message})` : ""}`);
}
const attr = (locator, name) => locator.getAttribute(name);
const activeLabel = () => page.evaluate(() => document.activeElement?.getAttribute("aria-label"));
const activeId = () => page.evaluate(() => document.activeElement?.id);
// Focus without Playwright's scroll-into-view click, then wait until it took.
async function focus(locator) {
  await locator.evaluate((el) => el.focus());
  const id = await locator.getAttribute("id");
  await until(async () => (await activeId()) === id, `focus on #${id}`);
}

// --- Meeting: 24-hour, 15-minute steps, starts at 09:30 ---------------------------------
const meeting = page.getByRole("group", { name: "Meeting" });
await meeting.waitFor();
const mHours = meeting.getByRole("spinbutton", { name: "Hours" });
const mMinutes = meeting.getByRole("spinbutton", { name: "Minutes" });
await mHours.scrollIntoViewIfNeeded();
assert.equal(await attr(mHours, "aria-valuenow"), "9");
assert.equal(await mMinutes.inputValue(), "30");

await focus(mMinutes);
await page.keyboard.press("ArrowUp");
await until(async () => (await mMinutes.inputValue()) === "45", "Up adds the 15-minute step");
await page.keyboard.press("ArrowUp");
await until(async () => (await mMinutes.inputValue()) === "00", "Up wraps past 59");
await until(() => page.getByText("Meeting: 09:00").isVisible(), "the value reaches the app");

await page.keyboard.press("ArrowLeft");
await until(async () => (await activeLabel()) === "Hours", "Left moves to the hours");
await page.keyboard.type("14");
await until(async () => (await attr(mHours, "aria-valuenow")) === "14", "typing 1 4 gives 14");
await until(async () => (await activeLabel()) === "Minutes", "two digits move on to the minutes");
await until(() => page.getByText("Meeting: 14:00").isVisible(), "14:00 reaches the app");

// --- Alarm: 12-hour with seconds ---------------------------------------------------------
const alarm = page.getByRole("group", { name: "Alarm" });
const period = alarm.getByRole("spinbutton", { name: "AM/PM" });
await focus(alarm.getByRole("spinbutton", { name: "Hours" }));
await page.keyboard.type("2");
await until(async () => (await activeLabel()) === "Minutes", "2 can't start a 12-hour hour, so it moves on");
await page.keyboard.type("0530");
await until(async () => (await activeLabel()) === "AM/PM", "minutes and seconds fill, then AM/PM");
await page.keyboard.press("p");
await until(async () => (await attr(period, "aria-valuetext")) === "PM", "P picks PM");
await page.keyboard.press("ArrowUp");
await until(async () => (await attr(period, "aria-valuetext")) === "AM", "Up toggles AM/PM");
await page.keyboard.press("Backspace");
await until(async () => (await attr(period, "aria-valuetext")) === "Empty", "Backspace clears");
await page.keyboard.press("Backspace");
await until(async () => (await activeLabel()) === "Seconds", "Backspace on an empty segment goes back");

// --- TimeField: required, focus and error on a failed submit ----------------------------
const reminderForm = page.locator("form", { has: page.getByRole("button", { name: "Set reminder" }) });
const rHours = reminderForm.getByRole("spinbutton", { name: /^Hours/ });
assert.match(await rHours.evaluate((el) => el.getAttribute("aria-labelledby")), /\S+ \S+/, "segments are also named by the field label");
assert.equal(await reminderForm.getByLabel("Reminder").count() >= 1, true, "the label reaches the first segment");
await reminderForm.getByRole("button", { name: "Set reminder" }).evaluate((el) => el.click());
await until(async () => (await attr(rHours, "aria-invalid")) === "true", "an empty required time is invalid");
await until(async () => (await activeId()) === (await attr(rHours, "id")), "a failed submit focuses the hours");
const errorId = (await attr(rHours, "aria-describedby")).split(" ").pop();
assert.match(await page.locator(`#${errorId}`).innerText(), /Enter a time/, "the error is linked");

await page.keyboard.type("915a");
await until(async () => (await attr(rHours, "aria-invalid")) === null, "a complete time clears the error");
await reminderForm.getByRole("button", { name: "Set reminder" }).evaluate((el) => el.click());
await until(() => page.getByText("Reminder set for 9:15 AM").isVisible(), "the form submits the time");

// --- DateTimeField: picking a date fills in 09:00 ----------------------------------------
const bookForm = page.locator("form", { has: page.getByRole("button", { name: "Book" }) });
const departure = bookForm.getByRole("group", { name: "Departure" });
assert.equal(await departure.count(), 1, "only the outer group carries the field's name");
const dHours = departure.getByRole("group", { name: "Time" }).getByRole("spinbutton", { name: "Hours" });
await bookForm.getByRole("button", { name: "Book" }).evaluate((el) => el.click());
await until(async () => (await attr(departure, "data-invalid")) === "true", "an empty date-time is invalid");
await until(async () => (await activeId()) === (await attr(dHours, "id")), "a failed submit focuses the time");
assert.match(await page.locator(`#${(await attr(departure, "aria-describedby")).split(" ").pop()}`).innerText(), /Pick a date and a time/);

for (const [name, text] of [["year", "2026"], ["month", "10"], ["day", "15"]]) {
  await departure.getByRole("spinbutton", { name }).evaluate((el) => el.focus());
  await page.keyboard.type(text);
}
await until(async () => (await dHours.inputValue()) === "09", "the default time fills in");
await until(async () => (await attr(departure, "data-invalid")) === null, "date and time clear the error");
await bookForm.getByRole("button", { name: "Book" }).evaluate((el) => el.click());
await until(() => page.getByText("Departure: 2026-10-15T09:00").isVisible(), "the form submits the date-time");

console.log("time picker: ok");
await browser.close();
