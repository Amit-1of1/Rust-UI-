//! Customers: the page that pushes the kit hardest. Cells hold badges and a row menu, so it
//! can't use `DataTable` (text cells only) and composes `Table` + `table_view` instead.

use crate::data::{money, Customer, PLANS, STATUSES};
use crate::use_store;
use dioxus::prelude::*;
use ui::prelude::*;

const PAGE_SIZE: usize = 10;

fn columns() -> Vec<DataColumn> {
    vec![
        DataColumn::new("Name").sortable(),
        DataColumn::new("Email"),
        DataColumn::new("Plan").sortable(),
        DataColumn::new("Status").sortable(),
        DataColumn::new("MRR").sortable().numeric(),
    ]
}

fn text_row(c: &Customer) -> Vec<String> {
    vec![
        c.name.clone(),
        c.email.clone(),
        c.plan.clone(),
        c.status.clone(),
        money(c.mrr),
    ]
}

fn status_badge(status: &str) -> BadgeVariant {
    match status {
        "Active" => BadgeVariant::Primary,
        "Churned" => BadgeVariant::Destructive,
        _ => BadgeVariant::Outline,
    }
}

#[component(lazy)]
pub fn Customers() -> Element {
    let mut store = use_store();
    let mut query = use_signal(String::new);
    let mut status = use_signal(|| "All".to_string());
    let mut sort = use_signal(|| None::<(usize, SortDirection)>);
    let mut page = use_signal(|| 0usize);
    let mut editing = use_signal(|| None::<usize>);
    let mut deleting = use_signal(|| None::<usize>);
    let mut confirm_delete = use_signal(|| false);
    let toasts = use_toast();

    let columns = columns();
    // Status filtering happens before table_view, which only knows the free-text query.
    let visible: Vec<Customer> = store
        .customers
        .read()
        .iter()
        .filter(|c| status() == "All" || c.status == status())
        .cloned()
        .collect();
    let text: Vec<Vec<String>> = visible.iter().map(text_row).collect();
    let view = table_view(&columns, &text, &query(), sort(), page(), PAGE_SIZE);
    let first = view.page * PAGE_SIZE + 1;
    let last = view.page * PAGE_SIZE + view.rows.len();

    let status_options = std::iter::once("All")
        .chain(STATUSES)
        .enumerate()
        .map(|(i, s)| {
            rsx! {
                SelectOption::<String> { index: i, value: s.to_string(), text_value: "{s}", "{s}" }
            }
        });

    let deleting_name = deleting().and_then(|id| {
        store
            .customers
            .read()
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.name.clone())
    });

    rsx! {
        div { class: "db-toolbar",
            Input {
                r#type: "search",
                placeholder: "Filter customers",
                "aria-label": "Filter customers",
                value: "{query()}",
                oninput: move |e: FormEvent| {
                    query.set(e.value());
                    page.set(0);
                },
            }
            div { class: "db-inline-field",
                span { id: "status-filter-label", "Status" }
                div { role: "group", "aria-labelledby": "status-filter-label",
                    Select::<String> {
                        default_value: Some("All".to_string()),
                        on_value_change: move |v: Option<String>| {
                            status.set(v.unwrap_or_else(|| "All".into()));
                            page.set(0);
                        },
                        {status_options}
                    }
                }
            }
            div { class: "db-spacer" }
            Button { onclick: move |_| store.adding.set(true), "Add customer" }
        }
        if view.matched == 0 {
            Empty {
                EmptyHeader {
                    EmptyTitle { "No customers match" }
                    EmptyDescription { "Try a different filter or status." }
                }
                EmptyContent {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            query.set(String::new());
                            status.set("All".into());
                        },
                        "Clear filters"
                    }
                }
            }
        } else {
            Table {
                TableCaption { "Customers" }
                TableHeader {
                    TableRow {
                        for (index , column) in columns.iter().enumerate() {
                            {
                                let direction = match sort() {
                                    Some((c, d)) if c == index => d,
                                    _ => SortDirection::None,
                                };
                                rsx! {
                                    TableHead {
                                        key: "{index}",
                                        sort: direction,
                                        on_sort: column
                                            .sortable
                                            .then(|| {
                                                EventHandler::new(move |_| {
                                                    let next = if direction == SortDirection::None {
                                                        SortDirection::Ascending
                                                    } else {
                                                        direction.cycle()
                                                    };
                                                    sort.set(Some((index, next)));
                                                    page.set(0);
                                                })
                                            }),
                                        style: if column.numeric { "text-align: right;" },
                                        "{column.header}"
                                    }
                                }
                            }
                        }
                        TableHead {
                            span { class: "db-sr-only", "Actions" }
                        }
                    }
                }
                TableBody {
                    for i in view.rows.iter().copied() {
                        CustomerRow {
                            key: "{visible[i].id}",
                            customer: visible[i].clone(),
                            on_edit: move |id| editing.set(Some(id)),
                            on_delete: move |id| {
                                deleting.set(Some(id));
                                confirm_delete.set(true);
                            },
                        }
                    }
                }
            }
            div { class: "db-table-footer",
                p { role: "status", class: "db-muted",
                    "Showing {first}–{last} of {view.matched} customers"
                }
                if view.pages > 1 {
                    nav { "aria-label": "Customer pages", class: "db-pages",
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            disabled: view.page == 0,
                            onclick: move |_| page.set(view.page.saturating_sub(1)),
                            "Previous"
                        }
                        span { "Page {view.page + 1} of {view.pages}" }
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            disabled: view.page + 1 >= view.pages,
                            onclick: move |_| page.set(view.page + 1),
                            "Next"
                        }
                    }
                }
            }
        }
        CustomerSheet {
            customer: editing().and_then(|id| store.customers.read().iter().find(|c| c.id == id).cloned()),
            on_close: move |_| editing.set(None),
        }
        AddCustomer {}
        // AlertDialogAction closes the dialog *before* calling on_click, so the target can't be
        // cleared in on_open_change: on_click would then see None. Keep the target in its own
        // signal and the open state in another.
        AlertDialog {
            open: confirm_delete(),
            on_open_change: move |v: bool| confirm_delete.set(v),
            AlertDialogTitle { "Delete customer?" }
            AlertDialogDescription {
                "{deleting_name.clone().unwrap_or_default()} and their history will be removed. This can't be undone."
            }
            AlertDialogActions {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction {
                    on_click: move |_| {
                        if let Some(id) = deleting.take() {
                            store.customers.write().retain(|c| c.id != id);
                            toasts.success("Customer deleted".to_string(), ToastOptions::new());
                        }
                    },
                    "Delete"
                }
            }
        }
    }
}

