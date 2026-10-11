use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::html::{FileData, HasFileData};
use dioxus::prelude::*;

#[css_module("/src/components/dropzone/style.css")]
struct Styles;

/// Does a file match an `accept` list such as `.png,image/*,application/pdf`? An empty list
/// accepts everything. Matching ignores case.
pub fn accepts(accept: &str, name: &str, content_type: &str) -> bool {
    let tokens: Vec<String> = accept
        .split(',')
        .map(|t| t.trim().to_lowercase())
        .filter(|t| !t.is_empty())
        .collect();
    if tokens.is_empty() {
        return true;
    }
    let (name, mime) = (name.to_lowercase(), content_type.to_lowercase());
    tokens.iter().any(|t| {
        if let Some(ext) = t.strip_prefix('.') {
            name.ends_with(&format!(".{ext}"))
        } else if let Some(group) = t.strip_suffix("/*") {
            mime.starts_with(&format!("{group}/"))
        } else {
            mime == *t
        }
    })
}

/// "1.5 MB" style size.
pub fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

/// A file picker you can also drop files onto. The real control is a native
/// `<input type="file">` inside the label, so keyboard and screen readers work as they do for
/// any file input; dropped files are checked against `accept` and `max_size` too.
#[component]
pub fn Dropzone(
    /// Visible instructions, for example "Drop files here or browse".
    /// Defaults to "Drop files here or click to browse" ([`UiStrings`](crate::components::ui_strings::UiStrings)).
    #[props(into)]
    prompt: Option<String>,
    /// Comma-separated: `.png`, `image/*`, `application/pdf`. Empty accepts anything.
    #[props(into, default)]
    accept: String,
    #[props(default)] multiple: bool,
    /// Largest file in bytes (0 for no limit).
    #[props(default)]
    max_size: u64,
    #[props(default)] disabled: bool,
    /// Called with the accepted files after every drop or pick.
    #[props(default)]
    on_files: Callback<Vec<FileData>>,
) -> Element {
    let t = use_ui_strings();
    let prompt = prompt.unwrap_or_else(|| t.drop_files.clone());
    let only_one = t.only_one_file.clone();
    let (not_accepted, too_large) = (t.not_accepted.clone(), t.too_large.clone());
    let mut dragging = use_signal(|| false);
    let mut names = use_signal(Vec::<(String, u64)>::new);
    let mut problem = use_signal(|| None::<String>);

    let accept_for_handle = accept.clone();
    let handle = Callback::new(move |files: Vec<FileData>| {
        let mut ok = Vec::new();
        let mut rejected = Vec::new();
        for file in files {
            let (name, size) = (file.name(), file.size());
            let kind = file.content_type().unwrap_or_default();
            if !accepts(&accept_for_handle, &name, &kind) {
                rejected.push(fill(&not_accepted, &[("name", &name)]));
            } else if max_size > 0 && size > max_size {
                rejected.push(fill(
                    &too_large,
                    &[("name", &name), ("size", &format_size(max_size))],
                ));
            } else {
                ok.push(file);
            }
        }
        if !multiple && ok.len() > 1 {
            rejected.push(only_one.clone());
            ok.truncate(1);
        }
        problem.set((!rejected.is_empty()).then(|| rejected.join(". ") + "."));
        if !ok.is_empty() {
            names.set(ok.iter().map(|f| (f.name(), f.size())).collect());
            on_files.call(ok);
        }
    });

    rsx! {
        div { class: Styles::dx_dropzone_root,
            label {
                class: Styles::dx_dropzone,
                "data-dragging": dragging().then_some("true"),
                "data-disabled": disabled.then_some("true"),
                ondragenter: move |e| {
                    e.prevent_default();
                    dragging.set(true);
                },
                ondragover: move |e| e.prevent_default(),
                ondragleave: move |_| dragging.set(false),
                ondrop: move |e| {
                    e.prevent_default();
                    dragging.set(false);
                    if !disabled {
                        handle.call(e.files());
                    }
                },
                input {
                    class: Styles::dx_dropzone_input,
                    r#type: "file",
                    accept: "{accept}",
                    multiple,
                    disabled,
                    onchange: move |e| handle.call(e.files()),
                }
                span { class: Styles::dx_dropzone_prompt, "{prompt}" }
                if !accept.is_empty() {
                    span { class: Styles::dx_dropzone_hint, {fill(&t.accepts, &[("types", &accept)])} }
                }
            }
            if let Some(message) = problem() {
                p { class: Styles::dx_dropzone_error, role: "alert", "{message}" }
            }
            if !names.read().is_empty() {
                ul { class: Styles::dx_dropzone_files, "aria-label": t.selected_files.clone(),
                    for (name , size) in names.read().iter() {
                        li { key: "{name}", "{name} ", span { class: Styles::dx_dropzone_hint, "({format_size(*size)})" } }
                    }
                }
            }
        }
    }
}
