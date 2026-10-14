// light-dark() theme tokens: the OS scheme picks light or dark, data-theme on <html> forces
// one, and accent presets follow the same switch. Also checks the chips and link demos.
// GALLERY_URL=... node theme.mjs
import { launch } from "./engine.mjs";

const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const browser = await launch();

let failed = 0;
const check = (ok, msg) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${msg}`);
  if (!ok) failed++;
};

// Paints a probe with a token and reads the colour back.
async function token(page, name, attrs = {}) {
  return page.evaluate(
    ([name, attrs]) => {
      const root = document.documentElement;
      for (const [k, v] of Object.entries(attrs)) {
        if (v === null) root.removeAttribute(k);
        else root.setAttribute(k, v);
      }
      const probe = document.createElement("div");
      probe.style.backgroundColor = `var(${name})`;
      document.body.append(probe);
      const colour = getComputedStyle(probe).backgroundColor;
      probe.remove();
      return colour;
    },
    [name, attrs],
  );
}

const WHITE = "rgb(255, 255, 255)";
const BLACK = "rgb(0, 0, 0)";
for (const scheme of ["light", "dark"]) {
  const page = await browser.newPage({ colorScheme: scheme });
  await page.goto(url);
  await page.waitForSelector(".gallery-main > .gallery-section:last-child");
  const os = scheme === "dark" ? BLACK : WHITE;
  check((await token(page, "--primary-color", { "data-theme": null })) === os, `${scheme} OS: --primary-color follows it`);
  check((await token(page, "--primary-color", { "data-theme": "dark" })) === BLACK, `${scheme} OS + data-theme=dark: dark`);
  check((await token(page, "--primary-color", { "data-theme": "light" })) === WHITE, `${scheme} OS + data-theme=light: light`);
  const blueLight = await token(page, "--accent-color", { "data-theme": "light", "data-accent": "blue" });
  const blueDark = await token(page, "--accent-color", { "data-theme": "dark", "data-accent": "blue" });
  check(blueLight === "rgb(29, 78, 216)" && blueDark === "rgb(96, 165, 250)", `${scheme} OS: blue accent switches (${blueLight} / ${blueDark})`);
  const cs = await page.evaluate(() => getComputedStyle(document.documentElement).colorScheme);
  check(cs === "dark", `data-theme=dark sets color-scheme (${cs})`);
  await page.close();
}

// Chips: Space on the focused checkbox toggles the chip and its check mark.
const page = await browser.newPage();
await page.goto(url);
await page.waitForSelector(".gallery-main > .gallery-section:last-child");
const chip = page.getByRole("group", { name: "Filter by status" }).getByRole("checkbox", { name: "In review" });
const label = chip.locator("xpath=..");
check((await label.getAttribute("data-state")) === "off", "chip starts off");
await chip.focus();
await page.keyboard.press("Space");
check((await chip.isChecked()) && (await label.getAttribute("data-state")) === "on", "Space turns the chip on");
const ring = await label.evaluate((el) => getComputedStyle(el).boxShadow);
check(ring !== "none", `focused chip shows a ring (${ring})`);
await page.keyboard.press("Space");
check((await label.getAttribute("data-state")) === "off", "Space turns it off again");
const done = page.getByRole("checkbox", { name: "Done" });
await done.focus();
await page.keyboard.press("Space");
await page.getByText("Showing done tasks.").waitFor({ timeout: 2000 }).catch(() => {});
check(await page.getByText("Showing done tasks.").isVisible(), "controlled chip reports its change");

// Link: external links open in a new tab and say so.
const ext = page.getByRole("link", { name: "Lucide icons (opens in a new tab)" });
check((await ext.count()) === 1, "external link is named with the new-tab note");
check((await ext.getAttribute("rel")) === "noopener noreferrer", "external link has rel=noopener noreferrer");

await browser.close();
if (failed) {
  console.error(`${failed} theme check(s) failed`);
  process.exit(1);
}
