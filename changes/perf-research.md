### Changed

- **docs:** The documentation site's release build is split: the home page is in the main wasm file and every other page is one lazy part, fetched once the home page is up. First screen 1,089 KB -> 326 KB brotli, home page ready on slow 4G 6.4 s -> 2.4 s; a component page opened from a link is about 0.9 s slower. Notes: docs/notes/split-docs-and-render-cost.md. (#PR)
- **gallery:** Sections off screen skip style, layout and paint (`content-visibility: auto`): ready at 4x CPU about 3.1 s -> 2.2 s. (#PR)
