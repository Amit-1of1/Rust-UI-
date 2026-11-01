The input element is used to accept a text input from the user. It can be styled and configured to fit various use cases, such as forms or search bars.

## Component Structure

```rust
input {
    // Global html attributes
    class: "dx-input",
    // Children
    {children}
}
```

## Sizes

`field_size: FieldSize::Sm | Default | Lg` changes height, padding and text size (`size` is already the HTML attribute).
