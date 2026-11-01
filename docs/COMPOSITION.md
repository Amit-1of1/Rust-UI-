# Composing components

Most Atoll components are small parts meant to be nested: a `Card` holds a `CardHeader`, a `Field` holds a label, an input and an error, a `Dialog` holds whatever you put in it. Some take content through a named prop instead of `children` (a "slot"): `DataTable`'s `toolbar` and `render_cell`, `Stepper`'s `footer`, a chat message's `avatar` and `actions`, `BottomNav`'s `icon`. This page shows the compositions that come up in real apps, taken from the example apps in `apps/`.

Every example here is compiled by `cargo test --doc -p atoll-ui` (the page is included as a doctest in `crates/ui/src/lib.rs`), so it can't fall behind the API. Each one is a complete component you can paste into an app that depends on `atoll-ui`; with copied components, change `use ui::prelude::*;` to `use crate::components::...`.

The rules behind all of them:

- **Nest the parts a component documents, in the order its "Component Structure" shows.** The parts carry the ARIA wiring between them (a dialog's title names the dialog, a field's error describes the input). The props table on each component's docs page lists what every part accepts.
- **A slot takes any `Element`**, so it can hold other kit components. Keep interactive things in slots meant for them (`toolbar`, `actions`, `footer`); a `render_cell` that returns a button is fine, a whole form is not.
- **Never put a button inside a button.** Triggers such as `TooltipTrigger` and `DropdownMenuTrigger` already render their own `<button>`; give them text or an icon, not a `Button`.
- **Style the layout around components, not inside them.** Put `style` or `class` on the component (they go on its root element, and `class` is merged with the component's own; see [BEST_PRACTICES.md](BEST_PRACTICES.md)) or use a wrapping `div` for spacing and grids.

## A settings card with a validated field

`Card` parts are plain containers; `Field` ties a label, a hint and an error to the input through ids from `use_field_ids`.

```rust
use dioxus::prelude::*;
use ui::prelude::*;

#[component]
fn ProfileCard() -> Element {
    let ids = use_field_ids();
    let mut name = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    rsx! {
        Card { style: "max-width: 28rem;",
            CardHeader {
                CardTitle { "Profile" }
                CardDescription { "This is how others see you." }
            }
            CardContent {
                Field { invalid: error().is_some(),
                    FieldLabel { html_for: ids.control.clone(), "Display name" }
                    Input {
                        id: ids.control.clone(),
                        value: name,
                        aria_invalid: error().is_some().then_some("true"),
                        aria_describedby: ids.described_by(error().is_some()),
                        oninput: move |e: FormEvent| name.set(e.value()),
                    }
                    FieldDescription { id: ids.description.clone(), "Shown on your public page." }
                    FieldError { id: ids.error.clone(), message: error() }
                }
            }
            CardFooter {
                Button {
                    onclick: move |_| error.set(validate::required(&name.read()).err()),
                    "Save"
                }
            }
        }
    }
}
```

## A confirmation dialog with its own actions

`Dialog`'s `title` and `description` name and describe it; everything in `children` comes after them. The open state lives in your signal, so the buttons inside can close it.

```rust
use dioxus::prelude::*;
use ui::prelude::*;

#[component]
fn DeleteProject(name: String, on_delete: EventHandler<()>) -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { variant: ButtonVariant::Destructive, onclick: move |_| open.set(true), "Delete project" }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Delete {name}?",
            description: "Its deployments and settings are removed for good.",
            div { style: "display: flex; gap: 0.75rem; justify-content: flex-end; margin-top: 1rem;",
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Cancel" }
                Button {
                    variant: ButtonVariant::Destructive,
                    onclick: move |_| {
                        open.set(false);
                        on_delete.call(());
                    },
                    "Delete"
                }
            }
        }
    }
}
```

## A data table with slots: a toolbar filter and badge cells

`toolbar` takes extra controls shown beside the table's own filter box. `render_cell` is called with `(row, column)` and returns what to show; the text in `rows` is still what filtering and sorting use, so a badge cell sorts by its label.

```rust
use dioxus::prelude::*;
use ui::prelude::*;

#[component]
fn Orders() -> Element {
    let mut open_only = use_signal(|| false);
    let all = [("#1042", "Ada", "Paid"), ("#1043", "Grace", "Open"), ("#1044", "Alan", "Refunded")];
    let rows: Vec<Vec<String>> = all
        .iter()
        .filter(|(_, _, status)| !open_only() || *status == "Open")
        .map(|(id, who, status)| vec![id.to_string(), who.to_string(), status.to_string()])
        .collect();
    let cells = rows.clone();

    rsx! {
        DataTable {
            columns: vec![
                DataColumn::new("Order"),
                DataColumn::new("Customer").sortable(),
                DataColumn::new("Status").sortable(),
            ],
            rows,
            caption: "Recent orders",
            toolbar: rsx! {
                Toggle { pressed: open_only(), on_pressed_change: move |on| open_only.set(on), "Open only" }
            },
            render_cell: move |(row, col): (usize, usize)| {
                let text = cells[row][col].clone();
                if col == 2 {
                    let variant = if text == "Refunded" { BadgeVariant::Destructive } else { BadgeVariant::Secondary };
                    rsx! { Badge { variant, "{text}" } }
                } else {
                    rsx! { "{text}" }
                }
            },
        }
    }
}
```

## An empty state that leads somewhere

`Empty` parts stack media, a title, a description and actions. Put the next step in `EmptyContent`; an icon in `EmptyMedia` is decoration, so it stays hidden from screen readers.

```rust
use dioxus::prelude::*;
use ui::prelude::*;

#[component]
fn NoProjects(on_create: EventHandler<()>) -> Element {
    rsx! {
        Empty {
            EmptyHeader {
                EmptyTitle { "No projects yet" }
                EmptyDescription { "Create your first project to see it here." }
            }
            EmptyContent {
                Button { onclick: move |_| on_create.call(()), "Create project" }
            }
        }
    }
}
```

## A row of actions behind a menu

A `DropdownMenu` holds its trigger and its content; each item carries a typed `value` and its position (`index`, used for arrow-key order). An `IconButton` needs a `label`, so an icon-only control is never nameless.

```rust
use dioxus::prelude::*;
use ui::icons::Settings;
use ui::prelude::*;

#[component]
fn RowActions(on_pick: EventHandler<&'static str>) -> Element {
    rsx! {
        div { style: "display: flex; gap: 0.5rem; align-items: center;",
            IconButton { label: "Settings", variant: ButtonVariant::Ghost,
                Settings { size: "1rem", "aria-hidden": "true" }
            }
            DropdownMenu {
                DropdownMenuTrigger { "More" }
                DropdownMenuContent {
                    for (i, action) in ["Edit", "Duplicate", "Archive"].into_iter().enumerate() {
                        DropdownMenuItem::<&'static str> {
                            value: action,
                            index: i,
                            on_select: move |v| on_pick.call(v),
                            {action}
                        }
                    }
                }
            }
        }
    }
}
```

## Larger compositions

Whole screens built the same way, to read or copy:

- **Blocks** (`atoll add login_01`, `signup_01`, `dashboard_01`, `settings_01`): sign-in, sign-up with a code step, a dashboard shell with a collapsible sidebar, settings in tabs. Source in `crates/ui/src/blocks/`.
- **Example apps**: `apps/dashboard` (tables with row menus, edit sheets, a command palette), `apps/booking` (routed pages, a three-step checkout, date-range booking), `apps/site` (a landing page), `apps/showcase` (web, desktop and phone layouts). Each one's `GAPS.md` lists what the kit lacked while it was built.
