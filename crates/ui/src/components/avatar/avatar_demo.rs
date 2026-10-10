use super::component::*;
use dioxus::prelude::*;

#[css_module("/src/components/avatar/style.css")]
struct Styles;

#[component]
pub fn Demo() -> Element {
    let mut avatar_state = use_signal(|| "No state yet".to_string());
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            justify_content: "center",
            flex_wrap: "wrap",
            gap: "1rem",
            div { class: Styles::dx_avatar_item,
                p { class: Styles::dx_avatar_label, "Basic Usage" }
                ImageAvatar {
                    size: AvatarImageSize::Small,
                    src: crate::components::DEMO_AVATAR_SRC,
                    alt: "User avatar",
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 1: {state:?}"));
                    },
                    aria_label: "Basic avatar",
                    "EA"
                }
            }
            div { class: Styles::dx_avatar_item,
                p { class: Styles::dx_avatar_label, "Rounded" }
                ImageAvatar {
                    size: AvatarImageSize::Small,
                    shape: AvatarShape::Rounded,
                    src: crate::components::DEMO_AVATAR_SRC,
                    alt: "User avatar",
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 2: {state:?}"));
                    },
                    aria_label: "Basic avatar",
                    "EA"
                }
            }
            div { class: Styles::dx_avatar_item,
                p { class: Styles::dx_avatar_label, "Error State" }
                ImageAvatar {
                    size: AvatarImageSize::Medium,
                    src: "/does-not-exist.png",
                    alt: "Invalid image",
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 3: {state:?}"));
                    },
                    aria_label: "Error avatar",
                    "JK"
                }
            }
            div { class: Styles::dx_avatar_item,
                p { class: Styles::dx_avatar_label, "Large Size" }
                ImageAvatar {
                    size: AvatarImageSize::Large,
                    src: crate::components::DEMO_AVATAR_SRC.to_string(),
                    alt: "Large avatar",
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 4: {state:?}"));
                    },
                    aria_label: "Large avatar",
                    "DX"
                }
            }
        }
    }
}
