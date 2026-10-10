use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 28rem;",
            Alert {
                AlertTitle { "Heads up" }
                AlertDescription { "You can add components to your app with the registry." }
            }
            Alert { variant: AlertVariant::Destructive,
                AlertTitle { "Something went wrong" }
                AlertDescription { "Your session has expired. Sign in again to continue." }
            }
        }
    }
}
