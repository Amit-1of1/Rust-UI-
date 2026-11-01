### Added

- **cli:** `atoll tokens` exports the theme as W3C Design Tokens (DTCG) JSON, one file per colour mode, for Figma variables, Tokens Studio, Penpot or Style Dictionary (docs/DESIGN_TOKENS.md). (#91)
- **ui:** `SidebarMenuButton` takes `close_on_mobile` (default `true`). (#91)

### Changed

- **ui:** On phones the sidebar sheet now closes after a `SidebarMenuButton` is clicked (buttons with `aria-expanded` or `aria-haspopup` keep it open), and with the `router` feature after every in-app navigation. (#91)
