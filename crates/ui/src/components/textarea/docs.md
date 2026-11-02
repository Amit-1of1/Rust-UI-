The textarea element is used to allow users to enter multi-line text input in a user interface.

## Component Structure

```rust
textarea {
    // Global html attributes
    class: "dx-textarea",
    "data-style": "default",
    // Children
    {children}
}
```

## Sizes

`field_size: FieldSize::Sm | Default | Lg` changes height, padding and text size (`size` is already the HTML attribute).
