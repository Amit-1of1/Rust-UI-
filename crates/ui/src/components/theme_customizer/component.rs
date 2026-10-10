use dioxus::prelude::*;

#[css_module("/src/components/theme_customizer/style.css")]
struct Styles;

/// Accent presets defined in `theme.css` (`[data-accent=...]`). `neutral` is the default.
pub const ACCENTS: [&str; 6] = ["neutral", "blue", "green", "violet", "rose", "orange"];

/// Set an attribute on `<html>` (web and desktop webviews). Values are fixed strings from this
/// file or a number, never user text, so they are safe to inline.
fn set_root_attr(name: &str, value: &str) {
    let js = if value.is_empty() {
        format!("document.documentElement.removeAttribute('{name}')")
    } else {
        format!("document.documentElement.setAttribute('{name}','{value}')")
    };
    let _ = document::eval(&js);
}

/// Controls for the accent colour, corner radius and light/dark mode of the whole page.
///
/// It only sets `data-accent` / `data-theme` and `--radius` on `<html>`, so an app can do the
/// same from its own settings screen.
#[component]
pub fn ThemeCustomizer() -> Element {
    let mut accent = use_signal(|| "neutral");
    let mut radius = use_signal(|| 8_u32);
    let mut mode = use_signal(|| "system");
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
                            let _ = document::eval(&format!(
                                "document.documentElement.style.setProperty('--radius','{v}px')"
                            ));
                        }
                    },
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
