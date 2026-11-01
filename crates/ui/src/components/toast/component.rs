use dioxus::prelude::*;
use crate::components::icons::{CircleAlert, CircleCheck, Info, TriangleAlert};
use dioxus_primitives::toast::{
    self, Toast, ToastCloseButtonProps, ToastContentProps, ToastDescriptionProps, ToastProps,
    ToastTitleProps, ToastType,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::time::Duration;

#[css_module("/src/components/toast/style.css")]
struct Styles;

/// Actions waiting for their toast, by title: the primitive's toast record holds only text,
/// so [`ToastsExt::notify_action`] parks the button here and the toast picks it up.
#[derive(Clone, Copy)]
struct ToastActions(Signal<Vec<(String, ToastAction)>>);

/// A button in a toast that answers it, such as "Undo" after a delete.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, PartialEq)]
pub struct ToastAction {
    /// The button's text ("Undo").
    pub label: String,
    /// Runs when the button is pressed; the toast then closes.
    pub on_action: Callback<()>,
}

#[component]
fn StyledToast(props: ToastProps) -> Element {
    let actions = try_use_context::<ToastActions>();
    let title = props.title.clone();
    let action = actions.and_then(|a| {
        a.0.read()
            .iter()
            .find(|(t, _)| *t == title)
            .map(|(_, action)| action.clone())
    });
    // Forget the action with its toast.
    let gone = props.title.clone();
    use_drop(move || {
        if let Some(mut a) = actions {
            a.0.write().retain(|(t, _)| *t != gone);
        }
    });
    let on_close = props.on_close;
    let icon = match props.toast_type {
        ToastType::Success => rsx! { CircleCheck { "aria-hidden": "true", size: "1rem" } },
        ToastType::Error => rsx! { CircleAlert { "aria-hidden": "true", size: "1rem" } },
        ToastType::Warning => rsx! { TriangleAlert { "aria-hidden": "true", size: "1rem" } },
        ToastType::Info => rsx! { Info { "aria-hidden": "true", size: "1rem" } },
    };
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
            span { class: Styles::dx_toast_icon, {icon} }
            ToastContent {
                ToastTitle {}
                ToastDescription {}
            }
            if let Some(action) = action {
                button {
                    r#type: "button",
                    class: Styles::dx_toast_action,
                    onclick: move |e: MouseEvent| {
                        action.on_action.call(());
                        on_close.call(e);
                    },
                    {action.label.clone()}
                }
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
    use_context_provider(|| ToastActions(Signal::new(Vec::new())));
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

/// Shorter toasts for the common case: `toasts.notify_success("Saved")`, or with a description
/// `toasts.notify_error(("Failed", "Try again."))`. The primitive's `success(String, ToastOptions)`
/// still works for durations and other options.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub trait ToastsExt {
    /// A toast with a button that answers it ("Undo", "Retry"). Pressing the button runs
    /// `on_action` and closes the toast. Give actions a generous time to be noticed: the
    /// toast's timer pauses while it is hovered or focused, and F6 moves focus to toasts.
    /// Must be called below the kit's [`ToastProvider`].
    fn notify_action(
        &self,
        kind: ToastType,
        message: impl Into<ToastMessage>,
        label: impl Into<String>,
        on_action: impl FnMut(()) + 'static,
    );
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
    fn notify_action(
        &self,
        kind: ToastType,
        message: impl Into<ToastMessage>,
        label: impl Into<String>,
        on_action: impl FnMut(()) + 'static,
    ) {
        let message = message.into();
        if let Some(mut actions) = try_consume_context::<ToastActions>() {
            let title = message.title.clone();
            actions.0.write().retain(|(t, _)| *t != title);
            actions.0.write().push((
                title,
                ToastAction {
                    label: label.into(),
                    on_action: Callback::new(on_action),
                },
            ));
        }
        let (title, options) = message.split();
        self.show(title, kind, options);
    }

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
