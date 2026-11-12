The card component is a flexible container for grouping related content and actions. It provides a structured layout with optional header, content, and footer sections.

## Component Structure

```rust
// The Card component must wrap all card elements.
Card {
    // CardHeader contains the title, description, and optional action.
    CardHeader {
        // CardTitle displays the main heading.
        CardTitle { "Card Title" }
        // CardDescription provides supporting text.
        CardDescription { "Card description goes here." }
        // CardAction positions action elements (e.g., buttons) in the header.
        CardAction {
            Button { "Action" }
        }
    }
    // CardContent holds the main body content.
    CardContent {
        p { "Main content of the card." }
    }
    // CardFooter contains footer actions or information.
    CardFooter {
        Button { "Submit" }
    }
}
```

## Layout Notes

- When `CardAction` is present inside `CardHeader`, the header automatically switches to a two-column grid layout.
- `CardMedia` holds an edge-to-edge image: put it first (or last) and it meets the card's edge with the card's rounded corners, no negative margins needed. An `img` inside is cropped to fill it.
- The card's corner radius is twice the theme `--radius` (1rem by default), so the theme's radius setting reaches cards.

```rust
Card {
    CardMedia { img { src: "/cabin.jpg", alt: "The cabin from the lake", style: "aspect-ratio: 16 / 9;" } }
    CardHeader { CardTitle { "Lakeside cabin" } }
}
```

## Whole-card links, side media and groups

- `CardLink { href }` goes inside the title. Its click area covers the whole card, while screen readers hear only the title. Buttons in the card still work on their own.
- `Card { horizontal: true }` puts a `CardMedia` beside the content on screens at least 40rem wide, and stacks it again below that.
- `CardGroup { min_width: "16rem" }` fits as many columns as there is room for, each at least `min_width` wide. It adds no list roles; add `role: "list"` and `role: "listitem"` yourself when the count matters.

```rust
CardGroup {
    Card { CardHeader { CardTitle { CardLink { href: "/guides/forms", "Forms guide" } } } }
    Card { horizontal: true,
        CardMedia { img { src: "/cabin.jpg", alt: "The cabin" } }
        CardHeader { CardTitle { "Lakeside cabin" } }
    }
}
```
