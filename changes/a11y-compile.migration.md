### Dialogs need a `title`; icon-only buttons are `IconButton`; avatar images need `alt` (PR #48)

**Who:** apps that use `Dialog`, `Sheet`, `Drawer`, `AlertDialog`, `ButtonSize::Icon*`, `AvatarImage` or `ImageAvatar`.

**Before:** the title was a child (`DialogTitle { "Edit profile" }`, inside `SheetHeader` for sheets) and could be forgotten; the dialog was then announced as just "dialog". Icon-only buttons were `Button { size: ButtonSize::Icon, aria_label: "Close", .. }`, and `aria_label` was optional. `alt` on avatar images defaulted to `""`.

**After:** leaving any of these out is a compile error that names the field ("Missing required field title", "label" or "alt").

```rust
Dialog { open: open(), title: "Edit profile", description: "Changes save when you close it.",
    // content
}
Sheet { open: open(), title: "Edit profile", /* SheetHeader is rendered for you */ }
IconButton { label: "Close", variant: ButtonVariant::Ghost, size: IconButtonSize::Sm,
    X { "aria-hidden": "true" }
}
ImageAvatar { src: "/ada.png", alt: "Ada Lovelace", "AL" }
```

**What to do:**
- Move each `DialogTitle` / `SheetTitle` / `DrawerTitle` / `AlertDialogTitle` text into the root's `title`. A one-line `*Description` can move into `description`; keep a `*Description` child for links or formatting (not both). A `SheetHeader` that only held the title and description can go: the sheet renders its own header.
- A title that was visually hidden (`class: "sr-only"`) becomes `hide_title: true`. A title with markup other than text has to become text; put the rest in the content.
- Replace `Button { size: ButtonSize::Icon, aria_label: "Close", .. }` with `IconButton { label: "Close", .. }`, and `ButtonSize::IconXs` / `IconSm` / `IconLg` with `IconButtonSize::Xs` / `Sm` / `Lg`.
- Add `alt` to every `AvatarImage` and `ImageAvatar`: the person's name, or `""` when the name is shown beside the avatar.
- Copy users: `atoll add --force dialog sheet drawer alert_dialog button avatar`, and also `command`, `sidebar` and `number_field` if you copied them (they use the new props).
