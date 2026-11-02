use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastType};
use ui::prelude::*;

#[component]
fn Trigger() -> Element {
    let toasts = use_toast();
    use_hook(move || {
        toasts.notify_action(ToastType::Info, ("Invoice deleted", "INV-1"), "Undo", |_| {});
        toasts.notify_error("Sync failed");
    });
    rsx! {}
}

fn render_toasts() -> String {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            // No timeout: the test has no timer runtime.
            ToastProvider { default_duration: None, Trigger {} }
        }
    });
    dom.rebuild_in_place();
    dom.mark_all_dirty();
    dom.render_immediate_to_vec();
    dioxus_ssr::render(&dom)
}

#[test]
fn toasts_show_an_icon_for_their_type_and_their_action() {
    let html = render_toasts();
    assert!(html.contains("data-icon=\"circle-alert\""), "{html}");
    assert!(html.contains("data-icon=\"info\""), "{html}");
    assert!(html.contains(">Undo"), "{html}");
    assert_eq!(html.matches(">Undo").count(), 1, "only the toast with the action: {html}");
}
