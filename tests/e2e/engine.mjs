// Which browser engine the e2e scripts drive: BROWSER=chromium (default), firefox or webkit.
// Every script launches through here so scripts/check.sh --browsers can run the same checks
// on each engine. Engines come from PLAYWRIGHT_BROWSERS_PATH; nothing is downloaded here.
import { existsSync } from "node:fs";
import { chromium, firefox, webkit } from "playwright";

const ENGINES = { chromium, firefox, webkit };
export const ENGINE = process.env.BROWSER ?? "chromium";

// WebKit on a CI runner can take well over 30 s to start the gallery's wasm; every wait on a
// page gets this long in WebKit.
export const TIMEOUT = ENGINE === "webkit" ? 90000 : 30000;

export async function launch(options) {
  const type = ENGINES[ENGINE];
  if (!type) throw new Error(`BROWSER=${ENGINE}: expected chromium, firefox or webkit`);
  // A Chromium Playwright did not download itself (cloud containers ship one older build at
  // /opt/pw-browsers/chromium): use it when the one this Playwright expects is missing.
  if (ENGINE === "chromium" && !existsSync(type.executablePath())) {
    const fallback = process.env.CHROMIUM_PATH ?? "/opt/pw-browsers/chromium";
    if (existsSync(fallback)) {
      console.log(`chromium: using ${fallback} (${type.executablePath()} is not installed)`);
      options = { ...options, executablePath: fallback };
    }
  }
  try {
    const browser = await type.launch(options);
    const newPage = browser.newPage.bind(browser);
    browser.newPage = async (opts) => {
      const page = await newPage(opts);
      page.setDefaultTimeout(TIMEOUT);
      // A wasm panic or engine crash otherwise shows up only as a timeout.
      page.on("pageerror", (e) => {
        console.log(`page error (${ENGINE}): ${e.message.split("\n")[0]}`);
        if (e.stack) console.log(`  ${e.stack.split("\n").slice(0, 5).join("\n  ")}`);
      });
      return page;
    };
    return browser;
  } catch (e) {
    if (/Executable doesn't exist/.test(e.message)) {
      console.error(`${ENGINE} is not installed (PLAYWRIGHT_BROWSERS_PATH=${process.env.PLAYWRIGHT_BROWSERS_PATH ?? "default"}).`);
      console.error(`Install it with \`npx playwright install ${ENGINE}\` or run the e2e job in CI.`);
      process.exit(3);
    }
    throw e;
  }
}
