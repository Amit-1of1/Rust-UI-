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
    #[props(default)] render_toast: Option<Callback<toast::ToastProps, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let render_toast = render_toast.unwrap_or_else(|| {
        Callback::new(|p: toast::ToastProps| rsx! { StyledToast { ..p } })
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

/// Shorter toasts for the common case: `toasts.notify_success("Saved")`, or with a description
/// `toasts.notify_error(("Failed", "Try again."))`. The primitive's `success(String, ToastOptions)`
/// still works for durations and other options.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub trait ToastsExt {
    fn notify_success(&self, message: impl Into<ToastMessage>);
    fn notify_error(&self, message: impl Into<ToastMessage>);
    fn notify_warning(&self, message: impl Into<ToastMessage>);
    fn notify_info(&self, message: impl Into<ToastMessage>);
}

/// A toast's title and optional description. Built from `"title"`, a `String`, or a
/// `(title, description)` pair.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, PartialEq, Debug)]
pub struct ToastMessage {
    pub title: String,
    pub description: Option<String>,
}

impl ToastMessage {
    fn split(self) -> (String, toast::ToastOptions) {
        let mut options = toast::ToastOptions::new();
        if let Some(description) = self.description {
            options = options.description(description);
        }
        (self.title, options)
    }
}

impl From<&str> for ToastMessage {
    fn from(title: &str) -> Self {
        Self {
            title: title.to_string(),
            description: None,
        }
    }
}

impl From<String> for ToastMessage {
    fn from(title: String) -> Self {
        Self {
            title,
            description: None,
        }
    }
}

impl<T: Into<String>, D: Into<String>> From<(T, D)> for ToastMessage {
    fn from((title, description): (T, D)) -> Self {
        Self {
            title: title.into(),
            description: Some(description.into()),
        }
    }
}

impl ToastsExt for toast::Toasts {
    fn notify_success(&self, message: impl Into<ToastMessage>) {
        let (title, options) = message.into().split();
        self.success(title, options);
    }

    fn notify_error(&self, message: impl Into<ToastMessage>) {
        let (title, options) = message.into().split();
        self.error(title, options);
    }

    fn notify_warning(&self, message: impl Into<ToastMessage>) {
        let (title, options) = message.into().split();
        self.warning(title, options);
    }

    fn notify_info(&self, message: impl Into<ToastMessage>) {
        let (title, options) = message.into().split();
        self.info(title, options);
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
