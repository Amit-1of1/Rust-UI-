use crate::components::accordion::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let items = [
        (
            "Is it accessible?",
            "Yes. Behaviour comes from dioxus-primitives, which follows WAI-ARIA.",
        ),
        (
            "Is it styled?",
            "Yes. It ships with a neutral, shadcn-like theme you own.",
        ),
        (
            "Is it animated?",
            "Open and close are animated in CSS; springs arrive in week 2.",
        ),
    ];
    rsx! {
        div { style: "max-width: 32rem;",
            Accordion { allow_multiple_open: false, horizontal: false,
                for (i, (q, a)) in items.into_iter().enumerate() {
                    AccordionItem { index: i,
                        AccordionTrigger { {q} }
                        AccordionContent {
                            p { style: "margin: 0; padding-bottom: 1rem;", {a} }
                        }
                    }
                }
            }
        }
    }
}
