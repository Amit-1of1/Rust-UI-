#!/usr/bin/env node
// Generates crates/ui/src/components/icons/lucide.rs from the published `lucide-static` npm
// package: every Lucide icon as shape data (`pub const CHECK: IconData`) plus a component
// (`pub fn Check(props: IconProps)`), and Lucide's old names as aliases.
//
//   node scripts/gen-lucide-icons.mjs                    # the pinned version below, via `npm pack`
//   node scripts/gen-lucide-icons.mjs --version 1.58.0   # another release
//   node scripts/gen-lucide-icons.mjs --from <dir|tgz>   # an unpacked or packed lucide-static
//
// Needs `npm` (for the download), `tar` and `rustfmt` on PATH. No npm dependencies.
// After an upgrade: run the kit's tests (`cargo test -p rust-ui-kit --test icons`), check that no
// name the kit or the apps import disappeared, and update the version in THIRD_PARTY.md.
import { execFileSync } from "node:child_process";
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const PINNED_VERSION = "1.57.0";
const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const OUTPUT = join(ROOT, "crates/ui/src/components/icons/lucide.rs");

// Icons that point along the reading direction, mirrored under `dir="rtl"` (an `Icon` can
// override this with `mirror_rtl`). Media controls, clocks, charts and up/down icons don't
// flip, nor do icons that only contain the words "left"/"right" for alignment.
const RTL_PREFIXES = [
  "arrow-", "arrows-", "chevron-", "chevrons-", "circle-arrow-", "circle-chevron-",
  "square-arrow-", "square-chevron-", "move-", "corner-", "panel-", "redo", "undo", "reply",
  "forward", "list-indent-", "indent-", "log-in", "log-out", "between-horizontal-",
  "iteration-ccw", "iteration-cw", "text-wrap", "wrap-text", "step-back", "step-forward",
];
const RTL_HORIZONTAL = /(^|-)(left|right|start|end)(-|$)/;
const RTL_ALWAYS = new Set([
  "redo", "redo-2", "redo-dot", "undo", "undo-2", "undo-dot", "reply", "reply-all", "forward",
  "log-in", "log-out", "list-indent-increase", "list-indent-decrease", "indent-increase",
  "indent-decrease", "text-wrap", "wrap-text", "iteration-ccw", "iteration-cw",
  "between-horizontal-start", "between-horizontal-end",
]);

function mirrorsInRtl(name) {
  if (RTL_ALWAYS.has(name)) return true;
  if (!RTL_PREFIXES.some((p) => name.startsWith(p))) return false;
  // `arrow-up`, `chevrons-up-down` and friends only move vertically.
  return RTL_HORIZONTAL.test(name);
}

function args() {
  const out = { version: PINNED_VERSION, from: null };
  const argv = process.argv.slice(2);
  for (let i = 0; i < argv.length; i++) {
    if (argv[i] === "--version") out.version = argv[++i];
    else if (argv[i] === "--from") out.from = argv[++i];
    else {
      console.error(`unknown argument: ${argv[i]}`);
      process.exit(2);
    }
  }
  return out;
}

/// Returns the unpacked package directory and a cleanup function.
function fetchPackage({ version, from }) {
  const work = mkdtempSync(join(tmpdir(), "lucide-"));
  const cleanup = () => rmSync(work, { recursive: true, force: true });
  let tgz = from;
  if (from && statSync(from).isDirectory()) return { dir: resolve(from), cleanup };
  if (!tgz) {
    // `npm pack` checks the tarball against the registry's integrity hash.
    const name = execFileSync("npm", ["pack", `lucide-static@${version}`, "--silent"], {
      cwd: work,
      encoding: "utf8",
    }).trim().split("\n").pop();
    tgz = join(work, name);
  }
  execFileSync("tar", ["xzf", resolve(tgz), "-C", work]);
  return { dir: join(work, "package"), cleanup };
}

const pascal = (kebab) =>
  kebab.split("-").map((s) => s.charAt(0).toUpperCase() + s.slice(1)).join("");
const constant = (kebab) => {
  const c = kebab.replaceAll("-", "_").toUpperCase();
  return /^[0-9]/.test(c) ? `I_${c}` : c;
};
const str = (s) => JSON.stringify(String(s));
const opt = (v) => (v === undefined ? "None" : `Some(${str(v)})`);

