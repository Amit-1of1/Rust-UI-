The HoverCard component can be used to display additional information when a user hovers over an element. It is useful for showing tooltips, additional details, or any other content that should be revealed on hover.

## Component Structure

```rust
// The HoverCard component wraps the trigger element and the content that will be displayed on hover.
HoverCard {
    HoverCardTrigger {
        // Anything inside the trigger (icon, text, rich markup) acts as the hover target.
        {children}
    }
    HoverCardContent {
        side: ContentSide::Bottom,
        align: ContentAlign::Start,
        {children}
    }
}
```

## Don't put controls in the trigger

`HoverCardTrigger` is itself focusable (`role="button"`, `tabindex="0"`), so keyboard users can open the card. A button or link inside it is a control inside a control (axe `nested-interactive`), and screen readers announce it badly. Put text, an avatar or an icon directly inside the trigger, and put links and buttons inside `HoverCardContent` (or next to the trigger).
