#!/usr/bin/env node
// Generates crates/ui/src/components/icons/atoll.rs from the Atoll icon sources: one SVG per
// icon in crates/ui/icons/atoll/<name>.svg plus crates/ui/icons/atoll/icons.json (default
// accessible name, keywords, category, reserved flag). Every icon becomes shape data
// (`pub const CRATE: IconData`) and a component (`pub fn Crate(props: IconProps)`), both under
// `ui::icons::atoll`, the same shape types and renderer as the Lucide icons.
//
//   node scripts/gen-atoll-icons.mjs           # write atoll.rs
//   node scripts/gen-atoll-icons.mjs --check   # fail if atoll.rs is out of date
//
// Needs `rustfmt` on PATH. No npm dependencies. The drawing rules (grid, keylines, stroke)
// are in docs/ICONS.md; this script enforces the mechanical ones and fails on anything else.
import { execFileSync } from "node:child_process";
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const SOURCE = join(ROOT, "crates/ui/icons/atoll");
const OUTPUT = join(ROOT, "crates/ui/src/components/icons/atoll.rs");

// Categories the docs page groups by, in display order.
const CATEGORIES = ["system", "navigation", "status", "content", "people", "dev", "app", "agent", "data", "design", "brand"];
// Names a component may not take: components are functions, so they would clash with these
// value names from Rust's prelude or the icons module (types and traits like `Copy` are fine).
const RESERVED_RUST = new Set(["Some", "None", "Ok", "Err", "Icon"]);
// The safe area: shapes stay inside 1..23 so a 2px stroke never leaves the 24px box.
const MIN = 1;
const MAX = 23;

const pascal = (kebab) => kebab.split("-").map((s) => s.charAt(0).toUpperCase() + s.slice(1)).join("");
const constant = (kebab) => kebab.replaceAll("-", "_").toUpperCase();
const str = (s) => JSON.stringify(String(s));
const opt = (v) => (v === undefined ? "None" : `Some(${str(v)})`);

function attrs(text) {
  const out = {};
  for (const m of text.matchAll(/([a-zA-Z:-]+)="([^"]*)"/g)) out[m[1]] = m[2];
  return out;
}

function inside(icon, label, ...values) {
  for (const v of values) {
    const n = Number(v);
    if (!Number.isFinite(n)) throw new Error(`${icon}: ${label} is not a number: ${v}`);
    if (n < MIN - 1e-9 || n > MAX + 1e-9) throw new Error(`${icon}: ${label} ${v} is outside the ${MIN}..${MAX} safe area`);
  }
}