function node(tag, a, icon) {
  const allowed = {
    path: ["d"],
    circle: ["cx", "cy", "r", "fill"],
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
  if (a.fill !== undefined && a.fill !== "currentColor") {
    throw new Error(`${icon}: fill="${a.fill}" (only currentColor is supported)`);
  }
  switch (tag) {
    case "path":
      return `Path { d: ${str(a.d)} }`;
    case "circle":
      return `Circle { cx: ${str(a.cx)}, cy: ${str(a.cy)}, r: ${str(a.r)}, filled: ${a.fill !== undefined} }`;
    case "ellipse":
      return `Ellipse { cx: ${str(a.cx)}, cy: ${str(a.cy)}, rx: ${str(a.rx)}, ry: ${str(a.ry)} }`;
    case "rect":
      return `Rect { x: ${str(a.x ?? "0")}, y: ${str(a.y ?? "0")}, width: ${str(a.width)}, height: ${str(a.height)}, rx: ${opt(a.rx)}, ry: ${opt(a.ry)} }`;
    case "line":
      return `Line { x1: ${str(a.x1)}, y1: ${str(a.y1)}, x2: ${str(a.x2)}, y2: ${str(a.y2)} }`;
    case "polyline":
      return `Polyline { points: ${str(a.points)} }`;
    case "polygon":
      return `Polygon { points: ${str(a.points)} }`;
  }
}

function generate(dir) {
  const pkg = JSON.parse(readFileSync(join(dir, "package.json"), "utf8"));
  const nodes = JSON.parse(readFileSync(join(dir, "icon-nodes.json"), "utf8"));
  const tags = existsSync(join(dir, "tags.json"))
    ? JSON.parse(readFileSync(join(dir, "tags.json"), "utf8"))
    : {};
  const license = readFileSync(join(dir, "LICENSE"), "utf8").trim();
  const index = readFileSync(join(dir, "dist/esm/lucide-static.mjs"), "utf8");

  // Old names: `export { default as AlertCircle, default as CircleAlert } from './icons/circle-alert.mjs'`.
  const aliases = new Map();
  for (const m of index.matchAll(/export \{([^}]*)\} from '\.\/icons\/([a-z0-9-]+)\.mjs'/g)) {
    const icon = m[2];
    if (!nodes[icon]) throw new Error(`export for unknown icon ${icon}`);
    for (const part of m[1].split(",")) {
      const name = part.trim().split(/\s+as\s+/)[1];
      if (name && name !== pascal(icon)) {
        if (!aliases.has(icon)) aliases.set(icon, []);
        aliases.get(icon).push(name);
      }
    }
  }

  // Kebab names of every export (aliases included) from the per-icon SVG files, for constants.
  const kebabOf = new Map();
  for (const f of readdirSync(join(dir, "icons"))) {
    if (f.endsWith(".svg")) kebabOf.set(pascal(f.slice(0, -4)), f.slice(0, -4));
  }

  const names = Object.keys(nodes).sort();
  // Constants and components live in separate modules (`lucide::X` is data, `components::X`
  // the component), so each set of names only has to be unique on its own.
  const unique = () => {
    const seen = new Set();
    return {
      has: (n) => seen.has(n),
      claim: (n) => {
        if (seen.has(n)) throw new Error(`duplicate Rust name ${n}`);
        seen.add(n);
      },
    };
  };
  const consts = unique();
  const fns = unique();
  const header = [
    `// @generated by scripts/gen-lucide-icons.mjs from lucide-static ${pkg.version}. Do not edit;`,
    "// re-run the script instead.",
    "//",
    "// Icon data: Lucide (https://lucide.dev), used under the licence below.",
    "//",
    ...license.split("\n").map((l) => (l ? `// ${l}` : "//")),
    "",
    `//! Every Lucide icon (lucide-static ${pkg.version}, ${names.length} icons) as shape data:`,
    "//! `Icon { icon: lucide::CHECK }`. The same icons as components (`Check { size: \"1rem\" }`)",
    "//! are in [`components`], re-exported as `ui::icons::Check`.",
    "//!",
    "//! Only the icons an app names end up in its binary; [`ALL`] names every one, so use it",
    "//! only in tests and tools.",
    "",
    "use super::component::{IconData, IconNode::*};",
    "",
    `/// The Lucide release these icons come from.`,
    `pub const LUCIDE_VERSION: &str = ${str(pkg.version)};`,
    "",
  ];
  const data = [];
  const comps = [];
  const all = [];
  const constAliases = [];
  const fnAliases = [];
  for (const icon of names) {
    const C = constant(icon);
    const P = pascal(icon);
    consts.claim(C);
    fns.claim(P);
    const kw = (tags[icon] || []).join(", ");
    const rtl = mirrorsInRtl(icon);
    const shapes = nodes[icon].map(([t, a]) => node(t, a, icon)).join(", ");
    data.push(
      `/// Lucide \`${icon}\`${kw ? ` (${kw})` : ""}.${rtl ? " Mirrored in right-to-left text." : ""}`,
      `pub const ${C}: IconData = IconData { name: ${str(icon)}, mirror_rtl: ${rtl}, nodes: &[${shapes}] };`,
    );
    comps.push(
      `/// Lucide \`${icon}\` ([\`${C}\`](super::${C})).`,
      `pub fn ${P}(props: IconProps) -> Element { lucide_icon(super::${C}, props) }`,
    );
    all.push(C);
    for (const alias of (aliases.get(icon) || []).sort()) {
      fns.claim(alias);
      fnAliases.push(`pub use self::${P} as ${alias};`);
      const aliasConst = kebabOf.has(alias) ? constant(kebabOf.get(alias)) : null;
      if (aliasConst && !consts.has(aliasConst)) {
        consts.claim(aliasConst);
        constAliases.push(`pub use self::${C} as ${aliasConst};`);
      }
    }
  }
  const text = [
    ...header,
    ...data,
    "",
    "/// Every icon, in name order. Naming this keeps all of them in the binary.",
    `pub static ALL: [IconData; ${all.length}] = [${all.join(", ")}];`,
    "",
    "// Older Lucide names, kept working after upstream renames.",
    ...constAliases,
    "",
    "/// One component per Lucide icon, re-exported as `ui::icons::Check`.",
    "pub mod components {",
    "#![allow(non_snake_case)]",
    "use super::super::component::{lucide_icon, IconProps};",
    "use dioxus::prelude::Element;",
    "",
    ...comps,
    "",
    "// Older Lucide names, kept working after upstream renames.",
    ...fnAliases,
    "}",
    "",
  ].join("\n");
  return { text, count: names.length, aliases: fnAliases.length, version: pkg.version };
}

const opts = args();
const { dir, cleanup } = fetchPackage(opts);
try {
  const { text, count, aliases, version } = generate(dir);
  const formatted = execFileSync("rustfmt", ["--edition", "2021", "--emit", "stdout", "--quiet"], {
    input: text,
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
  });
  writeFileSync(OUTPUT, formatted);
  console.log(`lucide-static ${version}: ${count} icons, ${aliases} alias exports -> ${OUTPUT}`);
} finally {
  cleanup();
}
