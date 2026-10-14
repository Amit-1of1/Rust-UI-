// Which browser engine the e2e scripts drive: BROWSER=chromium (default), firefox or webkit.
// Every script launches through here so scripts/check.sh --browsers can run the same checks
// on each engine. Engines come from PLAYWRIGHT_BROWSERS_PATH; nothing is downloaded here.
import { chromium, firefox, webkit } from "playwright";

const ENGINES = { chromium, firefox, webkit };
export const ENGINE = process.env.BROWSER ?? "chromium";

export async function launch(options) {
  const type = ENGINES[ENGINE];
  if (!type) throw new Error(`BROWSER=${ENGINE}: expected chromium, firefox or webkit`);
  try {
    return await type.launch(options);
  } catch (e) {
    if (/Executable doesn't exist/.test(e.message)) {
      console.error(`${ENGINE} is not installed (PLAYWRIGHT_BROWSERS_PATH=${process.env.PLAYWRIGHT_BROWSERS_PATH ?? "default"}).`);
      console.error(`Install it with \`npx playwright install ${ENGINE}\` or run the e2e job in CI.`);
      process.exit(3);
    }
    throw e;
  }
}
