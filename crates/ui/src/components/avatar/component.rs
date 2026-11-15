use dioxus::prelude::*;
use dioxus_primitives::avatar::{self, AvatarState};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/avatar/style.css")]
struct Styles;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarImageSize {
    #[default]
    Small,
    Medium,
    Large,
}

impl AvatarImageSize {
    fn to_class(self) -> &'static str {
        match self {
            AvatarImageSize::Small => Styles::dx_avatar_sm.inner,
            AvatarImageSize::Medium => Styles::dx_avatar_md.inner,
            AvatarImageSize::Large => Styles::dx_avatar_lg.inner,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Rounded,
}

impl AvatarShape {
    fn to_class(self) -> &'static str {
        match self {
            AvatarShape::Circle => Styles::dx_avatar_circle.inner,
            AvatarShape::Rounded => Styles::dx_avatar_rounded.inner,
        }
    }
}

/// The props for the [`Avatar`] root component.
#[derive(Props, Clone, PartialEq)]
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

    /// The person or thing shown, read by screen readers ("Ada Lovelace"). The avatar is
    /// `role="img"`, so neither the image's `alt` nor the fallback initials name it. Without
    /// a `label` (or an `aria-label` / `aria-labelledby`), the avatar is decorative and
    /// hidden from assistive technology: use that when the name is shown beside it.
    #[props(default, into)]
    pub label: Option<String>,

    /// Additional attributes for the avatar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The fallback content shown while the image is loading or if it fails to load.
    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let class = format!(
        "{} {} {}",
        Styles::dx_avatar,
        props.size.to_class(),
        props.shape.to_class()
    );
    let named = props.label.is_some()
        || props
            .attributes
            .iter()
            .any(|a| matches!(a.name, "aria-label" | "aria-labelledby"));
    let base = attributes!(span {
        class,
        aria_label: props.label.clone(),
        aria_hidden: (!named).then_some("true"),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        avatar::Avatar {
            on_load: props.on_load,
            on_error: props.on_error,
            on_state_change: props.on_state_change,
            attributes: merged,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    pub src: String,

    /// The image's text alternative. Required, so leaving it out is a compile error; pass
    /// `alt: ""` on purpose when the image adds nothing (the avatar's `label` names it, or the
    /// name is shown beside it).
    #[props(into)]
    pub alt: String,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let base = attributes!(img {
        class: Styles::dx_avatar_image,
        draggable: "false",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        avatar::AvatarImage {
            id: props.id,
            src: props.src,
            alt: props.alt,
            attributes: merged,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let base = attributes!(span {
        class: Styles::dx_avatar_fallback,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        avatar::AvatarFallback {
            attributes: merged,
            {props.children}
        }
    }
}

/// The props for the [`ImageAvatar`] convenience component.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { ImageAvatar { src: "/ada.png", alt: "Ada Lovelace", "AL" } }
/// # }
/// ```
///
/// Leaving `alt` out is a compile error ("Missing required field alt"), for
/// [`AvatarImage`] too:
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { ImageAvatar { src: "/ada.png", "AL" } }
/// # }
/// ```
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Avatar { AvatarImage { src: "/ada.png" } } }
/// # }
/// ```
#[derive(Props, Clone, PartialEq)]
pub struct ImageAvatarProps {
    /// The image source URL.
    pub src: String,

    /// The image's text alternative, which also names the avatar when there is no `label`.
    /// Required, so leaving it out is a compile error; pass `alt: ""` on purpose when the name
    /// is shown beside the avatar.
    #[props(into)]
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

    /// The person or thing shown, read by screen readers ("Ada Lovelace"). The avatar is
    /// `role="img"`, so neither the image's `alt` nor the fallback initials name it. Without
    /// a `label` (or an `aria-label` / `aria-labelledby`), the avatar is decorative and
    /// hidden from assistive technology: use that when the name is shown beside it.
    #[props(default, into)]
    pub label: Option<String>,

    /// Additional attributes for the avatar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The fallback content shown while the image is loading or if it fails to load.
    pub children: Element,
}

#[component]
pub fn ImageAvatar(props: ImageAvatarProps) -> Element {
    rsx! {
        Avatar {
            on_load: props.on_load,
            on_error: props.on_error,
            on_state_change: props.on_state_change,
            size: props.size,
            shape: props.shape,
            // The image's `alt` names the avatar unless a `label` is given.
            label: props.label.or_else(|| (!props.alt.is_empty()).then(|| props.alt.clone())),
            attributes: props.attributes,
            AvatarImage {
                src: props.src,
                alt: props.alt,
            }
            AvatarFallback {
                {props.children}
            }
        }
    }
}

/// Stacks avatars so they overlap, for "who is in this team" rows. Pass the avatars to show
/// as children and the number left out as `more`, which adds a "+N" avatar at the end.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! {
///     AvatarGroup { label: "Design team", more: 4,
///         Avatar { label: "Ada Lovelace", AvatarFallback { "AL" } }
///         Avatar { label: "Grace Hopper", AvatarFallback { "GH" } }
///     }
/// }
/// # }
/// ```
///
/// The group is a `role="group"` named by `label`; each avatar keeps its own name, and the
/// "+N" avatar is read as "N more".
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn AvatarGroup(
    /// What the group is ("Design team"), read by screen readers.
    #[props(into)]
    label: String,
    /// How many avatars were left out; shown as "+N" when above zero.
    #[props(default)]
    more: usize,
    /// The size of the "+N" avatar; match the avatars inside.
    #[props(default)]
    size: AvatarImageSize,
    #[props(default)] shape: AvatarShape,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let more_label = crate::components::ui_strings::fill(
        &crate::components::ui_strings::use_ui_strings().more_count,
        &[("count", &more.to_string())],
    );
    let more_class = format!(
        "{} {} {} {}",
        Styles::dx_avatar,
        Styles::dx_avatar_more,
        size.to_class(),
        shape.to_class()
    );
    rsx! {
        div {
            role: "group",
            aria_label: label,
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_avatar_group }), attributes]),
            {children}
            if more > 0 {
                span { class: more_class, role: "img", aria_label: more_label, "+{more}" }
            }
        }
    }
}
