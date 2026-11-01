# Core components checklist

How Atoll's components meet the "Core components" list of [designsystemchecklist.com](https://www.designsystemchecklist.com/category/core-components): 29 components and 166 items, each with what we ship and where. Each component's own reference systems (Reshaped, Radix, MUI, Atlassian, Spectrum, Polaris, Carbon, Mantine, Chakra, Material) were the benchmark for how a missing item should behave. Visual look (colours, fonts) follows the current theme tokens; the kit's visual identity is being chosen separately.

**Met** means a component, prop or documented pattern does it today, covered by the kit's tests (markup snapshots, the ARIA check, contracts, contrast proof). **Partly** names what is missing. **Not yet** is open work. `crates/ui/tests/checklist.rs` keeps the totals below in step with the rows.

<!-- checklist-totals -->
| | Met | Partly | Not yet | Total |
|---|---|---|---|---|
| All components | 136 | 21 | 9 | 166 |
<!-- /checklist-totals -->

## Accordion (4)

| Item | Status | How |
|---|---|---|
| Active state | Met | The chevron turns when an item opens; `data-open` styles the item. |
| Composition | Met | `AccordionContent` takes any children. |
| Toggle transition | Met | Height slide on open and close; off under reduced motion. |
| Content and trigger relation | Not yet | The trigger has `aria-controls`, but the panel is not a named `region`. |

## Alert (6)

| Item | Status | How |
|---|---|---|
| Colors | Met | `AlertVariant::Default`, `Destructive`, `Success`, `Warning`, `Info` on theme status tokens; the contrast proof checks the text. |
| Title support | Met | `AlertTitle`, `AlertDescription`. |
| Icon support | Met | `icon` slot, its own column, hidden from screen readers. |
| Supplementary actions | Met | `AlertActions` under the text. |
| Responsiveness | Met | Full width of its container, so full width on phones. |
| Accessibility roles | Met | `live`: `Assertive` (`role="alert"`), `Polite` (`role="status"`) or `Off` for messages present on load. |

## Avatar (7)

| Item | Status | How |
|---|---|---|
| Image | Met | Masked to the shape; `object-fit: cover`, so any size or ratio works. |
| Image fallback | Met | `AvatarFallback` (initials or an icon) while loading and on error. |
| Sizes | Met | `AvatarImageSize::Small`, `Medium`, `Large`. |
| Colors | Met | Filled background behind the fallback; text passes AA (contrast proof). |
| Shape | Met | `AvatarShape::Circle`, `Rounded`. |
| Avatar groups | Met | `AvatarGroup { label, more }` overlaps avatars and adds a "+N" avatar ("N more"). |
| Accessibility label | Met | `role="img"` named by `label` (or `alt`); unnamed avatars are hidden as decorative. `alt` is required at compile time. |

## Badge (7)

| Item | Status | How |
|---|---|---|
| Colors | Met | `Primary`, `Destructive`, `Success`, `Warning` on theme tokens (contrast proof). |
| Variants | Met | `Secondary` (faded fill) and `Outline`. |
| Sizes | Met | `BadgeSize::Sm`, `Default`, `Lg`. |
| Icon support | Met | Decorative icon before the text, sized with the text. |
| Dismissible action | Met | `on_dismiss` adds a "Remove {dismiss_label}" button. `TagGroup` for sets of removable values. |
| Empty state | Met | `dot: true`: a round marker, named by `aria-label` or decorative. |
| Positioning | Met | `BadgeAnchor` pins a badge to any corner of an element, mirrored on right-to-left pages. |

## Button (10)

| Item | Status | How |
|---|---|---|
| Colors | Met | `Primary`, `Destructive` (contrast proof covers hover too). |
| Variants | Met | `Secondary`, `Outline`, `Ghost`, `Link`; `ButtonGroup`; `button_look` for links. |
| Sizes | Met | `ButtonSize::Xs`, `Sm`, `Default`, `Lg`; density-aware. |
| Icon support | Met | Icons next to the label; `IconButton` requires a `label` at compile time. |
| Hover state | Met | Every variant has a hover colour. |
| Active state | Met | Pressed shift on `:active`; `aria-pressed="true"` (toggle buttons) is drawn as on. `Toggle` for on/off buttons. |
| Loading state | Met | `loading: true`: spinner over the label without changing the size, `aria-busy`, presses and repeat submits ignored. |
| Disabled state | Met | `disabled` dims it and blocks presses. |
| Accessibility role | Met | A native `button`; `button_look` keeps link semantics for router links. |
| Focus indicator | Met | `:focus-visible` ring on every variant, solid in forced colours. |

