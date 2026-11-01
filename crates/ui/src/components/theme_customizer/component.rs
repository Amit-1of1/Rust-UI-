use crate::components::hooks::use_media_query;
use dioxus::core::provide_root_context;
use dioxus::prelude::*;

#[css_module("/src/components/theme_customizer/style.css")]
struct Styles;

/// Accent presets defined in `theme.css` (`[data-accent=...]`). `neutral` means none: the
/// theme preset's own accent (lagoon teal by default).
pub const ACCENTS: [&str; 6] = ["neutral", "blue", "green", "violet", "rose", "orange"];

/// Theme presets defined in `theme.css` (`[data-preset=...]`): a whole look (colours, fonts,
/// corner radius, density, icon stroke) in one attribute. `lagoon` is the default and sets
/// none. docs/BRAND.md describes each.
pub const PRESETS: [&str; 5] = ["lagoon", "reef", "abyss", "driftwood", "classic"];

/// One theme preset, for pickers and docs.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PresetInfo {
    /// The `data-preset` value (`lagoon` sets none).
    pub name: &'static str,
    /// What it looks like, in a few words.
    pub tagline: &'static str,
    /// The heading and text fonts.
    pub fonts: &'static str,
    /// Its corner radius in pixels (`--radius`).
    pub radius_px: u32,
}

/// [`PRESETS`] with what each one looks like, in the same order.
pub const PRESET_INFO: [PresetInfo; 5] = [
    PresetInfo {
        name: "lagoon",
        tagline: "Deep lagoon teal on warm greys, Reshaped proportions",
        fonts: "Inter",
        radius_px: 10,
    },
    PresetInfo {
        name: "reef",
        tagline: "Coral pink on rosy greys, round and a little roomier",
        fonts: "Geist",
        radius_px: 16,
    },
    PresetInfo {
        name: "abyss",
        tagline: "Indigo on cool slate, square and compact",
        fonts: "IBM Plex Sans",
        radius_px: 4,
    },
    PresetInfo {
        name: "driftwood",
        tagline: "Weathered bronze on warm sand, serif headings",
        fonts: "Source Serif 4 + Inter",
        radius_px: 6,
    },
    PresetInfo {
        name: "classic",
        tagline: "Plain black and white, system fonts, nothing to download",
        fonts: "System fonts",
        radius_px: 8,
    },
];

/// The customizer's choices, shared with [`ThemeScope`]. Empty strings mean "not set".
#[derive(Clone, Copy, PartialEq)]
struct Choices {
    attrs: Signal<[(&'static str, &'static str); 4]>,
    radius: Signal<Option<u32>>,
    /// Whether the choices also go on `<html>`; off under `ThemeStyles { scoped: true }`, so
    /// the kit leaves an existing app's own `data-theme` alone.
    html: Signal<bool>,
}

fn choices() -> Choices {
    try_consume_context::<Choices>().unwrap_or_else(|| {
        provide_root_context(Choices {
            attrs: Signal::new_in_scope(
                [
                    ("data-accent", ""),
                    ("data-density", ""),
                    ("data-theme", ""),
                    ("data-preset", ""),
                ],
                ScopeId::ROOT,
            ),
            radius: Signal::new_in_scope(None, ScopeId::ROOT),
            html: Signal::new_in_scope(true, ScopeId::ROOT),
        })
    })
}

/// Set an attribute on `<html>` (web and desktop webviews) and on every [`ThemeScope`] (any
/// renderer, including native, which has no JavaScript). Values are fixed strings from this
/// file or a number, never user text, so they are safe to inline.
fn set_root_attr(name: &'static str, value: &'static str) {
    if *choices().html.peek() {
        let js = if value.is_empty() {
            format!("document.documentElement.removeAttribute('{name}')")
        } else {
            format!("document.documentElement.setAttribute('{name}','{value}')")
        };
        let _ = document::eval(&js);
    }
    let mut attrs = choices().attrs;
    for slot in attrs.write().iter_mut() {
        if slot.0 == name {
            slot.1 = value;
        }
    }
}

/// Whether [`use_theme`]'s setters also write `<html>` (on by default).
/// `ThemeStyles { scoped: true }` turns it off, so an existing app's own `data-theme` on
/// `<html>` is left alone; an app that copied the components and loads `theme.scoped.css`
/// itself calls `set_writes_html(false)` once at startup instead.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn set_writes_html(on: bool) {
    choices().html.set(on);
}

fn set_root_radius(px: Option<u32>) {
    if *choices().html.peek() {
        let js = match px {
            Some(px) => format!("document.documentElement.style.setProperty('--radius','{px}px')"),
            None => "document.documentElement.style.removeProperty('--radius')".to_string(),
        };
        let _ = document::eval(&js);
    }
    choices().radius.set(px);
}

/// Light or dark, or whatever the operating system prefers.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, PartialEq)]
pub struct Theme {
    choices: Choices,
    prefers_dark: ReadSignal<bool>,
}

