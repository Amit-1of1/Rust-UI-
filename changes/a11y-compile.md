### Breaking

- **ui:** `Dialog`, `Sheet`, `Drawer` and `AlertDialog` take a required `title` (their heading and accessible name) and an optional one-line `description`, so a dialog without a name no longer compiles. `DialogTitle`, `SheetTitle`, `DrawerTitle` and `AlertDialogTitle` are removed; the `*Description` components stay for rich text. `Dialog`, `Sheet` and `Drawer` also take `hide_title` (visually hidden, still read). Each root has its own props struct (`DialogProps`, `SheetProps`, `DrawerProps`, `AlertDialogProps`) instead of the primitive's. (#48)
- **ui:** `ButtonSize::Icon`, `IconXs`, `IconSm` and `IconLg` are removed. Icon-only buttons are the new `IconButton`, whose `label` (its `aria-label`) is required, sized with `IconButtonSize`. (#48)
- **ui:** `alt` is required on `AvatarImage` and `ImageAvatar`; `alt: ""` marks a decorative image on purpose. (#48)

### Added

- **ui:** `IconButton` and `IconButtonSize`. Doc tests (`compile_fail`) prove that a missing dialog `title`, icon button `label` or avatar image `alt` fails to compile. (#48)
