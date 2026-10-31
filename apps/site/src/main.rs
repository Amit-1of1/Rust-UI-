//! "Orbit": a small marketing site made only from the Atoll kit. It exists to test the kit
//! in a real page (navigation, forms, overlays, charts, motion), not to be a product.

use dioxus::prelude::*;
use ui::motion::{AnimatePresence, FlipItem, FlipList, Pop, SpringPreset, SwipeDismiss};
use ui::prelude::*;

const SITE_CSS: Asset = asset!("/assets/site.css");

fn main() {
    dioxus::launch(App);
}

/// Set `data-theme` on <html>; the kit's theme tokens key off it.
fn apply_theme(dark: bool) {
    #[cfg(target_arch = "wasm32")]
    if let Some(root) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    {
        let _ = root.set_attribute("data-theme", if dark { "dark" } else { "light" });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = dark;
}

#[component]
fn App() -> Element {
    let mut palette = use_signal(|| false);
    use_hotkey("k", move || palette.set(true));
    rsx! {
        ThemeStyles {}
        document::Stylesheet { href: SITE_CSS }
        a { class: "skip-link", href: "#main", "Skip to content" }
        ToastProvider {
            Header { on_palette: move |_| palette.set(true) }
            main { id: "main",
                Hero {}
                Features {}
                Pricing {}
                Numbers {}
                Faq {}
                Signup {}
            }
            Footer {}
            Palette { open: palette() , on_open_change: move |v| palette.set(v) }
        }
    }
}

#[component]
fn Header(on_palette: EventHandler<()>) -> Element {
    let mut dark = use_signal(|| false);
    rsx! {
        header { class: "site-header",
            a { class: "site-brand", href: "#top", "Orbit" }
            nav { class: "site-nav", "aria-label": "Main",
                a { href: "#features", "Features" }
                a { href: "#pricing", "Pricing" }
                a { href: "#faq", "FAQ" }
                a { href: "#signup", "Sign up" }
            }
            div { class: "site-tools",
                Button { id: "open-palette", variant: ButtonVariant::Outline, size: ButtonSize::Sm,
                    onclick: move |_| on_palette.call(()),
                    "Search"
                    KbdGroup { Kbd { "Ctrl" } Kbd { "K" } }
                }
                Switch {
                    id: "theme-switch",
                    checked: dark(),
                    aria_label: "Dark mode",
                    on_checked_change: move |v| {
                        dark.set(v);
                        apply_theme(v);
                    },
                }
            }
        }
    }
}

#[component]
fn Hero() -> Element {
    let mut banner = use_signal(|| true);
    rsx! {
        section { class: "hero", id: "top", "aria-labelledby": "hero-title",
            Badge { variant: BadgeVariant::Secondary, "Orbit 1.0 is here" }
            h1 { id: "hero-title", "Plan the work. Ship the thing." }
            p { "Orbit keeps your team's tasks, docs and deadlines in one calm place, so nothing slips." }
            div { class: "hero-actions",
                a { href: "#signup", Button { size: ButtonSize::Lg, "Start free" } }
                a { href: "#pricing", Button { size: ButtonSize::Lg, variant: ButtonVariant::Outline, "See pricing" } }
            }
        }
        if banner() {
            div { class: "banner-wrap",
                SwipeDismiss { id: "banner", on_dismiss: move |_| banner.set(false), width: 560.0,
                    Alert {
                        AlertTitle { "New: weekly digests" }
                        AlertDescription { "Swipe this away, or focus it and press Delete." }
                    }
                }
            }
        }
    }
}

#[component]
fn Features() -> Element {
    let items = [
        (
            "Boards that breathe",
            "Drag work between columns and see it settle into place.",
        ),
        (
            "Docs next to tasks",
            "Write specs where the work lives. Link them in one keystroke.",
        ),
        (
            "Deadlines you can trust",
            "Orbit nudges early and stays quiet when you are on track.",
        ),
    ];
    rsx! {
        section { class: "site-section", id: "features", "aria-labelledby": "features-title",
            h2 { id: "features-title", "Everything in one orbit" }
            p { class: "site-lede", "Three ideas, done well." }
            div { class: "grid-3",
                for (title, body) in items {
                    Card {
                        CardHeader {
                            CardTitle { {title} }
                            CardDescription { {body} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Pricing() -> Element {
    rsx! {
        section { class: "site-section", id: "pricing", "aria-labelledby": "pricing-title",
            h2 { id: "pricing-title", "Simple pricing" }
            p { class: "site-lede", "Pay yearly and get two months free." }
            Tabs { default_value: "monthly".to_string(), horizontal: true,
                TabList {
                    TabTrigger { value: "monthly".to_string(), index: 0usize, "Monthly" }
                    TabTrigger { value: "yearly".to_string(), index: 1usize, "Yearly" }
                }
                TabContent { index: 0usize, value: "monthly".to_string(), PlanGrid { yearly: false } }
                TabContent { index: 1usize, value: "yearly".to_string(), PlanGrid { yearly: true } }
            }
        }
    }
}

#[component]
fn PlanGrid(yearly: bool) -> Element {
    let plans = [
        ("Solo", 0, "For one person", false),
        ("Team", 12, "For small teams", true),
        ("Company", 29, "For many teams", false),
    ];
    rsx! {
        div { class: "grid-3", style: "margin-top: 1.5rem;",
            for (name, monthly, blurb, popular) in plans {
                {
                    // Yearly is ten months for the price of twelve.
                    let price = if yearly { monthly * 10 } else { monthly };
                    let per = if yearly { "/ user / year" } else { "/ user / month" };
                    rsx! {
                        Card { key: "{name}", "data-plan": name,
                            CardHeader {
                                div { class: "plan-card-head",
                                    CardTitle { {name} }
                                    if popular { Badge { "Popular" } }
                                }
                                CardDescription { {blurb} }
                            }
                            CardContent {
                                p { class: "price", "data-price": "{price}", "${price} " small { {per} } }
                                ul { class: "plan-list",
                                    li { "Unlimited tasks" }
                                    li { if monthly > 0 { "Shared boards" } else { "One private board" } }
                                    li { if monthly > 12 { "Audit log and SSO" } else { "Email support" } }
                                }
                            }
                            CardFooter {
                                a { href: "#signup",
                                    Button { variant: if popular { ButtonVariant::Primary } else { ButtonVariant::Outline }, "Choose {name}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Numbers() -> Element {
    let data: Vec<ChartDatum> = [
        ("Mon", 42.0),
        ("Tue", 58.0),
        ("Wed", 71.0),
        ("Thu", 64.0),
        ("Fri", 90.0),
    ]
    .into_iter()
    .map(|(l, v)| ChartDatum::new(l, v))
    .collect();
    rsx! {
        section { class: "site-section", id: "numbers", "aria-labelledby": "numbers-title",
            h2 { id: "numbers-title", "Teams are shipping more" }
            p { class: "site-lede", "Tasks completed per day in an average Orbit workspace this week." }
            BarChart { data, label: "Tasks completed per weekday, Monday to Friday" }
            div { style: "margin-top: 2rem;",
                Table {
                    TableCaption { "What each plan includes" }
                    TableHeader {
                        TableRow {
                            TableHead { "Feature" }
                            TableHead { "Solo" }
                            TableHead { "Team" }
                            TableHead { "Company" }
                        }
                    }
                    TableBody {
                        TableRow { TableCell { "Boards" } TableCell { "1" } TableCell { "Unlimited" } TableCell { "Unlimited" } }
                        TableRow { TableCell { "History" } TableCell { "7 days" } TableCell { "1 year" } TableCell { "Forever" } }
                        TableRow { TableCell { "SSO" } TableCell { "No" } TableCell { "No" } TableCell { "Yes" } }
                    }
                }
            }
        }
    }
}

#[component]
fn Faq() -> Element {
    let items = [
        (
            "Can I try it first?",
            "Yes. Solo is free forever, and Team has a 14 day trial.",
        ),
        (
            "Do you offer refunds?",
            "Within 30 days of a yearly purchase, no questions asked.",
        ),
        (
            "Is my data private?",
            "Your workspace is encrypted at rest and never used for training.",
        ),
    ];
    rsx! {
        section { class: "site-section", id: "faq", "aria-labelledby": "faq-title",
            h2 { id: "faq-title", "Questions" }
            div { style: "max-width: 40rem;",
                Accordion { allow_multiple_open: false, horizontal: false,
                    for (i, (q, a)) in items.into_iter().enumerate() {
                        AccordionItem { index: i,
                            AccordionTrigger { {q} }
                            AccordionContent { p { style: "margin: 0; padding-bottom: 1rem;", {a} } }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Signup() -> Element {
    let form = use_form();
    let name = form.field(vec![required()]);
    let email = form.field(vec![required(), email()]);
    let mut people = use_signal(Vec::<(u32, String)>::new);
    let mut next = use_signal(|| 1u32);
    let mut welcome = use_signal(|| false);
    let toasts = use_toast();

    let joined = move |_| {
        let id = next();
        next.set(id + 1);
        let who = name.value().trim().to_string();
        people.write().insert(0, (id, who.clone()));
        toasts.success(
            format!("Welcome, {who}"),
            ToastOptions::new().description("Check your inbox to confirm."),
        );
        welcome.set(true);
        form.reset();
    };

    rsx! {
        section { class: "site-section", id: "signup", "aria-labelledby": "signup-title",
            h2 { id: "signup-title", "Join the waitlist" }
            p { class: "site-lede", "We let people in every week." }
            div { class: "signup-wrap",
                Form { state: form, on_valid: joined, id: "signup-form",
                    TextField { field: name, label: "Name" }
                    TextField {
                        field: email,
                        label: "Work email",
                        r#type: "email",
                        placeholder: "you@company.com",
                        description: "We only email about your invite.",
                    }
                    Button { r#type: "submit", "Join waitlist" }
                }
                div {
                    h3 { style: "margin: 0 0 0.75rem; font-size: 1rem;", "Just joined" }
                    p { id: "empty-note", style: "margin: 0; color: var(--secondary-color-5); font-size: 0.9rem;",
                        if people.read().is_empty() { "Be the first." }
                    }
                    FlipList { id: "signup-list", class: "signup-list",
                        AnimatePresence {
                            for (id, who) in people() {
                                FlipItem { key: "{id}",
                                    Pop { class: "signup-row", preset: SpringPreset::Snappy, "{who}" }
                                }
                            }
                        }
                    }
                }
            }
        }
        Dialog { open: welcome(), on_open_change: move |v| welcome.set(v),
            title: "You are on the list",
            description: "Thanks for joining. We will email you when your invite is ready.",
            div { style: "display: flex; justify-content: flex-end; margin-top: 1rem;",
                Button { id: "welcome-close", onclick: move |_| welcome.set(false), "Great" }
            }
        }
    }
}

#[component]
fn Palette(open: bool, on_open_change: EventHandler<bool>) -> Element {
    let items = vec![
        CommandItem::new("features", "Go to features").group("Navigate"),
        CommandItem::new("pricing", "Go to pricing").group("Navigate"),
        CommandItem::new("faq", "Go to FAQ").group("Navigate"),
        CommandItem::new("signup", "Join the waitlist").group("Navigate"),
    ];
    rsx! {
        CommandDialog {
            open,
            on_open_change: move |v| on_open_change.call(v),
            items,
            on_select: move |id: String| {
                on_open_change.call(false);
                #[cfg(target_arch = "wasm32")]
                if let Some(win) = web_sys::window() {
                    let _ = win.location().set_hash(&id);
                }
                #[cfg(not(target_arch = "wasm32"))]
                let _ = id;
            },
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "site-footer",
            span { "Orbit is a pretend product. This page is a test of the Atoll kit." }
            Breadcrumb {
                    BreadcrumbItem { BreadcrumbLink { href: "#top", "Home" } }
                    BreadcrumbSeparator {}
                    BreadcrumbItem { BreadcrumbPage { "Landing" } }
            }
        }
    }
}