function node(tag, a, icon) {
  const allowed = {
    path: ["d", "fill", "stroke"],
    circle: ["cx", "cy", "r", "fill", "stroke"],
    ellipse: ["cx", "cy", "rx", "ry"],
    rect: ["x", "y", "width", "height", "rx", "ry"],
    line: ["x1", "y1", "x2", "y2"],
    polyline: ["points"],
    polygon: ["points"],
  }[tag];
  if (!allowed) throw new Error(`${icon}: unsupported element <${tag}>`);
  for (const k of Object.keys(a)) {
    if (!allowed.includes(k)) throw new Error(`${icon}: unsupported attribute ${k} on <${tag}>`);
  }
  const filled = a.fill !== undefined;
  if (filled && (a.fill !== "currentColor" || a.stroke !== "none")) {
    throw new Error(`${icon}: a filled shape is fill="currentColor" stroke="none"`);
  }
  if (!filled && a.stroke !== undefined) throw new Error(`${icon}: stroke is set on the <svg>, not per shape`);
  switch (tag) {
    case "path":
      return filled ? `Fill { d: ${str(a.d)} }` : `Path { d: ${str(a.d)} }`;
    case "circle": {
      const r = Number(a.r);
      inside(icon, "circle edge", Number(a.cx) - r, Number(a.cx) + r, Number(a.cy) - r, Number(a.cy) + r);
      return `Circle { cx: ${str(a.cx)}, cy: ${str(a.cy)}, r: ${str(a.r)}, filled: ${filled} }`;
    }
    case "ellipse":
      inside(icon, "ellipse edge", Number(a.cx) - Number(a.rx), Number(a.cx) + Number(a.rx), Number(a.cy) - Number(a.ry), Number(a.cy) + Number(a.ry));
      return `Ellipse { cx: ${str(a.cx)}, cy: ${str(a.cy)}, rx: ${str(a.rx)}, ry: ${str(a.ry)} }`;
    case "rect":
      inside(icon, "rect edge", a.x, a.y, Number(a.x) + Number(a.width), Number(a.y) + Number(a.height));
      return `Rect { x: ${str(a.x)}, y: ${str(a.y)}, width: ${str(a.width)}, height: ${str(a.height)}, rx: ${opt(a.rx)}, ry: ${opt(a.ry)} }`;
    case "line":
      inside(icon, "line end", a.x1, a.y1, a.x2, a.y2);
      return `Line { x1: ${str(a.x1)}, y1: ${str(a.y1)}, x2: ${str(a.x2)}, y2: ${str(a.y2)} }`;
    case "polyline":
      inside(icon, "point", ...a.points.trim().split(/[\s,]+/));
      return `Polyline { points: ${str(a.points)} }`;
    case "polygon":
      inside(icon, "point", ...a.points.trim().split(/[\s,]+/));
      return `Polygon { points: ${str(a.points)} }`;
  }
}

function parseSvg(icon, text) {
  const root = text.match(/<svg\b([^>]*)>/);
  if (!root) throw new Error(`${icon}: no <svg> element`);
  const ra = attrs(root[1]);
  if (ra.viewBox !== "0 0 24 24") throw new Error(`${icon}: viewBox must be "0 0 24 24"`);
  if (/<(g|use|defs|style|text|image|title|mask|clipPath)\b/.test(text)) {
    throw new Error(`${icon}: only flat shapes (no groups, text, styles or titles)`);
  }
  if (/\btransform=/.test(text)) throw new Error(`${icon}: transforms are not allowed; draw the shape in place`);
  const body = text.slice(root.index + root[0].length, text.lastIndexOf("</svg>"));
  const shapes = [];
  for (const m of body.matchAll(/<([a-z]+)\b([^>]*?)\/>/g)) shapes.push(node(m[1], attrs(m[2]), icon));
  if (shapes.length === 0) throw new Error(`${icon}: no shapes`);
  return shapes;
}

