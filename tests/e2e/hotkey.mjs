// Ctrl+K opens the command palette from anywhere on the page (ui::hooks::use_hotkey).
// GALLERY_URL=... node hotkey.mjs
import { launch } from "./engine.mjs";

const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const browser = await launch();
const page = await browser.newPage();
await page.goto(url);
await page.waitForSelector(process.env.READY_SELECTOR ?? ".gallery-main > .gallery-section:last-child");

let failed = 0;
const check = (ok, msg) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${msg}`);
  if (!ok) failed++;
};

const palette = page.getByRole("combobox").and(page.locator("[placeholder='Search commands...']"));
await page.locator("body").click({ position: { x: 5, y: 5 } });
check(!(await palette.isVisible()), "palette starts closed");
await page.keyboard.press("Control+k");
await palette.waitFor({ state: "visible", timeout: 3000 }).catch(() => {});
check(await palette.isVisible(), "Ctrl+K opens the palette");
check(await palette.evaluate((el) => el === document.activeElement), "focus moves into the palette");
await page.keyboard.press("Escape");
await palette.waitFor({ state: "hidden", timeout: 3000 }).catch(() => {});
check(!(await palette.isVisible()), "Escape closes it");
await page.keyboard.press("Control+Shift+k");
await page.waitForTimeout(300);
check(!(await palette.isVisible()), "Ctrl+Shift+K is left alone");

await browser.close();
if (failed) {
  console.error(`${failed} hotkey check(s) failed`);
  process.exit(1);
}
