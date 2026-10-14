use crate::components::hooks::use_media_query;
use dioxus::core::provide_root_context;
use dioxus::prelude::*;

#[css_module("/src/components/theme_customizer/style.css")]
struct Styles;

/// Accent presets defined in `theme.css` (`[data-accent=...]`). `neutral` is the default.
pub const ACCENTS: [&str; 6] = ["neutral", "blue", "green", "violet", "rose", "orange"];

/// The customizer's choices, shared with [`ThemeScope`]. Empty strings mean "not set".
#[derive(Clone, Copy, PartialEq)]
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

/// Light or dark, or whatever the operating system prefers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemeMode {
    /// The `data-theme` value, empty for `System`.
    fn attr(self) -> &'static str {
        match self {
            Self::System => "",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// The page's theme: mode, accent, density and corner radius. One shared state, so a
/// header switch, a settings screen and [`ThemeCustomizer`] always agree. Get it with
/// [`use_theme`]; the setters write `<html>` and every [`ThemeScope`], like the customizer.
#[derive(Clone, Copy, PartialEq)]
pub struct Theme {
    choices: Choices,
    prefers_dark: ReadSignal<bool>,
}

/// The shared [`Theme`]. Reading it subscribes the component, so it re-renders on changes
/// made anywhere else.
pub fn use_theme() -> Theme {
    let choices = use_hook(choices);
    let prefers_dark = use_media_query("(prefers-color-scheme: dark)");
    Theme {
        choices,
        prefers_dark,
    }
}

impl Theme {
    fn attr(&self, i: usize) -> &'static str {
        (self.choices.attrs).read()[i].1
    }

    pub fn mode(&self) -> ThemeMode {
        match self.attr(2) {
            "light" => ThemeMode::Light,
            "dark" => ThemeMode::Dark,
            _ => ThemeMode::System,
        }
    }

    pub fn set_mode(&self, mode: ThemeMode) {
        set_root_attr("data-theme", mode.attr());
    }

    /// Whether the page is dark now: `Dark`, or `System` while the OS prefers dark (known
    /// on the web only; elsewhere `System` counts as light).
    pub fn is_dark(&self) -> bool {
        match self.mode() {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => (self.prefers_dark)(),
        }
    }

    /// One of [`ACCENTS`]; `"neutral"` when none is set.
    pub fn accent(&self) -> &'static str {
        match self.attr(0) {
            "" => "neutral",
            name => name,
        }
    }

    /// Pick one of [`ACCENTS`]. Other names are ignored.
    pub fn set_accent(&self, name: &str) {
        if let Some(name) = ACCENTS.into_iter().find(|a| *a == name) {
            set_root_attr("data-accent", if name == "neutral" { "" } else { name });
        }
    }

    /// One of [`DENSITIES`]; `"default"` when none is set.
    pub fn density(&self) -> &'static str {
        match self.attr(1) {
            "" => "default",
            value => value,
        }
    }

    /// Pick one of [`DENSITIES`]. Other values are ignored.
    pub fn set_density(&self, value: &str) {
        if let Some(value) = DENSITIES.into_iter().find(|d| *d == value) {
            set_root_attr("data-density", if value == "default" { "" } else { value });
        }
    }

    /// The corner radius in pixels (8 until set).
    pub fn radius(&self) -> u32 {
        (self.choices.radius)().unwrap_or(8)
    }

    /// Set the corner radius, clamped to 0–20 px like the customizer's slider.
    pub fn set_radius(&self, px: u32) {
        set_root_radius(px.min(20));
    }
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
/// It only sets `data-accent` / `data-density` / `data-theme` and `--radius` on `<html>`. Its
/// state is the shared [`use_theme`], so app controls (a header dark-mode switch) stay in step.
#[component]
pub fn ThemeCustomizer() -> Element {
    let theme = use_theme();
    let accent = theme.accent();
    let radius = theme.radius();
    let mode = theme.mode();
    let density = theme.density();
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
                                checked: accent == name,
                                onchange: move |_| theme.set_accent(name),
                            }
                            span { class: Styles::dx_theme_dot, "aria-hidden": "true" }
                            span { "{name}" }
                        }
                    }
                }
            }
            div { class: Styles::dx_theme_group,
                label { r#for: "theme-radius", "Corner radius: {radius}px" }
                input {
                    id: "theme-radius",
                    r#type: "range",
                    min: "0",
                    max: "20",
                    value: "{radius}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u32>() {
                            theme.set_radius(v);
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
                                checked: density == value,
                                onchange: move |_| theme.set_density(value),
                            }
                            span { "{value}" }
                        }
                    }
                }
            }
            fieldset { class: Styles::dx_theme_group,
                legend { "Mode" }
                div { class: Styles::dx_theme_swatches,
                    for (value , label_text) in [
                        (ThemeMode::System, "System"),
                        (ThemeMode::Light, "Light"),
                        (ThemeMode::Dark, "Dark"),
                    ]
                    {
                        label { class: Styles::dx_theme_swatch,
                            input {
                                r#type: "radio",
                                name: "theme-mode",
                                value: label_text.to_lowercase(),
                                checked: mode == value,
                                onchange: move |_| theme.set_mode(value),
                            }
                            span { "{label_text}" }
                        }
                    }
                }
            }
        }
    }
}
