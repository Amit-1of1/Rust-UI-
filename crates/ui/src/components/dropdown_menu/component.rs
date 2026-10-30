use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::dropdown_menu::{
    self, DropdownMenuContentProps, DropdownMenuItemProps, DropdownMenuProps,
    DropdownMenuTriggerProps,
};
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/dropdown_menu/style.css")]
struct Styles;

/// The trigger's element, so closing the menu from the keyboard can put focus back on it.
/// Through the element handle rather than the DOM, so it also works on native, where the
/// focused item would otherwise be removed with focus still on it.
#[derive(Clone, Copy)]
struct TriggerEl(Signal<Option<std::rc::Rc<MountedData>>>);

fn refocus_trigger() {
    if let Some(TriggerEl(el)) = try_consume_context::<TriggerEl>() {
        if let Some(el) = el() {
            spawn(async move {
                _ = el.set_focus(true).await;
            });
        }
    }
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    use_context_provider(|| TriggerEl(Signal::new(None)));
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
    let trigger = try_use_context::<TriggerEl>();
    let base = attributes!(button {
        class: Styles::dx_dropdown_menu_trigger,
        onmounted: move |e: MountedEvent| {
            if let Some(TriggerEl(mut el)) = trigger {
                el.set(Some(e.data()));
            }
        },
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
            let space = e.key() == Key::Character(" ".into());
            if e.key() == Key::Escape || e.key() == Key::Enter || space {
                refocus_trigger();
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
        // Choosing an item closes the menu; the primitive stops the key here, so the
        // content's handler never sees it.
        onkeydown: move |e: KeyboardEvent| {
            if e.key() == Key::Enter || e.key() == Key::Character(" ".into()) {
                refocus_trigger();
            }
        },
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        dropdown_menu::DropdownMenuItem {
            disabled: props.disabled,
            value: props.value,
            index: props.index,
            on_select: props.on_select,
            attributes: merged,
            {props.children}
        }
    }
}
