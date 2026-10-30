use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    self, MenubarContentProps, MenubarItemProps, MenubarMenuProps, MenubarProps,
    MenubarTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
#[css_module("/src/components/menubar/style.css")]
struct Styles;

/// The trigger's element, so closing from the keyboard can put focus back on it. Through
/// the element handle rather than the DOM, so it also works on native, where focus would
/// otherwise stay on the removed menu.
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

fn is_close_key(e: &KeyboardEvent) -> bool {
    matches!(e.key(), Key::Escape | Key::Enter) || e.key() == Key::Character(" ".into())
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
    use_context_provider(|| TriggerEl(Signal::new(None)));
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
    let trigger = try_use_context::<TriggerEl>();
    let base = attributes!(div {
        class: Styles::dx_menubar_trigger,
        onmounted: move |e: MountedEvent| {
            if let Some(TriggerEl(mut el)) = trigger {
                el.set(Some(e.data()));
            }
        },
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
                        refocus_trigger();
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
            on_select: props.on_select,
            attributes: merge_attributes(vec![attributes!(div {
                class: Styles::dx_menubar_item,
                // Choosing an item closes the menu (the primitive stops the key here).
                onkeydown: move |e: KeyboardEvent| {
                    if is_close_key(&e) {
                        refocus_trigger();
                    }
                },
            }), props.attributes]),
            {props.children}
        }
    }
}