## Breadcrumbs (4)

| Item | Status | How |
|---|---|---|
| Icon support | Met | Any icon inside `BreadcrumbLink`, decorative. |
| Disabled state | Not yet | |
| Collapsed state | Not yet | |
| Custom separator | Partly | `BreadcrumbSeparator` always draws "/". |

## Calendar (7)

| Item | Status | How |
|---|---|---|
| Display modes | Met | `month_count` shows several months side by side; they wrap on narrow screens. |
| Selected state | Met | `Calendar` (one date) and `RangeCalendar` (start, end and the days between highlighted). |
| Month selection | Met | Previous and next month buttons plus month and year selects. |
| Day names | Met | Short weekday names over the grid; `day_name` to translate them. |
| Internationalisation | Partly | `first_day_of_week` and weekday names are configurable; month names come from dioxus-primitives in English. |
| Keyboard navigation | Met | Arrow keys, Page Up/Down, Home/End; leaving the grid changes the month (keyboard contract, model-checked). |
| State announcement | Met | `aria-selected` on selected days, read on focus. |

## Card (5)

| Item | Status | How |
|---|---|---|
| Content composition | Met | `CardHeader`, `CardTitle`, `CardDescription`, `CardAction`, `CardContent`, `CardFooter`. |
| Media sections | Partly | `CardMedia` meets the top or bottom edge; no side-media layout. |
| Supplementary actions | Partly | Footer and header actions; no whole-card link or button pattern. |
| Responsiveness | Met | Fills its container's width. |
| Card groups | Partly | Lay cards out in a grid yourself; `ItemGroup` for list rows. |

## Carousel (6)

| Item | Status | How |
|---|---|---|
| Navigation controls | Met | Previous and Next buttons. |
| Item composition | Met | `CarouselItem` takes any children; `media` for full-bleed photos. |
| Item sizes | Partly | Slides are 80% wide, so the next one peeks in; no size option. |
| Touch navigation | Met | A native scroll area with scroll snapping. |
| Responsiveness | Partly | Same slide width at every size. |
| Keyboard navigation | Met | The track is focusable and arrow keys scroll it; controls are buttons. |

## Checkbox (7)

| Item | Status | How |
|---|---|---|
| Label | Met | `Label { html_for }` or `CheckboxField`; clicking the label toggles it. |
| Checked state | Met | Filled with a check mark; submitted with the form (`name`, `value`). |
| Error state | Met | Red edge with `aria-invalid` or inside an invalid `Field`/`CheckboxGroup`; `FieldError` gives the text. |
| Disabled state | Met | `disabled`. |
| Indeterminate state | Met | `CheckboxState::Indeterminate` shows a dash and `aria-checked="mixed"`. |
| Checkbox group | Met | `CheckboxGroup { label }`: a fieldset with a legend; `Chips` for filter chips. |
| Keyboard support | Met | Space toggles (keyboard contract). |

## Divider (2)

| Item | Status | How |
|---|---|---|
| Direction | Met | `Separator { horizontal }`. |
| Accessibility role | Met | `decorative: false` gives `role="separator"` with `aria-orientation`. |

## Dropdown (6)

| Item | Status | How |
|---|---|---|
| Content composition | Met | `DropdownMenuContent` takes items and any other content. |
| Hover trigger | Not yet | Opens on click and keyboard only. `HoverCard` and `Menubar` cover hover-style menus. |
| Dynamic positioning | Met | CSS anchor positioning flips it at screen edges where supported. |
| Responsiveness | Met | Same flip and size limits on phones. |
| Focus trapping | Partly | Focus returns to the trigger on close; opening does not focus the first item (the first arrow key does). |
| Keyboard navigation | Met | Arrow keys, Enter, Escape closes, Tab leaves (keyboard contract). |

## Icon (3)

| Item | Status | How |
|---|---|---|
| Colors | Met | Drawn in `currentColor`, so icons inherit the text colour; any token through `color`. |
| Sizes | Met | `IconSize::Xs` to `Xl` on the `--icon-size-*` tokens, matched to text sizes. |
| Interactivity | Met | Decorative by default; `IconButton` (required label) for interactive icons. |

