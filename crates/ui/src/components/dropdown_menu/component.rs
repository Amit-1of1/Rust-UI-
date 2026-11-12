use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::dropdown_menu::{
    self, DropdownMenuContentProps, DropdownMenuItemProps, DropdownMenuProps,
    DropdownMenuTriggerProps,
};
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/dropdown_menu/style.css")]
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

fn is_close_key(e: &KeyboardEvent) -> bool {
    matches!(e.key(), Key::Named(NamedKey::Escape) | Key::Named(NamedKey::Enter)) || e.key() == Key::Character(" ".into())
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_dropdown_menu,
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        dropdown_menu::DropdownMenu {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let base = attributes!(button {
        class: Styles::dx_dropdown_menu_trigger,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dropdown_menu::DropdownMenuTrigger { as: props.r#as, attributes: merged, {props.children} }
    }
}

#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_dropdown_menu_content,
        // The primitive closes on Escape, Enter and Space but leaves focus on the removed
        // item; the APG menu button pattern returns it to the trigger.
        onkeydown: move |e: KeyboardEvent| {
            if is_close_key(&e) {
                refocus_within(&Styles::dx_dropdown_menu.to_string(), "[aria-haspopup]", false);
            }
        },
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        dropdown_menu::DropdownMenuContent { id: props.id, attributes: merged, {props.children} }
    }
}

#[component]
pub fn DropdownMenuItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuItemProps<T>,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_dropdown_menu_item,
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);
    // Choosing an item closes the menu. The item's own `onkeydown` picks it and stops the
    // key, and another `onkeydown` here would replace that one, so focus goes back to the
    // trigger from `on_select` instead (it runs while the item still has focus).
    let user_select = props.on_select;
    let on_select = move |value: T| {
        refocus_within(&Styles::dx_dropdown_menu.to_string(), "[aria-haspopup]", false);
        user_select.call(value);
    };

    rsx! {
        dropdown_menu::DropdownMenuItem {
            disabled: props.disabled,
            value: props.value,
            index: props.index,
            on_select,
            attributes: merged,
            {props.children}
        }
    }
}
