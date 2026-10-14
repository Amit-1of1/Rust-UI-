use dioxus::core::provide_root_context;
use dioxus::prelude::*;

#[css_module("/src/components/theme_customizer/style.css")]
struct Styles;

/// Accent presets defined in `theme.css` (`[data-accent=...]`). `neutral` is the default.
pub const ACCENTS: [&str; 6] = ["neutral", "blue", "green", "violet", "rose", "orange"];

/// The customizer's choices, shared with [`ThemeScope`]. Empty strings mean "not set".
#[derive(Clone, Copy)]
struct Choices {
    attrs: Signal<[(&'static str, &'static str); 3]>,
    radius: Signal<Option<u32>>,
}

fn choices() -> Choices {
    try_consume_context::<Choices>().unwrap_or_else(|| {
        provide_root_context(Choices {
            attrs: Signal::new_in_scope(
                [
                    ("data-accent", ""),
                    ("data-density", ""),
                    ("data-theme", ""),
                ],
                ScopeId::ROOT,
            ),
            radius: Signal::new_in_scope(None, ScopeId::ROOT),
        })
    })
}

/// Set an attribute on `<html>` (web and desktop webviews) and on every [`ThemeScope`] (any
/// renderer, including native, which has no JavaScript). Values are fixed strings from this
/// file or a number, never user text, so they are safe to inline.
fn set_root_attr(name: &'static str, value: &'static str) {
    let js = if value.is_empty() {
        format!("document.documentElement.removeAttribute('{name}')")
    } else {
        format!("document.documentElement.setAttribute('{name}','{value}')")
    };
    let _ = document::eval(&js);
    let mut attrs = choices().attrs;
    for slot in attrs.write().iter_mut() {
        if slot.0 == name {
            slot.1 = value;
        }
    }
}

fn set_root_radius(px: u32) {
    let _ = document::eval(&format!(
        "document.documentElement.style.setProperty('--radius','{px}px')"
    ));
    choices().radius.set(Some(px));
}

/// Applies [`ThemeCustomizer`]'s choices to its children.
///
/// On the web and in desktop webviews the customizer sets them on `<html>` and this wrapper
/// is not needed. Renderers without JavaScript (dioxus-native) can't reach `<html>`, so wrap
/// the app in `ThemeScope` there. It is a `display: contents` box: it doesn't change layout.
#[component]
pub fn ThemeScope(children: Element) -> Element {
    let choices = use_hook(choices);
    let attrs = (choices.attrs)();
    let value = |i: usize| (!attrs[i].1.is_empty()).then_some(attrs[i].1);
    let style = match (choices.radius)() {
        Some(px) => format!("display: contents; --radius: {px}px;"),
        None => "display: contents;".to_string(),
    };
    rsx! {
        div {
            "data-accent": value(0),
            "data-density": value(1),
            "data-theme": value(2),
            style,
            {children}
        }
    }
}

/// Density presets defined in `theme.css` (`[data-density=...]`). `default` sets nothing.
pub const DENSITIES: [&str; 3] = ["compact", "default", "comfortable"];

/// Controls for the accent colour, corner radius, density and light/dark mode of the whole page.
///
/// It only sets `data-accent` / `data-density` / `data-theme` and `--radius` on `<html>`, so an app can do the
/// same from its own settings screen.
#[component]
pub fn ThemeCustomizer() -> Element {
    let mut accent = use_signal(|| "neutral");
    let mut radius = use_signal(|| 8_u32);
    let mut mode = use_signal(|| "system");
    let mut density = use_signal(|| "default");
    rsx! {
        div { class: Styles::dx_theme_customizer,
            fieldset { class: Styles::dx_theme_group,
                legend { "Accent colour" }
                div { class: Styles::dx_theme_swatches,
                    for name in ACCENTS {
                        label {
                            class: Styles::dx_theme_swatch,
                            "data-accent": if name == "neutral" { None } else { Some(name) },
                            input {
                                r#type: "radio",
                                name: "theme-accent",
                                value: name,
                                checked: accent() == name,
                                onchange: move |_| {
                                    accent.set(name);
                                    set_root_attr("data-accent", if name == "neutral" { "" } else { name });
                                },
                            }
                            span { class: Styles::dx_theme_dot, "aria-hidden": "true" }
                            span { "{name}" }
                        }
                    }
                }
            }
            div { class: Styles::dx_theme_group,
                label { r#for: "theme-radius", "Corner radius: {radius()}px" }
                input {
                    id: "theme-radius",
                    r#type: "range",
                    min: "0",
                    max: "20",
                    value: "{radius()}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u32>() {
                            let v = v.min(20);
                            radius.set(v);
                            set_root_radius(v);
                        }
                    },
                }
            }
            fieldset { class: Styles::dx_theme_group,
                legend { "Density" }
                div { class: Styles::dx_theme_swatches,
                    for value in DENSITIES {
                        label { class: Styles::dx_theme_swatch,
                            input {
                                r#type: "radio",
                                name: "theme-density",
                                value,
                                checked: density() == value,
                                onchange: move |_| {
                                    density.set(value);
                                    set_root_attr("data-density", if value == "default" { "" } else { value });
                                },
                            }
                            span { "{value}" }
                        }
                    }
                }
            }
            fieldset { class: Styles::dx_theme_group,
                legend { "Mode" }
                div { class: Styles::dx_theme_swatches,
                    for (value , label_text) in [("system", "System"), ("light", "Light"), ("dark", "Dark")] {
                        label { class: Styles::dx_theme_swatch,
                            input {
                                r#type: "radio",
                                name: "theme-mode",
                                value,
                                checked: mode() == value,
                                onchange: move |_| {
                                    mode.set(value);
                                    set_root_attr("data-theme", if value == "system" { "" } else { value });
                                },
                            }
                            span { "{label_text}" }
                        }
                    }
                }
            }
        }
    }
}
