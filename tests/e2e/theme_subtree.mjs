// data-theme on any element switches its own subtree, in light-dark() browsers too: a dark box
// inside a light page gets dark tokens, keeps the page's accent (resolved for dark), and an
// element's own data-accent still wins. Plain values set on <html> (--radius) still reach it.
// Usage: serve any built app that loads theme.css, then
// `GALLERY_URL=http://localhost:8769/ node theme_subtree.mjs`.
import { launch } from "./engine.mjs";
import assert from "node:assert/strict";

const base = process.env.GALLERY_URL ?? "http://localhost:8769/";
const browser = await launch();
const page = await browser.newPage();
await page.goto(base);
await page.waitForFunction(() =>
  getComputedStyle(document.documentElement).getPropertyValue("--primary-color").trim() !== "",
);

const tokens = await page.evaluate(() => {
  const html = document.documentElement;
  html.setAttribute("data-theme", "light");
  html.setAttribute("data-accent", "blue");
  html.style.setProperty("--radius", "3px");
  const box = (attrs, parent = document.body) => {
    const el = document.createElement("div");
    for (const [k, v] of Object.entries(attrs)) el.setAttribute(k, v);
    parent.append(el);
    return el;
  };
  const dark = box({ "data-theme": "dark" });
  const inner = box({}, dark);
  const rose = box({ "data-theme": "dark", "data-accent": "rose" });
  const light = box({ "data-theme": "light" }, dark);
  // A look (theme preset) on a box switches its subtree too, a dark island inside it as well.
  const reef = box({ "data-preset": "reef" });
  const reefDark = box({ "data-theme": "dark" }, reef);
  const backToLagoon = box({ "data-preset": "lagoon" }, reef);
  const read = (el, name) => getComputedStyle(el).getPropertyValue(name).trim();
  const bg = (el) => {
    el.style.background = "var(--primary-color)";
    return getComputedStyle(el).backgroundColor;
  };
  return {
    page: read(html, "--primary-color"),
    dark: read(dark, "--primary-color"),
    inner: read(inner, "--primary-color"),
    darkBg: bg(dark),
    accent: read(dark, "--accent-color"),
    pageAccent: read(html, "--accent-color"),
    rose: read(rose, "--accent-color"),
    backToLight: read(light, "--primary-color"),
    radius: read(dark, "--radius"),
    reefPage: read(reef, "--primary-color"),
    reefDark: read(reefDark, "--primary-color"),
    reefFont: read(reefDark, "--font-sans"),
    lagoonAgain: read(backToLagoon, "--primary-color"),
  };
});

// Lagoon, the default look.
assert.equal(tokens.page, "#faffff", JSON.stringify(tokens));
assert.equal(tokens.dark, "#041111", JSON.stringify(tokens));
assert.equal(tokens.inner, "#041111", "children inherit the dark value");
assert.equal(tokens.darkBg, "rgb(4, 17, 17)");
assert.equal(tokens.pageAccent, "#1d4ed8");
assert.equal(tokens.accent, "#60a5fa", "the page's blue accent, in its dark shade");
assert.equal(tokens.rose, "#fb7185", "an element's own accent wins");
assert.equal(tokens.backToLight, "#faffff", "a light box inside a dark one is light again");
assert.equal(tokens.radius, "3px", "plain values set on <html> still reach the subtree");
assert.equal(tokens.reefPage, "#fffdfd", "data-preset on a box switches its look");
assert.equal(tokens.reefDark, "#150b0a", "a dark island inside a look gets that look's dark tokens");
assert.match(tokens.reefFont, /Nunito/, "and its fonts");
assert.equal(tokens.lagoonAgain, "#faffff", "data-preset=lagoon switches back to the default");
console.log("theme subtree: ok");
await browser.close();
