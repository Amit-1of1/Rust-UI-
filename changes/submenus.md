### Added

- **ui:** Nested submenus for `DropdownMenu`, `ContextMenu` and `Menubar`: `DropdownMenuSub`, `DropdownMenuSubTrigger`, `DropdownMenuSubContent` and the same three parts for the other two (experimental). ArrowRight, Enter or Space opens one and focuses its first item, ArrowLeft or Escape closes only the submenu and returns focus to its item, choosing an item closes the whole menu. The pointer opens a submenu after resting 100 ms on its item and has 300 ms to cross to it. Submenus nest, reuse each menu's look, and work on native too. The behaviour lives in the primitives fork (docs/decisions/2026-11-01-submenus-in-primitives-fork.md).

- The docs app's size budget is raised to its measured size (main wasm 1072.5 KB): main had already grown to 1058.5 KB, and the submenu demos add about 14 KB.
