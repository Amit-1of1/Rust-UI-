use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

use crate::components::hooks::use_focus_opener;

#[css_module("/src/components/button/style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Destructive => "destructive",
            ButtonVariant::Outline => "outline",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Link => "link",
        }
    }
}

/// A [`Button`]'s size. A button that shows only an icon is an [`IconButton`], which has its own
/// [`IconButtonSize`] and a required `label`.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
}

impl ButtonSize {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "xs",
            ButtonSize::Sm => "sm",
            ButtonSize::Default => "default",
            ButtonSize::Lg => "lg",
        }
    }
}

/// A [`Button`]'s look for an element that is not a `<button>`, such as the router's `Link`
/// (which renders its own `<a>`):
/// `Link { to: Route::Trips {}, attributes: button_look(ButtonVariant::Outline, ButtonSize::Default), "My trips" }`.
/// Don't also pass `class` to that element: it would replace the button class.
pub fn button_look(variant: ButtonVariant, size: ButtonSize) -> Vec<Attribute> {
    attributes!(a {
        class: Styles::dx_button,
        "data-style": variant.class(),
        "data-size": size.class(),
    })
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: Styles::dx_button,
        "data-style": variant.class(),
        "data-size": size.class(),
    });
    // Remembered as the opener while focused, so a dialog it opens can return focus to it.
    let opener = use_focus_opener();
    let merged = merge_attributes(vec![base, opener, attributes]);

    rsx! {
        button {
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    f.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(f) = &onkeydown {
                    f.call(event);
                }
            },
            ..merged,
            {children}
        }
    }
}

/// An [`IconButton`]'s size: a square button sized for one icon.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum IconButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
}

impl IconButtonSize {
    pub fn class(&self) -> &'static str {
        match self {
            IconButtonSize::Xs => "icon-xs",
            IconButtonSize::Sm => "icon-sm",
            IconButtonSize::Default => "icon",
            IconButtonSize::Lg => "icon-lg",
        }
    }
}

/// A button that shows only an icon, such as a close or search button.
///
/// `label` is required: with no text inside, it is the button's only accessible name
/// (`aria-label`), so "Close" or "Search" instead of nothing. Mark the icon itself decorative.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # use ui::icons::Search;
/// # fn app() -> Element {
/// rsx! { IconButton { label: "Search", variant: ButtonVariant::Ghost, Search { "aria-hidden": "true" } } }
/// # }
/// ```
///
/// Leaving `label` out is a compile error ("Missing required field label"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # use ui::icons::Search;
/// # fn app() -> Element {
/// rsx! { IconButton { variant: ButtonVariant::Ghost, Search { "aria-hidden": "true" } } }
/// # }
/// ```
///
/// and so is squaring a plain [`Button`] with an icon size, which no longer exists:
///
/// ```compile_fail,E0599
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Button { size: ButtonSize::Icon, "+" } }
/// # }
/// ```
#[component]
pub fn IconButton(
    /// What the button does, read by screen readers ("Close", "Search").
    #[props(into)]
    label: String,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: IconButtonSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// The icon.
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: Styles::dx_button,
        "data-style": variant.class(),
        "data-size": size.class(),
        aria_label: label,
    });
    // Remembered as the opener while focused, so a dialog it opens can return focus to it.
    let opener = use_focus_opener();
    let merged = merge_attributes(vec![base, opener, attributes]);

    rsx! {
        button {
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    f.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(f) = &onkeydown {
                    f.call(event);
                }
            },
            ..merged,
            {children}
        }
    }
}
