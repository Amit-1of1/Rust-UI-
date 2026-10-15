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
