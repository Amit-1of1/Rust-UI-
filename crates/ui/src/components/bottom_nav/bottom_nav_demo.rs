use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut current = use_signal(|| 0_usize);
    rsx! {
        div { style: "max-width: 24rem; border: 1px solid var(--primary-color-6); border-radius: var(--radius); overflow: hidden;",
            BottomNav { label: "Demo tabs",
                for (index , name) in ["Home", "Search", "Inbox"].into_iter().enumerate() {
                    BottomNavItem {
                        label: name,
                        current: current() == index,
                        onclick: move |_| current.set(index),
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                circle { cx: "12", cy: "12", r: "8" }
                            }
                        },
                    }
                }
            }
        }
    }
}
