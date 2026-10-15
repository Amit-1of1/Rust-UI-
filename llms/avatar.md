# Avatar

- Copy into your app: `rust-ui add avatar` or `dx components add avatar --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::avatar::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Avatar components display a user's profile picture or fallback initials. Use the composable `Avatar`, `AvatarImage`, and `AvatarFallback` primitives when you need full control, or `ImageAvatar` for the common image-with-fallback case.

## Component Structure

```rust
Avatar {
    aria_label: "Jane Doe",
    AvatarImage {
        src: "https://example.com/avatar.png",
        alt: "Jane Doe",
    }
    AvatarFallback { "JD" }
}
```

```rust
ImageAvatar {
    src: "https://example.com/avatar.png",
    alt: "Jane Doe",
    on_state_change: |state: AvatarState| { /* image is loading/loaded/failed */ },
    "JD"
}
```

## API

```rust
pub enum AvatarImageSize {
    #[default]
    Small,
    Medium,
    Large,
}

pub enum AvatarShape {
    #[default]
    Circle,
    Rounded,
}

/// The props for the [`Avatar`] root component.
pub struct AvatarProps {
    /// Callback when image loads successfully.
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /// Callback when image fails to load.
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    /// Callback when the avatar state changes.
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    #[props(default)]
    pub size: AvatarImageSize,

    #[props(default)]
    pub shape: AvatarShape,

    /// Additional attributes for the avatar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The fallback content shown while the image is loading or if it fails to load.
    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element

pub struct AvatarImageProps {
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    pub src: String,

    #[props(default)]
    pub alt: String,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element

pub struct AvatarFallbackProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element

/// The props for the [`ImageAvatar`] convenience component.
pub struct ImageAvatarProps {
    /// The image source URL.
    pub src: String,

    /// The image alt text.
    #[props(default)]
    pub alt: String,

    /// Callback when image loads successfully.
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /// Callback when image fails to load.
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    /// Callback when the avatar state changes.
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    #[props(default)]
    pub size: AvatarImageSize,

    #[props(default)]
    pub shape: AvatarShape,

    /// Additional attributes for the avatar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The fallback content shown while the image is loading or if it fails to load.
    pub children: Element,
}

#[component]
pub fn ImageAvatar(props: ImageAvatarProps) -> Element
```

## Example

```rust
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
        }
    }
}
```
