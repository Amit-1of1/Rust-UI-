# Competitor apps: React and Svelte Taskboards

These are the two JavaScript comparison points for `bench/SPEC.md`. Both implement the same
Taskboard app and `?bench` route as `apps/trial`, using each stack's normal accessible
primitives, one plain stylesheet and no extra tuning.

| | `bench/react` | `bench/svelte` |
|---|---|---|
| Framework | React 19.3.0 + react-dom 19.3.0 | Svelte 5.57.2 |
| Primitives | Radix UI (`@radix-ui/react-*`), as in shadcn/ui | Bits UI 2.19.5, as in shadcn-svelte |
| Command menu | cmdk 1.1.1 inside a Radix Dialog (the shadcn `CommandDialog` pattern) | Bits UI `Command` inside a Bits UI `Dialog` |
| Toast | `@radix-ui/react-toast` | svelte-sonner 1.2.1 |
| Build | Vite 8.3.4 (Rolldown 1.2.13) + `@vitejs/plugin-react` 6.1.2 | Vite 8.3.4 (Rolldown 1.2.13) + `@sveltejs/vite-plugin-svelte` 7.3.1 |
| TypeScript | 7.0.2 (`tsc --noEmit` runs as part of `npm run build`) | 7.0.2 (Svelte compiles `lang="ts"` itself, so the build does no type check) |

## Exact versions

All direct dependencies are pinned exactly in `package.json`, and each app commits its `package-lock.json`.

React (`bench/react/package.json`):

- react 19.3.0, react-dom 19.3.0
- @radix-ui/react-dialog 1.2.0, react-tabs 1.1.22, react-switch 1.3.8, react-checkbox 1.3.12,
  react-select 2.3.8, react-radio-group 1.4.8, react-slider 1.5.0, react-progress 1.1.17,
  react-toast 1.2.24
- cmdk 1.1.1
- dev: vite 8.3.4, @vitejs/plugin-react 6.1.2, typescript 7.0.2, @types/react 19.3.0, @types/react-dom 19.3.0

Svelte (`bench/svelte/package.json`):

- bits-ui 2.19.5, svelte-sonner 1.2.1
- @internationalized/date 3.12.4. This is a required peer dependency of bits-ui; the app does not use it directly.
- dev: svelte 5.57.2, @sveltejs/vite-plugin-svelte 7.3.1, vite 8.3.4, typescript 7.0.2

Node 22.22.0 and npm 10.9.4 were used.

## Build and serve

```sh
cd bench/react   # or bench/svelte
npm ci           # or npm install
npm run build    # react: tsc --noEmit -p . && vite build; svelte: vite build
python3 -m http.server 8000 --directory dist
```

`vite.config.ts` sets `base: './'`, so `dist/` can be served from any path. `bench/.gitignore`
ignores `dist/`, and the root `.gitignore` already ignores `node_modules/`.

## Results

### Build time (`npm run build`, clean `dist/`, 4 vCPU container, 3 runs)

| | runs | typical |
|---|---|---|
| React | 1.17 s, 0.92 s, 0.96 s | ~1.0 s (of which `tsc --noEmit` is ~0.4–1.0 s; TypeScript 7 is the native compiler) |
| Svelte | 3.16 s, 4.03 s, 3.48 s | ~3.5 s (823 modules are transformed against 101 for React, because bits-ui ships as many small `.svelte` files) |

### Dist size (all JS + CSS files; neither app has wasm)

Gzip at level 9 (`gzip -9 -c file | wc -c`), summed over every `.js`, `.css` and `.wasm` file in `dist/`:

| | JS raw | CSS raw | total raw | **total gzip** |
|---|---|---|---|---|
| React | 376,456 B | 8,753 B | 385,209 B | **119,497 B (116.7 KiB)** |
| Svelte | 269,272 B | 23,182 B | 292,454 B | **82,948 B (81.0 KiB)** |

Each app ships one JS chunk and one CSS file, and both routes are in the same bundle. Svelte's CSS
is larger because svelte-sonner injects its own stylesheet (about 14 KB raw). Our shared stylesheet is
8.7 KB in both apps. `index.html` (0.40 KB) is not counted.

### Lines of code written (excluding lockfiles and config)

