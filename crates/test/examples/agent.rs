//! A screen an AI agent can drive: `cargo run -p atoll-test --example agent` speaks MCP on
//! stdio. Register it with an agent, e.g.
//! `claude mcp add atoll-demo -- cargo run -q -p atoll-test --example agent`,
//! then ask it to "sign up with ada@example.com and tell me what the page says".
//! In your own app, swap `demo` for your root component (docs/AGENTS.md).

use dioxus::prelude::*;
use ui::prelude::*;

fn demo() -> Element {
    let mut email = use_signal(String::new);
    let mut plan = use_signal(|| "Monthly".to_string());
    let mut sent = use_signal(|| false);
    rsx! {
        ThemeStyles { reset: true }
        main { style: "padding: 1.5rem; max-width: 28rem;",
            Card {
                CardHeader {
                    CardTitle { "Join the beta" }
                    CardDescription { "We'll email you an invite." }
                }
                CardContent {
                    Label { html_for: "email", "Email" }
                    Input { id: "email", placeholder: "you@example.com", value: email,
                        oninput: move |e: FormEvent| email.set(e.value()) }
                    Tabs { default_value: "Monthly".to_string(), on_value_change: move |v: String| plan.set(v),
                        TabList {
                            TabTrigger { value: "Monthly".to_string(), index: 0usize, "Monthly" }
                            TabTrigger { value: "Yearly".to_string(), index: 1usize, "Yearly" }
                        }
                    }
                }
                CardFooter {
                    Button { onclick: move |_| sent.set(true), "Sign up" }
                }
            }
            if sent() {
                p { role: "status", "Thanks, {email}! Plan: {plan}." }
            }
        }
    }
}

fn main() {
    atoll_test::agent::serve(atoll_test::Screen::builder(demo).size(800, 600));
}
