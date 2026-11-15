use crate::components::tabs::*;
use crate::components::icons::{Bell, CreditCard, User};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1.5rem;",
        Tabs { default_value: "account".to_string(), horizontal: true, max_width: "24rem",
            TabList {
                TabTrigger { value: "account".to_string(), index: 0usize, "Account" }
                TabTrigger { value: "password".to_string(), index: 1usize, "Password" }
            }
            TabContent { index: 0usize, value: "account".to_string(),
                p { "Make changes to your account here." }
            }
            TabContent { index: 1usize, value: "password".to_string(),
                p { "Change your password here." }
            }
        }
        // Equal-width tabs with icons, and one you can't open yet.
        Tabs { default_value: "profile".to_string(), horizontal: true, stretch: true, variant: TabsVariant::Ghost, max_width: "24rem",
            TabList {
                TabTrigger { value: "profile".to_string(), index: 0usize, User { "aria-hidden": "true", size: "1rem" } "Profile" }
                TabTrigger { value: "alerts".to_string(), index: 1usize, Bell { "aria-hidden": "true", size: "1rem" } "Alerts" }
                TabTrigger { value: "billing".to_string(), index: 2usize, disabled: true, CreditCard { "aria-hidden": "true", size: "1rem" } "Billing" }
            }
            TabContent { index: 0usize, value: "profile".to_string(), p { "Your name and photo." } }
            TabContent { index: 1usize, value: "alerts".to_string(), p { "What we email you about." } }
            TabContent { index: 2usize, value: "billing".to_string(), p { "Plans and invoices." } }
        }
        }
    }
}
