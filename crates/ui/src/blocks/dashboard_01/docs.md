# Dashboard 01

An app shell plus an overview page: a sidebar that collapses to icons, a header with a dark-mode switch and an account menu, key figures, a line and a bar chart, and a recent-orders table. Built from `sidebar`, `card`, `chart`, `table`, `badge`, `dropdown_menu`, `switch`, `empty` and `theme_customizer` (for `use_theme`).

```sh
atoll add dashboard_01
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
