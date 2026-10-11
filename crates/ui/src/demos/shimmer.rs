use crate::components::shimmer::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut loading = use_signal(|| true);
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 24rem;",
            Shimmer { loading: loading(),
                div { style: "padding: 1rem;",
                    strong { "Quarterly report" }
                    p { style: "margin: 0.25rem 0 0;", "Revenue is up 12% on the previous quarter." }
                }
            }
            label {
                input {
                    r#type: "checkbox",
                    checked: loading(),
                    onchange: move |e| loading.set(e.checked()),
                }
                " Loading"
            }
        }
    }
}
