# Dashboard 01

- Copy into your app: `rust-ui add dashboard_01` (also copies `badge`, `button`, `card`, `chart`, `dropdown_menu`, `empty`, `icons`, `separator`, `sidebar`, `switch`, `table`, `theme_customizer`).
- Import: `use crate::blocks::dashboard_01::*;` after copying, or `use ui::blocks::dashboard_01::*;` with the `rust-ui-kit` crate's `blocks` feature (internal).

An app shell plus an overview page: a sidebar that collapses to icons, a header with a dark-mode switch and an account menu, key figures, a line and a bar chart, and a recent-orders table. Built from `sidebar`, `card`, `chart`, `table`, `badge`, `dropdown_menu`, `switch`, `empty` and `theme_customizer` (for `use_theme`).

```sh
rust-ui add dashboard_01
```

Use the whole screen, or only the shell around your own pages:

```rust
let mut page = use_signal(|| "home".to_string());
rsx! {
    DashboardShell {
        brand: "Acme",
        nav: vec![
            NavItem::new("home", "Home", rsx! { House {} }),
            NavItem::new("reports", "Reports", rsx! { ChartBar {} }),
        ],
        active: page(),
        on_navigate: move |id| page.set(id),
        on_sign_out: move |_| sign_out(),
        match page().as_str() {
            "home" => rsx! { Home {} },
            _ => rsx! { Reports {} },
        }
    }
}
```

## Props (`DashboardShell`)

- `brand: String`: the name in the sidebar and header; its first letter is the logo.
- `nav: Vec<NavItem>`: `NavItem::new(id, label, icon)`.
- `active: String`: the current page's `id`. Its label becomes the page `h1`.
- `on_navigate: Callback<String>`: called with the clicked item's `id`.
- `initials`, `on_sign_out`: the account menu.

## Accessibility

- The sidebar is a `nav` landmark named "Main"; the current page's link has `aria-current="page"`.
- `SidebarInset` is the page's `main`; it starts with the page title as an `h1`, and each card title is an `h2`.
- Collapsed to icons, every link keeps its name (and a tooltip); icons are hidden from screen readers.
- Charts are `role="img"` with a text summary; the table has a caption and right-aligned numbers.

## API

```rust
/// One link in the [`DashboardShell`] sidebar.
pub struct NavItem {
    pub id: String,
    pub label: String,
    /// Shown before the label and alone when the sidebar is collapsed. Hidden from screen readers.
    pub icon: Element,
}

/// The frame of an app screen: a collapsible sidebar (to icons), a header with the sidebar
/// toggle, a dark-mode switch and an account menu, and a `<main>` with the page title as its
/// `h1`. Put the page in `children`.
#[component]
pub fn DashboardShell(
    #[props(into)] brand: String,
    nav: Vec<NavItem>,
    /// The `id` of the current page; its link gets `aria-current` and its label is the `h1`.
    #[props(into)]
    active: String,
    #[props(default)] on_navigate: Callback<String>,
    /// Two letters for the account button.
    #[props(into, default = "ME".to_string())]
    initials: String,
    #[props(default)] on_sign_out: Callback<()>,
    children: Element,
) -> Element

/// A complete dashboard screen: [`DashboardShell`] with an overview page (key figures, two
/// charts, recent orders) and placeholders for the other pages. Replace the sample data with
/// your own.
#[component]
pub fn Dashboard01() -> Element
```
