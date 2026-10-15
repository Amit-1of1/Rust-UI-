# Rust UI

> Accessible, animated, shadcn-style components for Dioxus 0.7 (web and the desktop webview; native Blitz is a preview). Use them as source copied into your app (`rust-ui add <name>` or `dx components add`) or as the `rust-ui-kit` crate from git.

Rules for writing app code with this kit:

- Dioxus is pinned to `0.7.x`. Write Dioxus 0.7 code (`#[component]`, `rsx!`, signals); don't use 0.6 or 0.8 APIs.
- Load the theme once near the root: `ui::ThemeStyles {}` with the crate, or `document::Stylesheet { href: asset!("/assets/theme.css") }` after `rust-ui init`. Without it components are unstyled.
- Import components from `ui::prelude::*` with the crate, or `crate::components::<name>::*` after copying. Icons come from `ui::icons` by path (`ui::icons::Check`), hooks from `ui::hooks`, animation from `ui::motion`.
- Restyle with `class` (it adds to the kit's class) and the theme's CSS custom properties (`--primary-color`, `--accent-color`, `--radius`, ...). Don't hard-code colours, and don't change a component's markup: ARIA roles, focus and keys come from `dioxus-primitives`.
- Light and dark mode: `data-theme="light"` or `"dark"` on `<html>`; accent presets: `data-accent="blue|green|violet|rose|orange"`.
- Every interactive component works with the keyboard and every animation respects `prefers-reduced-motion`; keep it that way in your own code.
- Don't add JavaScript (`document::eval`) to get behaviour the kit already has. Ctrl+K and other shortcuts go through `ui::hooks::use_hotkey`.
- Each component page below has its props (the Rust signatures) and a working example. Prefer the example's patterns over guessing prop names.
