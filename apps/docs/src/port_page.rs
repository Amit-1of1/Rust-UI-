//! "From shadcn" (`#port`): paste shadcn/ui JSX and get Atoll `rsx!` back, live, with the
//! list of what is left to finish by hand.
//!
//! It runs the same code as `atoll port` (the CLI's `port.rs`, included below), so the page
//! and the command can't disagree. Built from kit components only.

// The CLI's translator, shared as source so the docs site doesn't depend on the CLI crate.
// `report` is for the terminal; this page lays the same facts out itself.
#[allow(dead_code)]
#[path = "../../../crates/cli/src/port.rs"]
mod port;

use dioxus::prelude::*;
use ui::hooks::use_copy_clipboard;
use ui::prelude::*;

/// What the box starts with: a typical shadcn login card.
const EXAMPLE: &str = r#"<Card className="w-full max-w-sm">
  <CardHeader>
    <CardTitle className="text-2xl">Log in</CardTitle>
    <CardDescription>Enter your email to log in.</CardDescription>
  </CardHeader>
  <CardContent className="grid gap-4">
    <div className="grid gap-2">
      <Label htmlFor="email">Email</Label>
      <Input id="email" type="email" placeholder="m@example.com" required />
    </div>
  </CardContent>
  <CardFooter className="flex justify-between">
    <Button variant="outline" onClick={() => router.back()}>Cancel</Button>
    <Button><Mail className="h-4 w-4" /> Sign in</Button>
  </CardFooter>
</Card>"#;

#[component]
pub fn PortPage() -> Element {
    let mut source = use_signal(|| EXAMPLE.to_string());
    let (copy, copied) = use_copy_clipboard();
    // Cheap (a few hundred microseconds for a page of JSX), so it runs on every keystroke.
    let p = port::port(&source());
    let code = p.code.clone();
    let to_copy = code.clone();
    let add: Vec<String> = p.add.iter().cloned().collect();

    rsx! {
        document::Title { "From shadcn - Atoll" }
        h1 { id: "page-title", tabindex: "-1", "From shadcn" }
        p { class: "docs-lede",
            "Paste a shadcn/ui component and get the Atoll version as you type. Components and their variants, Lucide icons and Tailwind classes carry over; handlers, lists and hooks are listed for you to finish. The same thing runs on your machine as "
            code { "atoll port file.tsx" }
            "."
        }
        div { class: "port-grid",
            section { "aria-labelledby": "port-in",
                h2 { id: "port-in", "shadcn (JSX)" }
                Label { html_for: "port-source", class: "sr-only", "JSX to port" }
                Textarea {
                    id: "port-source",
                    class: "port-code",
                    rows: 18,
                    "spellcheck": "false",
                    value: source,
                    oninput: move |e: FormEvent| source.set(e.value()),
                }
            }
            section { "aria-labelledby": "port-out",
                h2 { id: "port-out", "Atoll (Rust)" }
                pre { class: "port-code", tabindex: "0", "aria-label": "Ported Rust code",
                    code { "{code}" }
                }
                div { class: "tb-row",
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| copy.call(to_copy.clone()),
                        if copied() { "Copied" } else { "Copy Rust" }
                    }
                    span { role: "status", class: "tb-help",
                        if copied() { "The code is on your clipboard." }
                    }
                }
            }
        }
        section { "aria-labelledby": "port-report",
            h2 { id: "port-report", "What to do next" }
            if p.trees == 0 {
                p { "No JSX found yet: paste a component's markup (from " code { "return (" } " to the closing tag)." }
            } else {
                if !add.is_empty() {
                    p {
                        "Copy the components into your app: "
                        code { "atoll add {add.join(\" \")}" }
                    }
                }
                if p.todo.is_empty() && p.unmapped_classes.is_empty() {
                    p { "Nothing left to finish by hand." }
                } else {
                    ul { class: "port-todo",
                        for (i, t) in p.todo.iter().enumerate() {
                            li { key: "{i}", "{t}" }
                        }
                        if !p.unmapped_classes.is_empty() {
                            li {
                                "Classes with no token mapping: "
                                code { {p.unmapped_classes.iter().cloned().collect::<Vec<_>>().join(" ")} }
                            }
                        }
                    }
                }
            }
        }
    }
}
