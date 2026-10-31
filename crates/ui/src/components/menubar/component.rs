use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    self, MenubarContentProps, MenubarItemProps, MenubarMenuProps, MenubarProps,
    MenubarTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
#[css_module("/src/components/menubar/style.css")]
struct Styles;

/// Moves focus from the focused element to `selector` inside its nearest ancestor with
/// class `container`: back to the trigger when the keyboard closes the menu, as the APG
/// pattern asks. `only_single` skips it inside a multi-select list, which stays open.
///
/// Web only. A `MountedData` handle would also work on native, but getting one means an
/// `onmounted` on the trigger, and an element keeps one listener per event name, so it would
/// replace the primitive's own (which focuses the trigger on click in Safari and Firefox).
/// For the same reason nothing here adds a listener the primitive already sets on that
/// element. On native focus stays on the removed item; the fuzz test lists that gap.
fn refocus_within(container: &str, selector: &str, only_single: bool) {
    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    {
        use wasm_bindgen::JsCast;
        let active = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element());
        let multi = active
            .as_ref()
            .and_then(|el| el.closest("[role=listbox]").ok().flatten())
            .and_then(|list| list.get_attribute("aria-multiselectable"))
            .is_some_and(|v| v == "true");
        if only_single && multi {
            return;
        }
        let target = active
            .and_then(|el| el.closest(&format!(".{container}")).ok().flatten())
            .and_then(|root| root.query_selector(selector).ok().flatten())
            .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());
        // Focus after the primitive has handled the key and closed the menu.
        if let Some(el) = target {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = el.focus();
            });
        }
    }
    #[cfg(not(all(target_arch = "wasm32", feature = "web")))]
    let _ = (container, selector, only_single);
}

fn refocus_menubar_trigger() {
    let trigger = format!(".{}", Styles::dx_menubar_trigger);
    refocus_within(&Styles::dx_menubar_menu.to_string(), &trigger, false);
}

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    rsx! {
        menubar::Menubar {
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_menubar }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    rsx! {
        menubar::MenubarMenu {
            index: props.index,
            disabled: props.disabled,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_menubar_menu }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_menubar_trigger,
    });
    rsx! {
        menubar::MenubarTrigger { attributes: merge_attributes(vec![base, props.attributes]), {props.children} }
    }
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    rsx! {
        menubar::MenubarContent {
            id: props.id,
            attributes: merge_attributes(vec![attributes!(div {
                class: Styles::dx_menubar_content,
                // Escape closes the menu but leaves focus on the removed item; APG returns
                // it to the menubar item that opened the menu.
                onkeydown: move |e: KeyboardEvent| {
                    if e.key() == Key::Escape {
                        refocus_menubar_trigger();
                    }
                },
            }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    rsx! {
        menubar::MenubarItem {
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            // Choosing an item closes the menu. The item's own `onkeydown` picks it and stops
            // the key; another `onkeydown` here would replace it, so refocus from `on_select`.
            on_select: move |value: String| {
                refocus_menubar_trigger();
                props.on_select.call(value);
            },
            attributes: merge_attributes(vec![attributes!(div {
                class: Styles::dx_menubar_item,
            }), props.attributes]),
            {props.children}
        }
    }
}