| | TS/TSX or Svelte | CSS | total (blank lines included) | non-blank |
|---|---|---|---|---|
| React | 763 (8 files) | 637 | 1,400 | 1,312 |
| Svelte | 711 (9 files) | 637 | 1,348 | 1,258 |

`styles.css` and `data.ts` are byte-identical between the two apps. Not counted: `package.json`, `vite.config.ts`, `tsconfig.json`, `index.html`.

## How each piece is built

| Spec item | React | Svelte |
|---|---|---|
| Dark mode switch | `Switch.Root` + `<label for>` | `Switch.Root` + `<label for>` |
| Tabs | `Tabs.Root` (controlled; unmounts inactive panels) | `Tabs.Root bind:value` (keeps inactive panels mounted but hidden) |
| Table checkboxes | `Checkbox.Root` with `aria-label`, plus `"indeterminate"` on select-all | `Checkbox.Root` with `aria-label`, plus `indeterminate` |
| New task dialog | `Dialog.Root` + `Dialog.Trigger`; the form is a child component, so it remounts empty on every open | Same structure |
| Assignee | `Select.*` (trigger is `role=combobox`, popup is `role=listbox`) | `Select.*`, see the deviations below |
| Priority | `RadioGroup.Root aria-labelledby` | Same |
| Toast | `Toast.Provider` / `Toast.Root` / `Toast.Viewport` | `<Toaster />` + `toast('Task added')` |
| Command menu | Radix `Dialog` + cmdk `Command` (`label="Search commands"`, `vimBindings={false}`) | Bits `Dialog` + Bits `Command` (`label="Search commands"`) |
| Ctrl+K | `window.addEventListener('keydown')` in an effect; toggles open | `<svelte:window onkeydown>`; toggles open |
| Chart | Hand-written SVG, `role="img"` with `aria-label="Tasks per status: Todo 50, Doing 50, Done 100"` | Same |
| Progress | `Progress.Root aria-labelledby` ("Done"); `aria-valuenow` = percent Done | Same |
| Slider | `Slider.Root` 10–50 step 10; `Slider.Thumb aria-labelledby` | Same |
| Notes | `<textarea>` resized to `scrollHeight` in a layout effect | Same, in an `$effect` |
| `?bench` rows | `rows.map(row => <Row key={row.id} …/>)`, where `Row` is `React.memo` and callbacks are stable via `useCallback` (the usual js-framework-benchmark React hooks style) | `{#each rows as row (row.id)}` over a `$state` array |
| `data-ready` | `useEffect(() => { document.body.dataset.ready = '1' }, [])` in both routes | `onMount` in both routes |

## Interpretations of the spec

These are choices `apps/trial` should match so all three apps behave the same way.

1. **Bench ids continue across *every* create, not only `add`.** `run` and `runlots` also take the
   next ids, as in js-framework-benchmark (after `run`, `add`, `runlots`, the first row is id 2001).
   With keyed rendering, a `run` that restarted at id 1 would reuse every existing row and turn
   "replace all rows" into a no-op. Titles always come from the generator with `i = id - 1`, so a
   fresh page's first `run` gives ids 1–1000, `Quick report #1` and so on.
2. **`update` changes rows at index 0, 10, 20, …** (that is, 1-based rows 1, 11, 21, …).
3. **`swaprows` does nothing when there are fewer than 999 rows.**
4. **The bench page renders no `<h1>`**, because the spec says it renders only the listed elements.
   axe reports `page-has-heading-one` (moderate, best practice) on `?bench` in both apps. This is
   the only axe result left anywhere.
5. **Sorting**: Title and Assignee compare by plain string order (code units). Priority sorts by rank
   (Low < Medium < High), not alphabetically. Estimate sorts numerically. The sort is stable. Choosing
   a new column starts ascending, and each further click on that column toggles between ascending and
   descending. Unsorted sortable headers carry `aria-sort="none"`. Select and Status carry no `aria-sort`.
6. **New task** gets `status: "Todo"`, `estimate: 1`, the next id, the default priority Medium and the
   default assignee Ada. It is prepended to the list, and the view goes to page 1. If a sort is
   active, the new row is sorted like any other row.
7. **Filter** is the raw input, lowercased (not trimmed), and it resets to page 1. Changing rows per
   page also resets to page 1.
8. **Command menu**: every command closes the menu. "New task" also switches to the Tasks tab, so the
   "New task" button exists and focus can return to it. Ctrl+K (or Cmd+K) toggles the menu.
