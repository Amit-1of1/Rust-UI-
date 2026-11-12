The Avatar components display a user's profile picture or fallback initials. Use the composable `Avatar`, `AvatarImage`, and `AvatarFallback` primitives when you need full control, or `ImageAvatar` for the common image-with-fallback case.

## Component Structure

```rust
Avatar {
    label: "Jane Doe",
    AvatarImage {
        src: "https://example.com/avatar.png",
        alt: "Jane Doe",
    }
    AvatarFallback { "JD" }
}
```

```rust
ImageAvatar {
    src: "https://example.com/avatar.png",
    alt: "Jane Doe",
    on_state_change: |state: AvatarState| { /* image is loading/loaded/failed */ },
    "JD"
}
```

## Naming

The avatar is `role="img"`, so a screen reader reads its name, never the initials inside. `label` names it (`ImageAvatar` uses `alt` when there is no `label`). An avatar with no `label`, `alt`, `aria-label` or `aria-labelledby` is decorative: it renders `aria-hidden="true"`, which is right when the person's name is shown next to it.

`alt` on `AvatarImage` and `ImageAvatar` is required, so an image with no text alternative doesn't compile. Pass `alt: ""` on purpose when the name is shown beside the avatar.

## Sizes, shapes and groups

`size` is `Small`, `Medium` or `Large`; `shape` is `Circle` or `Rounded` (a rounded square). The image is cropped to the shape (`object-fit: cover`), so photos of any size or ratio work. Without an image, the fallback (initials or an icon) sits on a filled background whose text passes WCAG AA.

`AvatarGroup` overlaps several avatars, for a team or the people on a document. Pass the avatars you show and the number left out as `more`, which adds a "+N" avatar (read as "N more"):

```rust
AvatarGroup { label: "Design team", more: 3,
    Avatar { size: AvatarImageSize::Small, label: "Ada Lovelace", AvatarFallback { "AL" } }
    Avatar { size: AvatarImageSize::Small, label: "Grace Hopper", AvatarFallback { "GH" } }
}
```
