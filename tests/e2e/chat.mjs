// The chat set in the gallery: the log follows a streaming reply, lets go when the reader
// scrolls up and offers "New messages below", Enter sends, Shift+Enter adds a line, Escape
// stops, an IME Enter never sends, and screen readers get the whole reply once (never the
// fragments). Tool call approval changes status in words.
// Every wait polls for the expected state; there are no fixed pauses.
// Usage: serve the built gallery, then `GALLERY_URL=http://localhost:8765/ node chat.mjs`.
import { launch, ENGINE, TIMEOUT } from "./engine.mjs";
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

const log = page.getByRole("log", { name: "Conversation with the docs assistant" });
await log.waitFor();
const chat = page.locator("[data-slot=chat]", { has: log });
const box = page.getByRole("textbox", { name: "Message the docs assistant" });
const composer = page.locator("[data-slot=chat-composer]", { has: box });
const send = composer.getByRole("button", { name: "Send" });
const stop = composer.getByRole("button", { name: "Stop" });
const jump = chat.locator("[data-slot=chat-jump]");
await log.scrollIntoViewIfNeeded();

const gapToEnd = () => log.evaluate((el) => el.scrollHeight - el.scrollTop - el.clientHeight);
const atEnd = async () => (await gapToEnd()) <= 24;
const announced = () =>
  chat.locator("[data-slot=chat-announcer]").evaluateAll((els) => els.map((e) => e.textContent).join("|"));
const streaming = () => log.locator("[aria-busy=true]").count();
// Every announcement made while the test runs, so a fragment can't slip by between polls.
await page.evaluate(() => {
  window.__said = [];
  const watch = () =>
    document.querySelectorAll("[data-slot=chat-announcer]").forEach((el) => {
      if (el.__watched) return;
      el.__watched = true;
      new MutationObserver(() => el.textContent && window.__said.push(el.textContent)).observe(el, {
        childList: true,
        characterData: true,
        subtree: true,
      });
    });
  watch();
});
const said = () => page.evaluate(() => window.__said);

// --- Starts at the newest message ------------------------------------------------------
assert.equal(await log.getAttribute("aria-live"), "off", "the log is not itself live");
assert.equal(await log.getAttribute("tabindex"), "0", "the log can be scrolled by keyboard");
await until(atEnd, "the log opens at the newest message");
assert.equal(await jump.count(), 0, "no jump button at the end");
assert.equal(await send.getAttribute("aria-disabled"), "true", "Send is unavailable while empty");

// --- Enter sends; the reply streams and the log follows it --------------------------------
await box.evaluate((el) => el.focus());
await page.keyboard.press("Enter");
assert.equal(await log.locator("[data-from=user]").count(), 1, "Enter on an empty box sends nothing");
await page.keyboard.type("First");
await page.keyboard.press("Shift+Enter");
await page.keyboard.type("second line");
assert.equal(await box.inputValue(), "First\nsecond line", "Shift+Enter adds a line");
await page.keyboard.press("Enter");
await until(async () => (await box.inputValue()) === "", "sending empties the box");
assert.equal(await box.evaluate((el) => el === document.activeElement), true, "focus stays in the box");
await until(() => log.getByText("second line").first().isVisible(), "the message shows in the log");
await until(() => stop.isVisible(), "Send turns into Stop while the reply runs");
await until(async () => (await streaming()) > 0, "the reply starts streaming");
// While it streams, the log keeps the end in view.
for (let i = 0; i < 5 && (await streaming()) > 0; i++) {
  assert.ok(await atEnd(), `followed the stream (gap ${await gapToEnd()}px)`);
  await page.waitForTimeout(150);
}
await until(async () => (await streaming()) === 0, "the reply finishes");
await until(atEnd, "still at the end after the reply");
await until(() => send.isVisible(), "Stop turns back into Send");
await until(async () => (await announced()).includes("Assistant: You asked: “First\nsecond line”."), "the whole reply is announced");
const heard = await said();
assert.ok(
  heard.every((t) => !t.startsWith("Assistant: ") || t.endsWith("as it streams in.")),
  `only whole replies are announced: ${JSON.stringify(heard)}`,
);
assert.ok(heard.some((t) => t === "Assistant is typing"), `typing is announced: ${JSON.stringify(heard)}`);

// --- Scrolling up stops following; "New messages below" brings the reader back ----------
await box.fill("Another question");
await box.press("Enter");
await until(async () => (await streaming()) > 0, "the second reply starts streaming");
await log.evaluate((el) => el.scrollTo({ top: 0, behavior: "instant" }));
await until(() => jump.isVisible(), "scrolling up shows the jump button");
await until(async () => (await streaming()) === 0, "the second reply finishes");
assert.ok((await log.evaluate((el) => el.scrollTop)) < 40, "stayed where the reader scrolled to");
await until(async () => (await jump.textContent()).includes("New messages below"), "the button says there is more below");
await jump.evaluate((el) => el.click());
await until(atEnd, "the jump button goes to the newest message");
await until(async () => (await jump.count()) === 0, "the jump button goes away at the end");
assert.equal(await log.evaluate((el) => el === document.activeElement), true, "focus moves to the log");

// --- Escape stops a running reply --------------------------------------------------------
await box.fill("Stop me");
await box.press("Enter");
await until(async () => (await streaming()) > 0, "the third reply starts streaming");
await box.press("Escape");
await until(async () => (await streaming()) === 0, "Escape ends the stream");
const last = await log.locator("[data-slot=streaming-text]").last().textContent();
assert.ok(!last.endsWith("streams in."), `stopped early: ${last}`);

// --- Enter that confirms an IME composition never sends ----------------------------------
const before = await log.locator("[data-from=user]").count();
await box.fill("にほん");
await box.evaluate((el) =>
  el.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", isComposing: true, bubbles: true, cancelable: true })),
);
await page.waitForTimeout(200);
assert.equal(await log.locator("[data-from=user]").count(), before, "IME Enter did not send");
assert.equal(await box.inputValue(), "にほん", "IME Enter kept the text");
await box.fill("");

// --- The box grows with the text (field-sizing: content; Chromium has it) ----------------
if (ENGINE === "chromium") {
  const h1 = await box.evaluate((el) => el.getBoundingClientRect().height);
  await box.fill("one\ntwo\nthree\nfour");
  await until(async () => (await box.evaluate((el) => el.getBoundingClientRect().height)) > h1 + 20, "the box grows");
  await box.fill("");
}

// --- Tool call approval ----------------------------------------------------------------
const deploy = page.locator("[data-slot=tool-call]", { hasText: "deploy_preview" });
await deploy.scrollIntoViewIfNeeded();
assert.match(await deploy.getByRole("button", { expanded: false }).textContent(), /Needs approval/);
await deploy.getByRole("button", { name: "Allow" }).click();
await until(async () => (await deploy.getAttribute("data-status")) === "running", "Allow runs the call");
assert.match(await deploy.getByRole("button", { expanded: false }).textContent(), /Running/);
const plan = page.getByRole("list", { name: "Plan" });
assert.equal(await plan.locator("[aria-current=step]").count(), 2, "both running steps are current");

console.log(`chat: ok (${ENGINE})`);
await browser.close();
