use dioxus::prelude::*;
use dioxus_primitives::dialog::{self, DialogCtx, DialogDescriptionProps};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use motion::SwipeDismiss;
use crate::components::hooks::{use_return_focus, ReturnFocus};

#[css_module("/src/components/drawer/style.css")]
struct Styles;

/// The props for [`Drawer`].
///
/// `title` is required: a drawer is a modal dialog, and one without a name is announced as just
/// "dialog". Hide it with `hide_title` when the content makes the purpose clear to sighted users.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Drawer { open: true, title: "Rename file", p { "…" } } }
/// # }
/// ```
///
/// Leaving `title` out is a compile error ("Missing required field title"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Drawer { open: true, p { "…" } } }
/// # }
/// ```
#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    /// The drawer's heading, and its accessible name (`aria-labelledby`).
    #[props(into)]
    pub title: String,

    /// One line read after the title (`aria-describedby`), shown under it. For richer text, put
    /// a [`DrawerDescription`] in the children instead (not both).
    #[props(default, into)]
    pub description: Option<String>,

    /// Hide the title and `description` visually; screen readers still read them.
    #[props(default)]
    pub hide_title: bool,

    /// The ID of the dialog root element.
    pub id: ReadSignal<Option<String>>,

    /// Whether the drawer is modal. If true, it will trap focus within the drawer when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the drawer.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the drawer if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// Called when the open state changes, including after a drag dismisses it.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Attributes for the drawer panel.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The drawer's content, shown below the title.
    pub children: Element,
}

/// A panel that slides up from the bottom of the screen and can be dragged down to dismiss.
///
/// It is a modal dialog underneath (focus is trapped, Escape closes, the page behind is inert),
/// named by its required `title`. Dragging is an extra: the Close button and Escape always work.
#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    let on_open_change = props.on_open_change;
    let base = attributes!(div {
        class: Styles::dx_drawer,
    });
    let (inside, return_focus) = use_return_focus();
    let merged = merge_attributes(vec![base, return_focus, props.attributes]);
    let hide = props.hide_title;
    let title_class = with_sr_only(Styles::dx_drawer_title.to_string(), hide);
    let description_class = with_sr_only(Styles::dx_drawer_description.to_string(), hide);
    rsx! {
        dialog::DialogRoot {
            class: Styles::dx_drawer_backdrop,
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent { class: None, attributes: merged,
                SwipeDismiss {
                    vertical: true,
                    one_way: true,
                    width: 320.0,
                    on_dismiss: move |_| on_open_change.call(false),
                    class: Styles::dx_drawer_body,
                    div { class: Styles::dx_drawer_handle, "aria-hidden": "true" }
                    dialog::DialogTitle { attributes: attributes!(h2 { class: title_class }), {props.title} }
                    if let Some(description) = props.description {
                        dialog::DialogDescription { attributes: attributes!(p { class: description_class }), {description} }
                    }
                    {props.children}
                }
                ReturnFocus { inside }
            }
        }

    }
}

/// Adds the theme's `.sr-only` class (visually hidden, still read) when `hide` is set.
fn with_sr_only(class: String, hide: bool) -> String {
    if hide {
        format!("{class} sr-only")
    } else {
        class
    }
}

/// A longer description for a [`Drawer`] (links, formatting). For one line of text, use the
/// drawer's `description` prop instead.
#[component]
pub fn DrawerDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: Styles::dx_drawer_description,
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    rsx! {
        dialog::DialogDescription { id: props.id, attributes: merged, {props.children} }
    }
}

/// A button that closes the drawer.
#[component]
pub fn DrawerClose(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx: DialogCtx = use_context();
    rsx! {
        button {
            r#type: "button",
            onclick: move |_| ctx.set_open(false),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_drawer_close }), attributes]),
            {children}
        }
    }
}
