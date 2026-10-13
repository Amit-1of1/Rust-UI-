use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::tooltip::{self, TooltipContentProps, TooltipProps, TooltipTriggerProps};

#[css_module("/src/components/tooltip/style.css")]
struct Styles;

/// Whether the tooltip is open, mirrored here because the primitive keeps it private.
/// The primitive's trigger always points `aria-describedby` at the content, but the
/// content only exists while open, so a closed trigger references a missing id. The
/// styled trigger drops the reference while closed.
#[derive(Clone, Copy)]
struct TooltipOpen(Memo<bool>);

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tooltip,
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let mut uncontrolled = use_signal(|| props.default_open);
    let controlled = props.open;
    let open = use_memo(move || controlled().unwrap_or(uncontrolled()));
    use_context_provider(|| TooltipOpen(open));
    let on_open_change = props.on_open_change;

    rsx! {
        tooltip::Tooltip {
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: move |open: bool| {
                uncontrolled.set(open);
                on_open_change.call(open);
            },
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let open = try_use_context::<TooltipOpen>().is_none_or(|o| (o.0)());
    let base = if open {
        attributes!(button {
            class: Styles::dx_tooltip_trigger,
        })
    } else {
        attributes!(button {
            class: Styles::dx_tooltip_trigger,
            aria_describedby: None::<String>,
        })
    };
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tooltip::TooltipTrigger {
            id: props.id,
            as: props.r#as,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_tooltip_content,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tooltip::TooltipContent {
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: merged,
            {props.children}
        }
    }
}