9. The select column header holds a visually hidden "Select" plus the select-all checkbox.
10. Paging buttons are `disabled` on the first and last page. "Page X of Y" uses Y ≥ 1, and an empty
    result shows "Showing 0–0 of 0 rows".

## Deviations and non-default code

- **Svelte Assignee trigger**: Bits UI 2.19.5 renders `Select.Trigger` as a plain `<button
  aria-haspopup="listbox">`, but puts `aria-activedescendant` on it while open. axe flags that as
  critical (`aria-allowed-attr`), and its listbox has no accessible name (`aria-input-field-name`).
  The app passes `role="combobox"` and `aria-controls` (while open) to the trigger, and
  `aria-labelledby` to `Select.Content`. This matches what Radix renders, and the semantics are now
  correct. Nothing is hidden.
- **Select popup portal (both apps)**: the listbox portals into the dialog element (Radix
  `Select.Portal container`, Bits `Select.Portal to`) rather than into `<body>`. With the default, axe
  reports `region` (content outside landmarks) for Radix, and the popup sits outside the modal's subtree.
  For the same reason the dialog CSS has no `overflow: auto`, which would otherwise clip the popup.
- **Svelte "New task" focus return**: Bits UI returns focus to the element focused *before* the dialog
  opened. That is right for the button, but not when the dialog was opened from the command menu.
  `onCloseAutoFocus` sends focus to the "New task" trigger explicitly, which is Radix's default
  behaviour and what the spec asks for. Both apps also suppress the command dialog's own focus return
  when it hands off to the New task dialog.
- **No type check in the Svelte build**: `svelte-check` was not on the allowed package list, so
  `npm run build` for Svelte is `vite build` only. React's build includes `tsc --noEmit`.
- **Toast styling**: Radix Toast is unstyled, so our stylesheet styles it. svelte-sonner brings its own
  CSS, themed through its `theme` prop.

## Verification

A Playwright + axe-core script (Chromium from `/opt/pw-browsers`) drove each built `dist/` served
by `python3 -m http.server`. It checks:

- every name and role in the spec
- sorting, `aria-sort` and select-all
- paging and the status text
- the filter
- dialog validation (error text, `aria-invalid`, `aria-describedby`, focus kept on Title)
- Save (row at the top, toast, focus back on "New task"), and Escape and Cancel (focus back on "New task")
- the command menu: 4 options, `aria-activedescendant` moving with the arrow keys, Enter running the
  command, Escape returning focus to the header button
- the chart, progress, slider (page count follows it), textarea growth, dark mode and `?rows=45`
- all six bench buttons, keyed DOM reuse on swap (the same `<tr>` nodes move), row selection and row removal

axe ran in 12 states per app, in light and dark: tasks, dialog, dialog with error, select open, command
menu, stats, settings and toast. It found 0 violations in both apps, apart from the bench-page
`page-has-heading-one` noted above. Neither app logged a console error or warning.

## Things worth knowing

- **Radix Toast and Dialog Escape race (React only, intermittent).** In 2 of about 15 full runs, the
  Escape that should close the command dialog did not close it. In both runs the "Task added" toast was
  gone afterwards. `@radix-ui/react-toast` registers its own Escape handler as a dismissable layer, and
  whether a layer listens depends on it being the topmost layer, which every layer recomputes on a
  re-render. When the toast is timing out while a dialog opens, the toast can still be listening and
  swallow the key. I could not reproduce it on demand, and it is library behaviour rather than app
  code, so it is left as is. A 27-point timing sweep (dialog opened 4.7–5.1 s after the toast appeared) did not reproduce it.
- **Bits UI** marks the Command root `role="application"` and points the input's `aria-controls` at
  an inner viewport, not the `role=listbox` element. axe does not flag either.
- svelte-sonner depends on `runed@0.28.0` while bits-ui uses `runed@0.35.1`, so the Svelte bundle
  includes two copies of runed.
- Both apps are idiomatic but not tuned. The React bench uses the standard `memo` row with stable
  callbacks and keeps StrictMode on (it costs nothing in production). The Svelte bench mutates a deep
  `$state` array in place (`push`, index assignment, `splice`, `title += ' !!!'`), which is the
  natural Svelte 5 style. No `$state.raw` or other tuning is used.
