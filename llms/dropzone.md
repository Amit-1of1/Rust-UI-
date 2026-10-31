# Dropzone

- Copy into your app: `atoll add dropzone` or `dx components add dropzone --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::dropzone::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A file picker you can also drop files onto.

```rust
Dropzone { accept: "image/*,.pdf", multiple: true, max_size: 5 * 1024 * 1024,
           on_files: move |files| upload(files) }
```

The control is a native `<input type="file">` inside the label, so it behaves like any file input. Dropped files are checked against `accept` (`.ext`, `type/*`, `type/subtype`) and `max_size`; rejected files produce a message and are not passed on. `accepts()` and `format_size()` are public helpers.

## Accessibility

- Role and name: the label text is the input's name; the selected files are a labelled list; rejection messages are `role="alert"`.
- Keyboard: Tab to the (visually hidden) input, Space or Enter opens the file picker. Drag and drop is an extra, never the only way.
- Reduced motion: no animation.

## Source

Original to this repo. Rust/UI's `dropzone.rs` (498 lines, wasm-bindgen based) was used only to see the feature set.

## API

```rust
/// Does a file match an `accept` list such as `.png,image/*,application/pdf`? An empty list
/// accepts everything. Matching ignores case.
pub fn accepts(accept: &str, name: &str, content_type: &str) -> bool

/// "1.5 MB" style size.
pub fn format_size(bytes: u64) -> String

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
) -> Element
```

## Example

```rust
use crate::components::dropzone::*;
use dioxus::html::FileData;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut count = use_signal(|| 0_usize);
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 28rem;",
            Dropzone {
                accept: "image/*,.pdf",
                multiple: true,
                max_size: 5 * 1024 * 1024,
                on_files: move |files: Vec<FileData>| count.set(count() + files.len()),
            }
            p { style: "margin: 0; font-size: 0.875rem;", role: "status", "Files received: {count()}" }
        }
    }
}
```