#[component]
fn CustomerRow(
    customer: Customer,
    on_edit: EventHandler<usize>,
    on_delete: EventHandler<usize>,
) -> Element {
    let id = customer.id;
    rsx! {
        TableRow {
            TableCell { class: "db-strong", "{customer.name}" }
            TableCell { class: "db-muted", "{customer.email}" }
            TableCell { "{customer.plan}" }
            TableCell {
                Badge { variant: status_badge(&customer.status), "{customer.status}" }
            }
            TableCell { style: "text-align: right;", {money(customer.mrr)} }
            TableCell { style: "text-align: right;",
                DropdownMenu {
                    DropdownMenuTrigger { aria_label: "Actions for {customer.name}", "…" }
                    DropdownMenuContent {
                        DropdownMenuItem::<&'static str> {
                            index: 0usize,
                            value: "edit",
                            on_select: move |_| on_edit.call(id),
                            "Edit"
                        }
                        DropdownMenuItem::<&'static str> {
                            index: 1usize,
                            value: "delete",
                            on_select: move |_| on_delete.call(id),
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

/// Edit a customer in a side sheet.
#[component]
fn CustomerSheet(customer: Option<Customer>, on_close: EventHandler<()>) -> Element {
    let mut store = use_store();
    let toasts = use_toast();
    let form = use_form();
    let name = form.field(vec![required()]);
    let email_field = form.field(vec![required(), email()]);
    let mut plan = use_signal(|| PLANS[0].to_string());
    // A controlled Select wants a ReadSignal<Option<T>>, not a plain value.
    let plan_value = use_memo(move || Some(plan()));

    // Load the customer into the form whenever a different one is opened.
    let id = customer.as_ref().map(|c| c.id);
    let loaded = customer.clone();
    use_effect(use_reactive!(|id| {
        let _ = id;
        if let Some(c) = &loaded {
            name.set(c.name.clone());
            email_field.set(c.email.clone());
            plan.set(c.plan.clone());
        }
    }));

    let plans = PLANS.iter().enumerate().map(|(i, p)| {
        rsx! {
            SelectOption::<String> { index: i, value: p.to_string(), text_value: "{p}", "{p}" }
        }
    });

    rsx! {
        Sheet {
            open: customer.is_some(),
            on_open_change: move |v: bool| {
                if !v {
                    on_close.call(());
                }
            },
            "data-side": SheetSide::Right.as_str(),
            SheetHeader {
                SheetTitle { "Edit customer" }
                SheetDescription { "Changes are saved to this session only." }
            }
            Form {
                class: "db-sheet-form",
                state: form,
                on_valid: move |_| {
                    if let Some(id) = id {
                        if let Some(c) = store.customers.write().iter_mut().find(|c| c.id == id) {
                            c.name = name.value();
                            c.email = email_field.value();
                            c.plan = plan();
                        }
                    }
                    toasts.success("Customer saved".to_string(), ToastOptions::new());
                    on_close.call(());
                },
                TextField { field: name, label: "Name", autocomplete: "off" }
                TextField { field: email_field, label: "Email", r#type: "email", autocomplete: "off" }
                div { class: "db-field",
                    span { id: "edit-plan-label", class: "db-label", "Plan" }
                    div { role: "group", "aria-labelledby": "edit-plan-label",
                        Select::<String> {
                            value: Some(plan_value.into()),
                            on_value_change: move |v: Option<String>| {
                                if let Some(v) = v {
                                    plan.set(v);
                                }
                            },
                            {plans}
                        }
                    }
                }
                SheetFooter {
                    Button { r#type: "submit", "Save changes" }
                }
            }
            SheetContentClose {}
        }
    }
}

/// "Add customer" dialog, opened from the toolbar or the command palette.
#[component]
fn AddCustomer() -> Element {
    let mut store = use_store();
    let toasts = use_toast();
    let form = use_form();
    let name = form.field(vec![required(), min_len(2)]);
    let email_field = form.field(vec![required(), email()]);
    let mut plan = use_signal(|| "Pro".to_string());
    let mut trial = use_signal(|| true);
    // The command palette can open this page and the dialog at once. A dialog that mounts
    // already open misses its focus trap (its script isn't loaded yet), so open after mount.
    let mut mounted = use_signal(|| false);
    use_effect(move || mounted.set(true));

    let plans = PLANS.iter().enumerate().map(|(i, p)| {
        rsx! {
            RadioItem { value: p.to_string(), index: i, "{p}" }
        }
    });

    rsx! {
        Dialog {
            open: (store.adding)() && mounted(),
            on_open_change: move |v| store.adding.set(v),
            DialogTitle { "Add customer" }
            DialogDescription { "They'll get an invite email (not in this demo)." }
            Form {
                state: form,
                on_valid: move |_| {
                    let id = store.customers.read().iter().map(|c| c.id).max().unwrap_or(0) + 1;
                    let plan_name = plan();
                    store
                        .customers
                        .write()
                        .insert(
                            0,
                            Customer {
                                id,
                                name: name.value(),
                                email: email_field.value(),
                                mrr: if plan_name == "Free" { 0 } else { 49 },
                                plan: plan_name,
                                status: if trial() { "Trial" } else { "Active" }.into(),
                            },
                        );
                    form.reset();
                    store.adding.set(false);
                    toasts.success("Customer added".to_string(), ToastOptions::new());
                },
                TextField { field: name, label: "Name", autocomplete: "off" }
                TextField { field: email_field, label: "Email", r#type: "email", autocomplete: "off" }
                div { class: "db-field",
                    span { id: "add-plan-label", class: "db-label", "Plan" }
                    RadioGroup {
                        "aria-labelledby": "add-plan-label",
                        value: Some(plan()),
                        on_value_change: move |v| plan.set(v),
                        horizontal: true,
                        {plans}
                    }
                }
                div { class: "db-check",
                    Checkbox {
                        id: "add-trial",
                        checked: Some(if trial() { CheckboxState::Checked } else { CheckboxState::Unchecked }),
                        on_checked_change: move |s| trial.set(s == CheckboxState::Checked),
                    }
                    Label { html_for: "add-trial", "Start on a 14-day trial" }
                }
                div { class: "db-actions",
                    Button {
                        r#type: "button",
                        variant: ButtonVariant::Outline,
                        onclick: move |_| store.adding.set(false),
                        "Cancel"
                    }
                    Button { r#type: "submit", "Add customer" }
                }
            }
        }
    }
}
