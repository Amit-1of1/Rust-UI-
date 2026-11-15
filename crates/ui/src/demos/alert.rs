use crate::components::alert::*;
use crate::components::button::*;
use crate::components::icons::{CircleAlert, CircleCheck, Info, TriangleAlert};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 28rem;",
            Alert { live: AlertLive::Off,
                AlertTitle { "Heads up" }
                AlertDescription { "You can add components to your app with the registry." }
            }
            Alert { variant: AlertVariant::Destructive,
                icon: rsx! { CircleAlert {} },
                AlertTitle { "Something went wrong" }
                AlertDescription { "Your session has expired. Sign in again to continue." }
                AlertActions { Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Sign in" } }
            }
            Alert { variant: AlertVariant::Success, live: AlertLive::Polite,
                icon: rsx! { CircleCheck {} },
                AlertTitle { "Invoice sent" }
                AlertDescription { "Ledgerly emailed the invoice to your client." }
            }
            Alert { variant: AlertVariant::Warning, live: AlertLive::Off,
                icon: rsx! { TriangleAlert {} },
                AlertTitle { "Trial ends in 3 days" }
            }
            Alert { variant: AlertVariant::Info, live: AlertLive::Off,
                icon: rsx! { Info {} },
                AlertTitle { "Maintenance on Sunday" }
                AlertDescription { "Sync pauses from 02:00 to 03:00 UTC." }
            }
        }
    }
}