## Image (4)

| Item | Status | How |
|---|---|---|
| Sizes | Met | `Image { ratio }` keeps an aspect ratio at any width; `fit` is `Cover` or `Contain`. `AspectRatio` for other content. |
| Image fallback | Met | Empty or failing `src` shows a muted box with an icon, or your `fallback`. |
| Screen density support | Met | `srcset` and `sizes`. |
| Alt text | Met | `alt` is required at compile time; `""` for decorative pictures. |

## Link (6)

| Item | Status | How |
|---|---|---|
| Icon support | Met | Decorative icons beside the text; external links add an icon and "(opens in a new tab)". |
| Colors | Met | `LinkVariant::Default`, `Muted`, `Inherit` (parent's colour), `Destructive`. |
| Disabled state | Met | `disabled`: no `href`, `aria-disabled`, out of the tab order, dimmed. |
| Font inheritance | Met | Size, family and line height always inherited; `Inherit` takes the weight too. |
| Multiline display | Met | Inline, wraps with the paragraph. |
| Accessibility role | Met | A native link (`role="link"` when disabled); `Button { variant: Link }` for actions. |

## List (3)

| Item | Status | How |
|---|---|---|
| Order | Met | `List { marker }`: `Bullet`, `Number`, `Letter`, `Roman`, `None`. |
| Content composition | Met | `ListItem` takes any children; lists nest. |
| Accessibility role | Met | Native `ul`/`ol`, so the item count is read; `None` keeps `role="list"` for Safari. |

## Loading indicator (5)

| Item | Status | How |
|---|---|---|
| Colors | Met | `Spinner` follows its parent's text colour. |
| Sizes | Met | `SpinnerSize::Sm`, `Default`, `Lg`, `Xl`. |
| Loading duration | Met | `Spinner` for unknown waits; `Progress` with a value (and `show_value`) when the time left is known. |
| Reduced motion | Met | The spin slows to 3 seconds instead of stopping. |
| Accessibility label | Met | `role="status"` with `label` (default "Loading", translatable). |

## Modal (8)

| Item | Status | How |
|---|---|---|
| Content composition | Met | `Dialog` takes any children. |
| Supplementary actions | Partly | Put buttons in the dialog yourself; `SheetFooter` exists, no `DialogFooter`. |
| Close action | Partly | Escape and buttons you add; no built-in close button on `Dialog` (`Sheet` has one). |
| Positioning | Met | Centred `Dialog`; `Sheet` slides in from any side; `Drawer` from the bottom. |
| Sizes | Not yet | One width; override with `style`. |
| Focus trapping | Met | Focus moves in, stays in, and returns to the opener. |
| Keyboard navigation | Met | Escape closes; Tab cycles inside (keyboard contract). |
| Title and subtitle labeling | Met | `title` is required (`aria-labelledby`), `description` gives `aria-describedby`; `hide_title` keeps them for screen readers only. |

## Pagination (6)

| Item | Status | How |
|---|---|---|
| Selected page state | Met | `aria-current="page"`, highlighted; pressing it does nothing. |
| Page display ranges | Met | `Pager { siblings }` collapses long ranges to "1 … 4 5 6 … 20". |
| Amount of items per page | Partly | `DataTable` has a page size; `Pager` has none. |
| Indeterminate amount of pages | Not yet | |
| Full page label announcements | Partly | Previous and Next are named; page buttons are read as bare numbers. |
| State announcement | Met | The current page announces `aria-current="page"` on focus. |

## Progress bar (4)

| Item | Status | How |
|---|---|---|
| Label | Met | `Progress { label }` above the bar names it; `show_value` adds the percentage. |
| Sizes | Met | `ProgressSize::Sm`, `Default`, `Lg`. |
| Duration | Met | `value: None` is indeterminate; a value shows what is left. |
| Accessibility label | Met | `aria-label` when there is no visible label. |

## Input radio (5)

| Item | Status | How |
|---|---|---|
| Label | Met | Each `RadioItem` is named by its text; clicking it selects. |
| Checked state | Met | Filled dot; submitted with the form. |
| Error state | Met | Red edge with `aria-invalid` on the group or inside an invalid `Field`; pair with `FieldError`. |
| Radio group | Met | `RadioGroup` (`role="radiogroup"`). |
| Keyboard support | Met | Arrow keys move and select (keyboard contract). |

## Select (8)

| Item | Status | How |
|---|---|---|
| Label | Met | `SelectField { label }`; clicking the label opens it. |
| Error state | Partly | `aria-invalid` and a red edge with `FieldError`; no error icon. |
| Disabled state | Met | `disabled`. |
| Placeholder | Met | `placeholder` while nothing is chosen. |
| Helper text | Met | `SelectField { description }`, in `aria-describedby`. |
| Icon support | Partly | Icons inside options; none at the start of the trigger. |
| Prefix | Not yet | |
| Accessibility label | Met | `aria_label` moves to the trigger. |

## Skeleton (4)

| Item | Status | How |
|---|---|---|
| Sizes | Met | Sized with `style` to match what it replaces. |
| Shapes | Met | `SkeletonShape::Rect`, `Circle`, `Text` (follows the font size; `lines` for paragraphs). |
| Composition | Met | Combine freely; hidden from screen readers (`aria-busy` on the loading region). |
| Reduced motion | Met | The pulse stops (theme rule). |

## Switch (5)

| Item | Status | How |
|---|---|---|
| Label | Met | `Label { html_for }`; clicking the label toggles. |
| Checked state | Met | `role="switch"`, `aria-checked`. |
| Disabled state | Met | `disabled`. |
| Keyboard navigation | Partly | Space toggles; Enter does not (optional in the APG, recorded in its contract). |
| Accessibility label | Met | `aria-label` when there is no visible label. |

## Tabs (7)

| Item | Status | How |
|---|---|---|
| Content composition | Met | `TabContent` takes any children. |
| Variants | Met | `TabsVariant::Default` (pills on a track) and `Ghost`. |
| Selected state | Met | One tab is always selected and highlighted. |
| Disabled state | Met | `TabTrigger { disabled }`. |
| Icon support | Met | Icons inside `TabTrigger`. |
| Equal width layout | Not yet | |
| Keyboard support | Met | Arrow keys, Home and End move between tabs (manual activation: Enter or Space selects; keyboard contract). |

## Text area (7)

| Item | Status | How |
|---|---|---|
| Label | Met | `TextareaField { label }` or `Label { html_for }`. |
| Error state | Met | `aria-invalid` red edge with `FieldError`. |
| Disabled state | Met | `disabled`. |
| Placeholder | Met | `placeholder`, never instead of the label. |
| Helper text | Met | `TextareaField { description }`. |
| Sizes | Partly | Variants and `autosize`; no size scale. |
| Accessibility label | Met | `aria-label` when there is no visible label. |

## Text field (9)

| Item | Status | How |
|---|---|---|
| Label | Met | `TextField { label }` or `FieldLabel { html_for }`. |
| Error state | Met | `aria-invalid` (or an invalid `Field`) turns the edge red; `FieldError` announces the text. |
| Disabled state | Met | `disabled`. |
| Placeholder | Met | `placeholder`, never instead of the label. |
| Helper text | Met | `FieldDescription`, in `aria-describedby`. |
| Icon support | Met | `InputGroup` with an `InputGroupAddon` icon at the start. |
| Prefix / Suffix | Met | `InputGroupAddon` before or after the input. |
| Sizes | Partly | Follows the density setting; no per-field size. |
| Accessibility label | Met | `aria-label` when there is no visible label. |

## Toast (8)

| Item | Status | How |
|---|---|---|
| Content composition | Partly | Title and description; `render_toast` for anything else. |
| Colors | Met | Success, error, warning and info types. |
| Icon support | Partly | No built-in icon per type. |
| Timeout | Met | 5 seconds by default, `permanent` toasts with a close button. |
| Stacking | Met | Toasts stack and spread out on hover and focus. |
| Supplementary action | Partly | Through `render_toast` only. |
| Focus management | Partly | F6 jumps to the toasts and their buttons take focus; the timeout keeps running while focus or the pointer is inside. |
| Reduced motion | Met | Animations off (theme rule). |

## Tooltip (3)

| Item | Status | How |
|---|---|---|
| Positioning | Met | CSS anchor positioning flips it when it would leave the screen. |
| Timeout | Not yet | Opens at once on hover. |
| Keyboard support | Met | Opens on focus, Escape closes. |
