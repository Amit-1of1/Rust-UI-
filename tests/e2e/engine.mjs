// Which browser engine the e2e scripts drive: BROWSER=chromium (default), firefox or webkit.
// Every script launches through here so scripts/check.sh --browsers can run the same checks
// on each engine. Engines come from PLAYWRIGHT_BROWSERS_PATH; nothing is downloaded here.
import { chromium, firefox, webkit } from "playwright";

const ENGINES = { chromium, firefox, webkit };
export const ENGINE = process.env.BROWSER ?? "chromium";

// WebKit on a CI runner can take well over 30 s to start the gallery's wasm; every wait on a
// page gets this long in WebKit.
export const TIMEOUT = ENGINE === "webkit" ? 90000 : 30000;

export async function launch(options) {
  const type = ENGINES[ENGINE];
  if (!type) throw new Error(`BROWSER=${ENGINE}: expected chromium, firefox or webkit`);
  try {
    const browser = await type.launch(options);
    const newPage = browser.newPage.bind(browser);
    browser.newPage = async (opts) => {
      const page = await newPage(opts);
      page.setDefaultTimeout(TIMEOUT);
      // A wasm panic or engine crash otherwise shows up only as a timeout.
      page.on("pageerror", (e) => console.log(`page error (${ENGINE}): ${e.message.split("\n")[0]}`));
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
