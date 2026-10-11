# Taskboard: one small app, built three ways

The same app is built with:

- `apps/trial`: Rust, Dioxus 0.7 and this kit (wasm).
- `bench/react`: React 19, Radix UI primitives (what shadcn/ui is built on) and plain CSS, built with Vite.
- `bench/svelte`: Svelte 5 and Bits UI (what shadcn-svelte is built on) and plain CSS, built with Vite.

Each version uses its own stack's normal accessible primitives. The DOM contract below must match exactly, because the scripts in `bench/` drive all three versions the same way.

## Data

Rows are generated deterministically, so every version shows the same data:

```
ADJ   = ["Quick","Lazy","Bright","Quiet","Brave","Calm","Eager","Fancy","Gentle","Happy"]
NOUN  = ["report","review","deploy","design","fix","test","sync","plan","audit","launch"]
NAMES = ["Ada","Grace","Linus","Margaret","Alan"]
row i (0-based):
  id       = i + 1
  title    = `${ADJ[i % 10]} ${NOUN[(i * 7) % 10]} #${i + 1}`
  assignee = NAMES[i % 5]
  priority = ["Low","Medium","High"][i % 3]
  status   = ["Todo","Doing","Done","Done"][i % 4]
  estimate = (i * 37) % 13 + 1          // hours
```

`?rows=N` sets the row count. The default is 200.

## App (default route)

- Page `<h1>Taskboard</h1>`, plus a header holding a **dark mode switch** (role=switch, name "Dark mode") and a button "Command menu (Ctrl+K)".
- When the app has rendered and can respond to input, set `document.body.dataset.ready = "1"`.
- **Tabs** (role=tablist) with three tabs: "Tasks", "Stats", "Settings".
- **Tasks tab**
  - A text input labelled "Filter tasks" that filters rows by title (case-insensitive contains).
  - A table with a `<caption>Tasks</caption>` and columns Select, Title, Assignee, Priority, Status, Estimate.
    - Title, Assignee, Priority and Estimate headers hold a `<button>` that sorts. Clicks cycle ascending, then descending. The `<th>` carries `aria-sort`.
    - The select-all checkbox has the name "Select all rows on this page". Each row checkbox is named "Select {title}".
  - Paging is 20 rows per page. A nav labelled "Table pages" holds the buttons "Previous" and "Next" and the text "Page X of Y".
  - A status line (role=status) reads "Showing A–B of N rows".
  - A button "New task" opens a **modal dialog** named "New task" containing:
    - Input "Title". It is required: submitting it empty shows the text "Title is required", linked with aria-describedby, sets aria-invalid=true and keeps focus in the dialog.
    - A select (listbox popup) labelled "Assignee", with the 5 names.
    - A radio group labelled "Priority" (Low, Medium, High).
    - Buttons "Save" and "Cancel".
    - Save adds the task at the top of the table, closes the dialog and shows a **toast** reading "Task added".
    - Escape or Cancel closes the dialog. Focus returns to "New task".
- **Command menu**: Ctrl+K, or the header button, opens a dialog containing an input with role=combobox named "Search commands" and a listbox of commands: "New task", "Toggle dark mode", "Go to Stats", "Go to Settings". Arrow keys move the highlight (aria-activedescendant). Enter runs the highlighted command. Escape closes.
- **Stats tab**
  - An SVG bar chart (role=img, with an aria-label that summarises it) showing the number of tasks per status.
  - A progress bar (role=progressbar, aria-valuenow = percent done) labelled "Done".
- **Settings tab**
  - A slider (role=slider) labelled "Rows per page" (10–50, step 10). It changes the page size.
  - A textarea labelled "Notes" that grows with its content.

## Bench route (`?bench`)

This route is a stripped page in the style of js-framework-benchmark. It renders only the following elements, built with the stack's normal list rendering (keyed rows) and the same table markup as the app:

- buttons with ids `run` (replace with 1,000 rows), `runlots` (replace with 10,000 rows), `add` (append 1,000), `update` (append " !!!" to every 10th row's title), `swaprows` (swap rows 2 and 999, 1-based), and `clear`
- `<table id="bench"><tbody>`: one `<tr>` per row with `<td>{id}</td><td>{title}</td>`, plus a `<td>` holding a button "×" that removes that row
- Clicking a row's title cell marks it selected by adding `class="selected"` to its `<tr>` (one row at a time)
- Rows use the same generator as the app, with ids continuing across `add`
- `document.body.dataset.ready = "1"` once mounted

## Builds measured

Release builds only:

- Dioxus: `dx build --release --platform web`
- React and Svelte: `vite build`

Each build is served as static files.