function generate() {
  const meta = JSON.parse(readFileSync(join(SOURCE, "icons.json"), "utf8"));
  const files = readdirSync(SOURCE).filter((f) => f.endsWith(".svg")).map((f) => f.slice(0, -4)).sort();
  const names = Object.keys(meta.icons).sort();
  const missing = files.filter((f) => !meta.icons[f]);
  const orphan = names.filter((n) => !files.includes(n));
  if (missing.length) throw new Error(`no entry in icons.json for: ${missing.join(", ")}`);
  if (orphan.length) throw new Error(`icons.json lists icons without an SVG: ${orphan.join(", ")}`);

  const data = [];
  const comps = [];
  const index = [];
  for (const icon of names) {
    const m = meta.icons[icon];
    if (!/^[a-z][a-z0-9]*(-[a-z0-9]+)*$/.test(icon)) throw new Error(`${icon}: names are kebab-case`);
    const P = pascal(icon);
    const C = constant(icon);
    if (RESERVED_RUST.has(P)) throw new Error(`${icon}: ${P} would shadow a Rust or Dioxus name`);
    if (!m.label || typeof m.label !== "string") throw new Error(`${icon}: needs a default accessible name ("label")`);
    if (!Array.isArray(m.keywords) || m.keywords.length < 2) throw new Error(`${icon}: needs at least two keywords`);
    if (!CATEGORIES.includes(m.category)) throw new Error(`${icon}: category must be one of ${CATEGORIES.join(", ")}`);
    const rtl = Boolean(m.mirror_rtl);
    const reserved = m.reserved ? str(m.reserved) : null;
    const shapes = parseSvg(icon, readFileSync(join(SOURCE, `${icon}.svg`), "utf8")).join(", ");
    const kw = m.keywords.join(", ");
    const doc = [
      `/// Atoll \`${icon}\`: ${m.label} (${kw}).${rtl ? " Mirrored in right-to-left text." : ""}`,
      ...(reserved ? [`/// Reserved for: ${m.reserved}.`] : []),
    ];
    data.push(...doc, `pub const ${C}: IconData = IconData { name: ${str(icon)}, mirror_rtl: ${rtl}, nodes: &[${shapes}] };`);
    comps.push(...doc, `pub fn ${P}(props: IconProps) -> Element { lucide_icon(${C}, props) }`);
    index.push(
      `IconMeta { name: ${str(icon)}, label: ${str(m.label)}, keywords: &[${m.keywords.map(str).join(", ")}], category: ${str(m.category)}, reserved: ${reserved ? `Some(${reserved})` : "None"} }`,
    );
  }
  const text = [
    "// @generated by scripts/gen-atoll-icons.mjs from crates/ui/icons/atoll. Do not edit;",
    "// change the SVG or icons.json there and re-run the script.",
    "",
    `//! Atoll's own icons (${names.length}), drawn for the kit on the same 24px grid as Lucide.`,
    "//! Each is shape data (`atoll::CRATE`, for [`Icon`](super::Icon)) and a component",
    "//! (`atoll::Crate { size: IconSize::Sm }`), rendered exactly like the Lucide icons.",
    "//!",
    "//! Only the icons an app names end up in its binary. [`ALL`] names every one, so use it",
    "//! only in tests, tools and the icon browser; [`INDEX`] holds only names and keywords",
    "//! and costs no shapes.",
    "#![allow(non_snake_case)]",
    "",
    "use super::component::{lucide_icon, IconData, IconMeta, IconNode::*, IconProps};",
    "use dioxus::prelude::Element;",
    "",
    `/// The categories of [\`INDEX\`], in display order.`,
    `pub const CATEGORIES: [&str; ${CATEGORIES.length}] = [${CATEGORIES.map(str).join(", ")}];`,
    "",
    ...data,
    "",
    ...comps,
    "",
    "/// Every icon, in name order. Naming this keeps all of them in the binary.",
    `pub static ALL: [IconData; ${names.length}] = [${names.map(constant).join(", ")}];`,
    "",
    "/// Every icon's default accessible name, keywords, category and reserved meaning, in the",
    "/// same order as [`ALL`]. Strings only: naming this keeps no shapes in the binary.",
    `pub static INDEX: [IconMeta; ${names.length}] = [${index.join(", ")}];`,
    "",
    "/// The entry for `name` (`\"crate\"`), for its default accessible name and keywords.",
    "pub fn meta(name: &str) -> Option<&'static IconMeta> {",
    "    INDEX.iter().find(|m| m.name == name)",
    "}",
    "",
  ].join("\n");
  return { text, count: names.length };
}

const check = process.argv.includes("--check");
const { text, count } = generate();
const formatted = execFileSync("rustfmt", ["--edition", "2021", "--emit", "stdout", "--quiet"], {
  input: text,
  encoding: "utf8",
  maxBuffer: 16 * 1024 * 1024,
});
if (check) {
  if (readFileSync(OUTPUT, "utf8") !== formatted) {
    console.error(`${OUTPUT} is out of date: run node scripts/gen-atoll-icons.mjs`);
    process.exit(1);
  }
  console.log(`atoll icons: ${count}, up to date`);
} else {
  writeFileSync(OUTPUT, formatted);
  console.log(`atoll icons: ${count} -> ${OUTPUT}`);
}
