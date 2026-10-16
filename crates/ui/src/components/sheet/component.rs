use crate::components::icons::X;
use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{self, DialogCtx, DialogDescriptionProps};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/sheet/style.css")]
struct Styles;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetSide::Top => "top",
            SheetSide::Right => "right",
            SheetSide::Bottom => "bottom",
            SheetSide::Left => "left",
        }
    }
}

/// The props for [`Sheet`].
///
/// `title` is required: a sheet is a dialog, and one without a name is announced as just
/// "dialog". Hide it with `hide_title` when the content makes the purpose clear to sighted users.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Sheet { open: true, title: "Rename file", p { "…" } } }
/// # }
/// ```
///
/// Leaving `title` out is a compile error ("Missing required field title"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Sheet { open: true, p { "…" } } }
/// # }
/// ```
#[derive(Props, Clone, PartialEq)]
pub struct SheetProps {
    /// The sheet's heading, and its accessible name (`aria-labelledby`). Shown in the header.
    #[props(into)]
    pub title: String,

    /// One line read after the title (`aria-describedby`), shown under it. For richer text, put
    /// a [`SheetDescription`] in the children instead (not both).
    #[props(default, into)]
    pub description: Option<String>,

    /// Hide the header (title and `description`) visually; screen readers still read it.
    #[props(default)]
    pub hide_title: bool,

    /// The ID of the dialog root element.
    pub id: ReadSignal<Option<String>>,

    /// Whether the sheet is modal. If true, it will trap focus within the sheet when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the sheet.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the sheet if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// Called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Attributes for the sheet panel (`"data-side"` picks the edge).
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The sheet's content, shown below the header.
    pub children: Element,
}

#[component]
pub fn Sheet(props: SheetProps) -> Element {
    let content_base = attributes!(div {
        class: Styles::dx_sheet,
        "data-slot": "sheet-content",
        "data-side": SheetSide::Right.as_str(),
    });
    let content_attributes = merge_attributes(vec![content_base, props.attributes]);
    let header_class = if props.hide_title {
        format!("{} sr-only", Styles::dx_sheet_header)
    } else {
        Styles::dx_sheet_header.to_string()
    };

    rsx! {
        dialog::DialogRoot {
            class: Styles::dx_sheet_root,
            "data-slot": "sheet-root",
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent {
                class: None,
                attributes: content_attributes,
                div { class: header_class, "data-slot": "sheet-header",
                    dialog::DialogTitle {
                        "data-slot": "sheet-title",
                        attributes: attributes!(div { class: Styles::dx_sheet_title }),
                        {props.title}
                    }
                    if let Some(description) = props.description {
                        dialog::DialogDescription {
                            "data-slot": "sheet-description",
                            attributes: attributes!(div { class: Styles::dx_sheet_description }),
                            {description}
                        }
                    }
                }
                {props.children}
            }
        }

    }
}

#[component]
pub fn SheetContentClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    // Icon-only, so it needs a name of its own.
    let close = use_ui_strings().close;
    let base = attributes!(button {
        class: Styles::dx_sheet_close,
        aria_label: close,
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        SheetClose { attributes,
            X { size: "20px", "aria-hidden": "true" }
        }
    }
}

#[component]
pub fn SheetHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { "data-slot": "sheet-header", ..merge_attributes(vec![attributes!(div { class: Styles::dx_sheet_header }), attributes]), {children} }
    }
}

#[component]
pub fn SheetFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { "data-slot": "sheet-footer", ..merge_attributes(vec![attributes!(div { class: Styles::dx_sheet_footer }), attributes]), {children} }
    }
}

/// A longer description for a [`Sheet`] (links, formatting). For one line of text, use the
/// sheet's `description` prop instead.
#[component]
pub fn SheetDescription(props: DialogDescriptionProps) -> Element {
    rsx! {
        dialog::DialogDescription {
            id: props.id,
            "data-slot": "sheet-description",
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_sheet_description }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn SheetClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let ctx: DialogCtx = use_context();

    let base = attributes! {
        button {
            onclick: move |_| {
                ctx.set_open(false);
            }
        }
    };
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged, {children} }
        }
    }
}
