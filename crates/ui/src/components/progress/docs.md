The Progress component is used to display the progress of a task or operation. It can be used to indicate loading states, file uploads, or any other process that takes time to complete.

## Component Structure

```rust
Progress {
    // The current progress value (0 to max). `None` is indeterminate.
    value: 0.5,
    // The maximum value of the progress (default is 100.0)
    max: 1.0,
}
```

## Label, value and size

`label` puts a visible label above the bar and names it for screen readers; without one, give the bar an `aria-label`. `show_value` adds the percentage. `size` is `Sm`, `Default` or `Lg`.

```rust
Progress { label: "Uploading photos", show_value: true, value: uploaded() }
```

When the length of the wait is unknown, pass `value: None`: the bar moves until the work finishes or fails (and stops moving when the system asks for reduced motion). When you know how much is left, pass the value so the bar and the percentage show it.
