use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::tooltip::{self, TooltipContentProps, TooltipTriggerProps};

#[css_module("/src/components/tooltip/style.css")]
struct Styles;

/// Whether the tooltip is open, mirrored here because the primitive keeps it private.
/// The primitive's trigger always points `aria-describedby` at the content, but the
/// content only exists while open, so a closed trigger references a missing id. The
/// styled trigger drops the reference while closed.
#[derive(Clone, Copy)]
struct TooltipOpen(Memo<bool>);

/// The props for [`Tooltip`].
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Whether the tooltip is open (controlled). A controlled tooltip opens without delay.
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the tooltip is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// How long the pointer must rest on the trigger before the tooltip opens, in
    /// milliseconds, so tooltips don't flash open as the cursor crosses the screen. Keyboard
    /// focus opens it at once. Defaults to 400.
    #[props(default = 400)]
    pub delay_ms: u64,

    /// Additional attributes for the tooltip.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// A [`TooltipTrigger`] and a [`TooltipContent`].
    pub children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let mut uncontrolled = use_signal(|| props.default_open);
    // Bumped on every open or close request, so a pending delayed open can tell it is stale.
    let mut request = use_signal(|| 0u64);
    let controlled = props.open;
    let open = use_memo(move || controlled().unwrap_or(uncontrolled()));
    use_context_provider(|| TooltipOpen(open));
    let on_open_change = props.on_open_change;
    let delay = props.delay_ms;
    let mut set = move |value: bool| {
        if uncontrolled() != value {
            uncontrolled.set(value);
            on_open_change.call(value);
        }
    };
    let base = attributes!(div {
        class: Styles::dx_tooltip,
        // Focus (keyboard) opens at once; it bubbles here right after the trigger's own focus
        // handler asked for a delayed open.
        onfocusin: move |_| {
            if !(props.disabled)() {
                *request.write() += 1;
                set(true);
            }
        },
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        tooltip::Tooltip {
            disabled: props.disabled,
            open: Some(open()),
            default_open: props.default_open,
            on_open_change: move |value: bool| {
                *request.write() += 1;
                if controlled().is_some() {
                    on_open_change.call(value);
                } else if !value || delay == 0 {
                    set(value);
                } else {
                    let ticket = *request.peek();
                    spawn(async move {
                        dioxus_sdk_time::sleep(std::time::Duration::from_millis(delay)).await;
                        if *request.peek() == ticket {
                            set(true);
                        }
                    });
                }
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
