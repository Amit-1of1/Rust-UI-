// Keyboard-only walkthrough of the whole gallery: Tab from the top of the page to the end, then
// Shift+Tab back a stretch, and check what a keyboard or screen-reader user would run into.
//   - every Tab stop is visible (not zero-size, hidden, aria-hidden or inert),
//   - every Tab stop shows a focus indicator (something changes when it takes focus),
//   - no positive tabindex reorders the page,
//   - focus is never trapped outside a modal: the walk reaches the end of the page,
//   - Shift+Tab retraces the same stops in reverse.
// GALLERY_URL=... node walkthrough.mjs
import { ENGINE, launch } from "./engine.mjs";

const url = process.env.GALLERY_URL ?? "http://localhost:8765/";
const MAX = Number(process.env.MAX_TABS ?? 1500);
const browser = await launch();
const page = await browser.newPage({ viewport: { width: 1100, height: 900 } });
await page.goto(url);
await page.waitForSelector(process.env.READY_SELECTOR ?? ".gallery-main");
await page.waitForTimeout(1000);

let failed = 0;
const check = (ok, msg) => {
  console.log(`${ok ? "ok  " : "FAIL"} ${msg}`);
  if (!ok) failed++;
};

const positive = await page.evaluate(() =>
  [...document.querySelectorAll("[tabindex]")]
    .filter((el) => el.tabIndex > 0)
    .map((el) => el.outerHTML.slice(0, 80)),
);
check(positive.length === 0, `no positive tabindex${positive.length ? ": " + positive.join(" | ") : ""}`);

// The unfocused look of everything focusable, so a Tab stop's indicator is something that
// changes on focus (outline, ring, fill or border on the element, its ::before/::after or its
// wrapper), not a shadow it always has. Forced colors get an outline from theme.css instead.
await page.evaluate(() => {
  const ring = (el, p) => {
    const s = getComputedStyle(el, p);
    return `${s.outlineStyle} ${s.outlineWidth} ${s.outlineColor} | ${s.boxShadow} | ${s.backgroundColor} | ${s.borderColor}`;
  };
  window.__ring = (el) =>
    [ring(el, null), ring(el, "::before"), ring(el, "::after"), el.parentElement ? ring(el.parentElement, null) : ""];
  window.__base = new Map();
  for (const el of document.querySelectorAll("a[href], button, input, select, textarea, [tabindex], [contenteditable]"))
    window.__base.set(el, window.__ring(el));
});

// Describes the focused element and the problems a keyboard user would hit on it.
const stop = () =>
  page.evaluate(() => {
    const el = document.activeElement;
    if (!el || el === document.body || el === document.documentElement) return null;
    if (!el.dataset.walk) el.dataset.walk = String(document.querySelectorAll("[data-walk]").length);
    const shows = (s) =>
      (s.outlineStyle !== "none" && parseFloat(s.outlineWidth) > 0) || (s.boxShadow && s.boxShadow !== "none");
    // A control may draw its ring on a pseudo-element (radio dot) or on its wrapper via
    // :focus-within / :has(:focus-visible) (calendar's transparent native selects).
    const now = window.__ring(el);
    const base = window.__base.get(el);
    const changed = (i) => !base || base[i] !== now[i];
    const self = [0, 1, 2].some(changed);
    const parent = el.parentElement;
    const wrap = parent && changed(3) && shows(getComputedStyle(parent));
    const boxOf = wrap && getComputedStyle(el).opacity === "0" ? parent : el;
    const r = boxOf.getBoundingClientRect();
    const s = getComputedStyle(boxOf);
    const problems = [];
    if (r.width < 1 || r.height < 1) problems.push("zero-size");
    if (s.visibility === "hidden" || s.display === "none" || Number(s.opacity) === 0)
      problems.push(
        `invisible (opacity ${s.opacity}, :focus-visible ${el.matches(":focus-visible")}, ` +
          `running animations ${boxOf.getAnimations?.().length ?? "?"})`,
      );
    if (el.closest("[aria-hidden=true]")) problems.push("inside aria-hidden");
    if (el.closest("[inert]")) problems.push("inside inert");
    if (!self && !wrap) problems.push("no focus indicator");
    const role = el.getAttribute("role") ?? el.tagName.toLowerCase();
    const label = (el.getAttribute("aria-label") ?? el.textContent ?? el.getAttribute("placeholder") ?? "")
      .trim()
      .replace(/\s+/g, " ")
      .slice(0, 40);
    const section = el.closest("section.gallery-section")?.querySelector("h2")?.textContent ?? "page";
    const modal = !!el.closest("[aria-modal=true], dialog[open]");
    return { id: el.dataset.walk, what: `${section} > ${role} "${label}"`, problems, modal };
  });

const stops = [];
const seen = new Set();
let reachedEnd = false;
for (let i = 0; i < MAX; i++) {
  await page.keyboard.press("Tab");
  const s = await stop();
  if (!s) {
    reachedEnd = true; // focus left the document (past the last stop)
    break;
  }
  if (seen.has(s.id)) {
    // Wrapped round to the first stop, or (Firefox headless) Tab on the last stop left focus there.
    reachedEnd = s.id === stops[0]?.id || s.id === stops[stops.length - 1]?.id;
    if (!reachedEnd) check(false, `focus loops back to an earlier stop at ${s.what} (trap)`);
    break;
  }
  if (s.modal) {
    check(false, `Tab walked into an open modal at ${s.what}; it should be closed on load`);
    break;
  }
  if (s.problems.length) {
    // Revealed-on-focus controls fade in; look again once the transition is done. Headless
    // WebKit can leave a transition unstarted for a stop it scrolled to, so after the wait the
    // focused element's own transitions are run to their end state, which is what a user sees.
    await page.waitForTimeout(350);
    await page.evaluate(() => {
      const el = document.activeElement;
      for (const a of [el, el?.parentElement].flatMap((n) => n?.getAnimations?.() ?? [])) {
        try {
          a.finish(); // throws for an infinite animation (a spinner), which is left alone
        } catch {}
      }
    });
    s.problems = (await stop()).problems;
  }
  seen.add(s.id);
  stops.push(s);
}
check(reachedEnd, `Tab reaches the end of the page (${stops.length} stops)`);

const bad = stops.filter((s) => s.problems.length);
for (const s of bad.slice(0, 25)) console.log(`     ${s.what}: ${s.problems.join(", ")}`);
check(bad.length === 0, `every Tab stop is visible and shows focus (${bad.length} of ${stops.length} do not)`);

// Shift+Tab from the end retraces the last stops in reverse.
const back = Math.min(30, stops.length - 1);
if (reachedEnd && back > 0) {
  const last = stops[stops.length - 1];
  await page.evaluate((id) => document.querySelector(`[data-walk="${id}"]`)?.focus(), last.id);
  let same = 0;
  for (let i = stops.length - 2; i >= stops.length - 1 - back; i--) {
    await page.keyboard.press("Shift+Tab");
    const s = await stop();
    if (s?.id === stops[i].id) same++;
    else {
      console.log(`     Shift+Tab expected ${stops[i].what}, got ${s?.what ?? "nothing"}`);
      break;
    }
  }
  check(same === back, `Shift+Tab retraces the last ${back} stops in reverse`);
}

console.log(`${ENGINE}: ${stops.length} Tab stops walked`);
await browser.close();
if (failed) {
  console.error(`${failed} walkthrough check(s) failed`);
  process.exit(1);
}
