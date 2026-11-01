//! Remember the theme between visits.

use super::component::{use_theme, Theme, ThemeMode, ACCENTS, DENSITIES, PRESETS};
use dioxus::prelude::*;

/// Saves [`use_theme`]'s choices (mode, preset, accent, density, radius) in the browser's
/// `localStorage` under `storage_key` (default `"theme"`), and restores them on the next visit. Render it once, next to
/// `ThemeStyles {}`:
///
/// ```rust,ignore
/// rsx! { ThemeStyles {} PersistTheme {} Router::<Route> {} }
/// ```
///
/// The saved value is restored right after the first render, so it is safe in a fullstack
/// app (the server has no `localStorage`). Until then the page shows the default theme; to
/// avoid that flash, also put [`THEME_INIT_SCRIPT`] in the page's `<head>`.
///
/// Web only (a Rust `web-sys` call, no JavaScript of the kit's); elsewhere it does nothing.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn PersistTheme(
    /// The `localStorage` key. Change it when two apps share an origin.
    #[props(into, default = "theme".to_string())]
    storage_key: String,
) -> Element {
    let theme = use_theme();
    let mut restored = use_signal(|| false);
    use_effect(move || {
        let now = encode_theme(&theme); // subscribes: re-runs on every change
        if !*restored.peek() {
            restored.set(true);
            if let Some(saved) = storage::get(&storage_key) {
                apply_saved_theme(&theme, &saved);
                return;
            }
        }
        storage::set(&storage_key, &now);
    });
    rsx! {}
}

/// `mode=dark;preset=reef;accent=blue;density=compact;radius=12` (preset only when it isn't
/// lagoon, radius only when one was set).
pub fn encode_theme(theme: &Theme) -> String {
    let mode = match theme.mode() {
        ThemeMode::System => "system",
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    };
    let mut out = format!("mode={mode}");
    if theme.preset() != "lagoon" {
        out.push_str(&format!(";preset={}", theme.preset()));
    }
    out.push_str(&format!(
        ";accent={};density={}",
        theme.accent(),
        theme.density()
    ));
    // Unset means the preset's own; leave it out so the app's own `--radius` keeps working.
    if let Some(px) = theme.radius_override() {
        out.push_str(&format!(";radius={px}"));
    }
    out
}

/// Applies a value written by [`encode_theme`]. The stored text is user-controlled, so every part
/// is checked against the known values and anything else is ignored.
pub fn apply_saved_theme(theme: &Theme, saved: &str) {
    // The preset first: picking one resets the radius, which may come before it.
    let preset = saved
        .split(';')
        .filter_map(|p| p.strip_prefix("preset="))
        .find(|v| PRESETS.contains(v));
    if let Some(name) = preset {
        theme.set_preset(name);
    }
    for part in saved.split(';') {
        let Some((name, value)) = part.split_once('=') else {
            continue;
        };
        match name {
            "mode" => match value {
                "light" => theme.set_mode(ThemeMode::Light),
                "dark" => theme.set_mode(ThemeMode::Dark),
                "system" => theme.set_mode(ThemeMode::System),
                _ => {}
            },
            "accent" if ACCENTS.contains(&value) => theme.set_accent(value),
            "density" if DENSITIES.contains(&value) => theme.set_density(value),
            "radius" => {
                if let Ok(px) = value.parse::<u32>() {
                    theme.set_radius(px);
                }
            }
            _ => {}
        }
    }
}

/// A small script for the page's `<head>` that applies the saved theme before the first
/// paint, so a returning dark-mode user never sees a white page while the app loads. It
/// reads the `theme` key that [`PersistTheme`] writes and only sets known values.
///
/// This is the app's own script, opt in: put it in your `index.html`, or render it in a
/// fullstack app with `document::Script { {THEME_INIT_SCRIPT} }`.
pub const THEME_INIT_SCRIPT: &str = r#"(function(){try{var s=localStorage.getItem("theme");if(!s)return;var h=document.documentElement,ok={mode:/^(light|dark)$/,preset:/^(reef|abyss|driftwood|classic)$/,accent:/^(blue|green|violet|rose|orange)$/,density:/^(compact|comfortable)$/,radius:/^([0-9]|1[0-9]|20)$/};s.split(";").forEach(function(p){var kv=p.split("="),k=kv[0],v=kv[1];if(!ok[k]||!ok[k].test(v))return;if(k==="mode")h.setAttribute("data-theme",v);else if(k==="radius")h.style.setProperty("--radius",v+"px");else h.setAttribute("data-"+k,v);});}catch(e){}})();"#;

#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod storage {
    fn local() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }
    pub fn get(key: &str) -> Option<String> {
        local()?.get_item(key).ok().flatten()
    }
    pub fn set(key: &str, value: &str) {
        if let Some(s) = local() {
            let _ = s.set_item(key, value);
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
mod storage {
    pub fn get(_key: &str) -> Option<String> {
        None
    }
    pub fn set(_key: &str, _value: &str) {}
}
