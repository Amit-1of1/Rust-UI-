use crate::components::avatar::*;
use dioxus::prelude::*;

/// A neutral placeholder image (an inline SVG) for avatar demos, so demos never hit the network.
pub const DEMO_AVATAR_SRC: &str = concat!(
    "data:image/svg+xml;utf8,",
    "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 64 64'>",
    "<rect width='64' height='64' fill='%23888'/>",
    "<circle cx='32' cy='25' r='11' fill='%23fff'/>",
    "<path d='M10 58c3-15 13-21 22-21s19 6 22 21z' fill='%23fff'/></svg>"
);

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
                    src: DEMO_AVATAR_SRC,
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
                    src: DEMO_AVATAR_SRC,
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
                    src: DEMO_AVATAR_SRC.to_string(),
                    alt: "Large avatar",
                    on_state_change: move |state| {
                        avatar_state.set(format!("Avatar 4: {state:?}"));
                    },
                    aria_label: "Large avatar",
                    "DX"
                }
            }
            div { class: Styles::dx_avatar_item,
                p { class: Styles::dx_avatar_label, "Group" }
                AvatarGroup { label: "Design team", more: 3,
                    Avatar { size: AvatarImageSize::Small, label: "Ada Lovelace", AvatarFallback { "AL" } }
                    Avatar { size: AvatarImageSize::Small, label: "Grace Hopper", AvatarFallback { "GH" } }
                    ImageAvatar { size: AvatarImageSize::Small, src: DEMO_AVATAR_SRC, alt: "Alan Turing", "AT" }
                }
            }
        }
    }
}
