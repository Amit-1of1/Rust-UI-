The sheet component is a panel that slides in from the edge of the screen. It can be used to display additional content, forms, or navigation menus without leaving the current page.

## Component Structure

```rust
Sheet {
    open: open(),
    // Which edge to slide in from. Available sides: Top, Right (default), Bottom, Left.
    "data-side": SheetSide::Right.as_str(),
    // Required: shown in the sheet's header and used as its accessible name.
    // Leaving it out is a compile error.
    title: "Edit Profile",
    description: "Make changes to your profile here.",
    SheetContentClose {}
    SheetFooter {
        SheetClose { "Close" }
    }
}
```

The sheet renders its own header (`title`, then `description`) above the children. `hide_title: true` hides that header visually but keeps it for screen readers, as the mobile sidebar does. For a description with links or formatting, put a `SheetDescription` in the children instead of `description`.

## SheetClose with `as` prop

The `as` prop allows you to render a custom element while preserving the close behavior, similar to shadcn/ui's `asChild` pattern.

```rust
// Default: renders as <button>
SheetClose { "Close" }

// Custom element: attributes include the preset onclick handler
SheetClose {
    as: |attributes| rsx! {
        a { href: "#", ..attributes, "Go back" }
    }
}
```
