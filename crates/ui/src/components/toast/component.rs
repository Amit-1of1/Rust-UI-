use dioxus::prelude::*;
use dioxus_primitives::toast::{
    self, Toast, ToastCloseButtonProps, ToastContentProps, ToastDescriptionProps, ToastProps,
    ToastTitleProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::time::Duration;

#[css_module("/src/components/toast/style.css")]
struct Styles;

#[component]
fn StyledToast(props: ToastProps) -> Element {
    rsx! {
        Toast {
            id: props.id,
            index: props.index,
            title: props.title,
            description: props.description,
            toast_type: props.toast_type,
            on_close: props.on_close,
            permanent: props.permanent,
            duration: props.duration,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_toast }), props.attributes]),
            ToastContent {
                ToastTitle {}
                ToastDescription {}
            }
            ToastCloseButton {}
        }
    }
}

#[component]
fn ToastContent(props: ToastContentProps) -> Element {
    rsx! {
        toast::ToastContent {
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_toast_content }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
fn ToastTitle(props: ToastTitleProps) -> Element {
    rsx! {
        toast::ToastTitle {
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_toast_title }), props.attributes]),
            children: props.children,
        }
    }
}

#[component]
fn ToastDescription(props: ToastDescriptionProps) -> Element {
    rsx! {
        toast::ToastDescription {
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_toast_description }), props.attributes]),
            children: props.children,
        }
    }
}

#[component]
fn ToastCloseButton(props: ToastCloseButtonProps) -> Element {
    rsx! {
        toast::ToastCloseButton {
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_toast_close }), props.attributes]),
            children: props.children,
        }
    }
}

#[component]
pub fn ToastProvider(
    #[props(default = ReadSignal::new(Signal::new(Some(Duration::from_secs(5)))))]
    default_duration: ReadSignal<Option<Duration>>,
    #[props(default = ReadSignal::new(Signal::new(10)))] max_toasts: ReadSignal<usize>,
    #[props(default)] render_toast: Option<Callback<toast::ToastPropsWithOwner, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let render_toast = render_toast.unwrap_or_else(|| {
        Callback::new(|p: toast::ToastPropsWithOwner| rsx! { StyledToast { ..p } })
    });

    rsx! {
        toast::ToastProvider {
            default_duration,
            max_toasts,
            render_toast,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_toast_container }), attributes]),
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_primitives::toast::{use_toast, ToastOptions};

    #[component]
    fn TriggerToast() -> Element {
        let toast_api = use_toast();
        use_hook(move || {
            toast_api.success(
                "Saved".to_string(),
                ToastOptions::new()
                    .description("Everything synced")
                    .permanent(true),
            );
        });

        rsx! {}
    }

    #[test]
    fn styled_toast_preserves_primitive_fallback_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                ToastProvider {
                    TriggerToast {}
                }
            }
        });
        dom.rebuild_in_place();
        dom.mark_all_dirty();
        dom.render_immediate_to_vec();
        let html = dioxus_ssr::render(&dom);

        assert!(html.contains("Saved"));
        assert!(html.contains("Everything synced"));
        assert!(html.contains('\u{00d7}') || html.contains("&#215;") || html.contains("&times;"));
    }
}
