# Porting shadcn/ui code to Atoll

`atoll port` turns shadcn/ui JSX into an Atoll `rsx!` starting point. It ports **markup**,
not logic: every JSX tree in the file becomes one `rsx!` block, and everything it could not
carry over is listed so you finish it by hand. Nothing is dropped silently.

```sh
atoll port src/components/login-form.tsx            # print code + report
atoll port login-form.tsx --out src/login_form.rs   # write the code, print the report
pbpaste | atoll port -                              # read from stdin
atoll port login-form.tsx --json                    # for editors and agents
```

The MCP server (`atoll mcp`) has the same thing as the `port_jsx` tool.

## What it carries over

| shadcn | Atoll |
| --- | --- |
| `<Button variant="outline" size="sm">` | `Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, ... }` |
| `Card*`, `Alert*`, `Badge`, `Input`, `Label`, `Tabs*`, `Accordion*`, `Tooltip*`, `Popover*`, `DropdownMenu*`, `Avatar*`, `Table*` and more | the kit component with the same job (`TabsList` becomes `TabList`, `Popover` becomes `PopoverRoot`; `TooltipProvider` is dropped, the kit needs none) |
| Lucide icons (`<Mail className="h-4 w-4" />`) | the kit's generated icon of the same name, `Mail { size: "1rem", "aria-hidden": "true" }` |
| Tailwind layout, spacing, type, colour, radius and shadow classes (`flex gap-4 text-sm text-muted-foreground rounded-lg`) | an inline `style` on the theme's tokens (`--unit-x4`, `--font-size-body-2`, `--color-foreground-neutral-faded`, `--radius-large`) |
| `htmlFor`, `type`, `tabIndex`, `aria-*`, `data-*` | `html_for` on kit components, `r#for` / `r#type` / `tabindex` on elements |
| `{" "}`, `{"{"}`, `{name}` | text (braces escaped), `"{name}"` |
| `{/* comment */}` | `// comment` |

It also prints the `atoll add ...` line for the components it used.

## What it leaves for you

Each of these becomes a `// port:` comment in the code and a line in the report:

- **Handlers** (`onClick={() => ...}`): ported as `onclick: move |_| { /* port: ... */ }`.
- **Lists** (`items.map(...)`): write them as a `for` loop inside `rsx!`.
- **Hooks** (`useState`, `useEffect`, `useMemo`, `useRef`, `useForm`): use `use_signal`,
  `use_effect`, `use_memo`; forms have `#[derive(Form)]` (see the `form` component docs).
- **Responsive and state classes** (`md:`, `hover:`, `dark:`) and classes it doesn't know:
  the kit's components already handle hover, focus and dark mode; for layout breakpoints use
  `Grid`/`Stack` from the `layout` component or your own CSS.
- **`asChild`**: the kit's components render their own element; move the child's props onto it.
- **`size="icon"` buttons**: use `IconButton` with a `label` (it won't compile without one).
- Variants the kit doesn't have (`variant="success"` on a `Badge`) and components it doesn't
  have (ported as a `div`).

## Accuracy

The mapping table is checked against the kit's sources by a test
(`port::tests::mappings_match_the_kit`), so every component, enum and variant it emits
exists. The JSX reader handles elements, fragments, self-closing tags, string and `{...}`
props, spreads, comments and nested JSX inside expressions; it is not a TypeScript parser, so
JSX built in unusual ways (for example a tag name in a variable) is skipped.