/// The shared [`Theme`]. Reading it subscribes the component, so it re-renders on changes
/// made anywhere else.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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

    /// The corner radius in pixels: the one set with [`set_radius`](Self::set_radius), else
    /// the preset's own (12 for lagoon).
    pub fn radius(&self) -> u32 {
        (self.choices.radius)().unwrap_or_else(|| self.preset_info().radius_px)
    }

    /// The radius set with [`set_radius`](Self::set_radius), `None` while the preset's own
    /// applies.
    pub fn radius_override(&self) -> Option<u32> {
        (self.choices.radius)()
    }

    /// Set the corner radius, clamped to 0–20 px like the customizer's slider.
    pub fn set_radius(&self, px: u32) {
        set_root_radius(Some(px.min(20)));
    }

    /// One of [`PRESETS`]; `"lagoon"` when none is set.
    pub fn preset(&self) -> &'static str {
        match self.attr(3) {
            "" => "lagoon",
            name => name,
        }
    }

    /// The [`PresetInfo`] of [`preset`](Self::preset).
    pub fn preset_info(&self) -> PresetInfo {
        let name = self.preset();
        PRESET_INFO
            .into_iter()
            .find(|p| p.name == name)
            .unwrap_or(PRESET_INFO[0])
    }

    /// Pick one of [`PRESETS`]: a whole look at once. It also drops a radius set with
    /// [`set_radius`](Self::set_radius), so the preset's own corners show. Other names are
    /// ignored.
    pub fn set_preset(&self, name: &str) {
        if let Some(name) = PRESETS.into_iter().find(|p| *p == name) {
            set_root_attr("data-preset", if name == "lagoon" { "" } else { name });
            if self.choices.radius.peek().is_some() {
                set_root_radius(None);
            }
        }
    }
}

/// Applies [`ThemeCustomizer`]'s choices to its children, and marks where the kit's scoped
/// theme applies (`data-rui-scope`).
///
/// On the web and in desktop webviews the customizer sets them on `<html>` and this wrapper
/// is not needed. Renderers without JavaScript (dioxus-native) can't reach `<html>`, so wrap
/// the app in `ThemeScope` there. It is a `display: contents` box: it doesn't change layout.
///
/// With `ThemeStyles { scoped: true }` (an existing app with its own CSS) the kit's tokens
/// and rules apply only inside a `ThemeScope`; wrap each part of the app that uses the kit,
/// or the whole app. Several scopes share one [`use_theme`] state.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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
            "data-rui-scope": "",
            "data-accent": value(0),
            "data-density": value(1),
            "data-theme": value(2),
            "data-preset": value(3),
            style,
            {children}
        }
    }
}

/// Density presets defined in `theme.css` (`[data-density=...]`). `default` sets nothing.
pub const DENSITIES: [&str; 3] = ["compact", "default", "comfortable"];

/// Controls for the look (theme preset), accent colour, corner radius, density and light/dark
/// mode of the whole page.
///
/// It only sets `data-preset` / `data-accent` / `data-density` / `data-theme` and `--radius`
/// on `<html>`. Its state is the shared [`use_theme`], so app controls (a header dark-mode
/// switch) stay in step.
#[component]
pub fn ThemeCustomizer() -> Element {
    let theme = use_theme();
    let preset = theme.preset();
    let accent = theme.accent();
    let radius = theme.radius();
    let mode = theme.mode();
    let density = theme.density();
    rsx! {
        div { class: Styles::dx_theme_customizer,
            PresetPicker {}
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
                            span { if name == "neutral" { "{preset}" } else { "{name}" } }
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

/// Picks the page's look, one of [`PRESETS`]: a card per preset, each drawn in its own
/// colours and fonts. [`ThemeCustomizer`] shows it first; render it alone in a header or a
/// settings page. Writes the shared [`use_theme`] state like the customizer.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn PresetPicker(
    /// The group's visible label.
    #[props(into, default = "Look".to_string())]
    label: String,
) -> Element {
    let theme = use_theme();
    let preset = theme.preset();
    // Its own radio group, so a header picker and the customizer's don't merge into one.
    let group = crate::components::hooks::use_unique_id("dx-theme-preset");
    rsx! {
        fieldset { class: Styles::dx_theme_group,
            legend { "{label}" }
            div { class: Styles::dx_theme_presets,
                for info in PRESET_INFO {
                    label {
                        class: Styles::dx_theme_preset,
                        "data-preset": info.name,
                        title: info.tagline,
                        input {
                            r#type: "radio",
                            name: "{group}",
                            value: info.name,
                            checked: preset == info.name,
                            onchange: move |_| theme.set_preset(info.name),
                        }
                        span { class: Styles::dx_theme_preset_name, "{info.name}" }
                        span { class: Styles::dx_theme_preset_fonts, "{info.fonts}" }
                    }
                }
            }
        }
    }
}
