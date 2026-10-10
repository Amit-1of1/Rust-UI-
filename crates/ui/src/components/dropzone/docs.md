# Dropzone

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
