# Core components checklist

How Atoll's components meet the "Core components" list of [designsystemchecklist.com](https://www.designsystemchecklist.com/category/core-components): 29 components and 166 items, each with what we ship and where. Each component's own reference systems (Reshaped, Radix, MUI, Atlassian, Spectrum, Polaris, Carbon, Mantine, Chakra, Material) were the benchmark for how a missing item should behave. Visual look (colours, fonts) follows the current theme tokens; the kit's visual identity is being chosen separately.

**Met** means a component, prop or documented pattern does it today, covered by the kit's tests (markup snapshots, the ARIA check, contracts, contrast proof). **Partly** names what is missing. **Not yet** is open work. `crates/ui/tests/checklist.rs` keeps the totals below in step with the rows.

### Names next to Reshaped

The owner picked Reshaped as the reference design system. The kit keeps its own names so existing apps keep compiling; this is how they line up:

| Reshaped | Atoll |
|---|---|
| `size`: small, medium, large, xlarge | `Sm`, `Default`, `Lg`, `Xl` (`size` on buttons, badges, avatars, spinners, progress; `field_size` on inputs and text areas) |
| `variant`: solid, outline, faded, ghost | `Default`, `Outline`, `Secondary`, `Ghost` (`Fade` on text areas) |
| `color`: neutral, primary, critical, positive, warning | `Secondary`, `Default`, `Destructive`, `Success`, `Warning` (badges and alerts; `Info` for alerts) |
| `Modal` `size` | `DialogSize::Sm` to `Full` |
| `Toast` `actionsSlot` | `notify_action` |

<!-- checklist-totals -->
| | Met | Partly | Not yet | Total |
|---|---|---|---|---|
| All components | 161 | 4 | 1 | 166 |
<!-- /checklist-totals -->

## Accordion (4)

| Item | Status | How |
|---|---|---|
| Active state | Met | The chevron turns when an item opens; `data-open` styles the item. |
| Composition | Met | `AccordionContent` takes any children. |
| Toggle transition | Met | Height slide on open and close; off under reduced motion. |
| Content and trigger relation | Met | Each panel is a `region` named by its trigger (`aria-labelledby`), which has `aria-controls` and `aria-expanded`. |

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
| Disabled state | Met | `BreadcrumbLink { disabled }` (and `Crumb::disabled`): no `href`, `aria-disabled`, skipped by Tab. |
| Collapsed state | Met | `BreadcrumbTrail { max_items }` keeps the first and last steps with a "Show N more" button that expands the trail and focuses the first revealed step. |
| Custom separator | Met | `BreadcrumbSeparator { ChevronRight {} }`, or `separator` on `BreadcrumbTrail`; "/" by default. |

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
| Media sections | Met | `CardMedia` meets the top or bottom edge; `Card { horizontal: true }` puts it beside the content on wide screens. |
| Supplementary actions | Met | Footer and header actions; `CardLink` makes the whole card open one link while its buttons still work. |
| Responsiveness | Met | Fills its container's width. |
| Card groups | Met | `CardGroup` fits as many columns as there is room for (`min_width`); `ItemGroup` for list rows. |

## Carousel (6)

| Item | Status | How |
|---|---|---|
| Navigation controls | Met | Previous and Next buttons. |
| Item composition | Met | `CarouselItem` takes any children; `media` for full-bleed photos. |
| Item sizes | Met | `item_size` takes any CSS length; the default 80% lets the next slide peek in. |
| Touch navigation | Met | A native scroll area with scroll snapping. |
| Responsiveness | Met | `item_size: "max(40%, 11rem)"` (or `min()`, `clamp()`) adapts the number of slides in view to the space. |
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
| Supplementary actions | Met | `DialogFooter` (end-aligned, stacked on phones); `SheetFooter`. |
| Close action | Met | `show_close` adds a named X; `DialogClose` turns any button into a close action; Escape. |
| Positioning | Met | Centred `Dialog`; `Sheet` slides in from any side; `Drawer` from the bottom. |
| Sizes | Met | `DialogSize::Sm`, `Default`, `Lg`, `Xl`, `Full`; full width on phones. |
| Focus trapping | Met | Focus moves in, stays in, and returns to the opener. |
| Keyboard navigation | Met | Escape closes; Tab cycles inside (keyboard contract). |
| Title and subtitle labeling | Met | `title` is required (`aria-labelledby`), `description` gives `aria-describedby`; `hide_title` keeps them for screen readers only. |

## Pagination (6)

| Item | Status | How |
|---|---|---|
| Selected page state | Met | `aria-current="page"`, highlighted; pressing it does nothing. |
| Page display ranges | Met | `Pager { siblings }` collapses long ranges to "1 … 4 5 6 … 20". |
| Amount of items per page | Met | `Pager { page_sizes, page_size, on_page_size_change }` adds a "Rows per page" select; `DataTable` has its own. |
| Indeterminate amount of pages | Met | `Pager { has_next }` without `pages`: Previous, "Page N", Next. |
| Full page label announcements | Met | Page buttons are named "Page N" (translatable), Previous and Next "Go to previous/next page", the `nav` "Pagination". |
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
| Error state | Met | `aria-invalid` gives a red edge and an error icon; `FieldError` gives the text. |
| Disabled state | Met | `disabled`. |
| Placeholder | Met | `placeholder` while nothing is chosen. |
| Helper text | Met | `SelectField { description }`, in `aria-describedby`. |
| Icon support | Met | `start` puts an icon at the start of the trigger; options take icons too. |
| Prefix | Met | `start` also takes any content, such as the chosen country's flag. |
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
| Equal width layout | Met | `Tabs { stretch: true }`: the list fills the container, tabs share it equally. |
| Keyboard support | Met | Arrow keys, Home and End move between tabs (manual activation: Enter or Space selects; keyboard contract). |

## Text area (7)

| Item | Status | How |
|---|---|---|
| Label | Met | `TextareaField { label }` or `Label { html_for }`. |
| Error state | Met | `aria-invalid` red edge with `FieldError`. |
| Disabled state | Met | `disabled`. |
| Placeholder | Met | `placeholder`, never instead of the label. |
| Helper text | Met | `TextareaField { description }`. |
| Sizes | Met | `field_size: FieldSize::Sm`, `Default`, `Lg`; variants and `autosize` too. |
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
| Sizes | Met | `field_size: FieldSize::Sm`, `Default`, `Lg`, on top of the density setting. |
| Accessibility label | Met | `aria-label` when there is no visible label. |

## Toast (8)

| Item | Status | How |
|---|---|---|
| Content composition | Partly | Title and description; `render_toast` for anything else. |
| Colors | Met | Success, error, warning and info types. |
| Icon support | Met | Each type shows its icon (check, alert, warning, info) before the text. |
| Timeout | Met | 5 seconds by default, `permanent` toasts with a close button. |
| Stacking | Met | Toasts stack and spread out on hover and focus. |
| Supplementary action | Met | `notify_action(kind, message, "Undo", on_action)` adds a button that runs the action and closes the toast. |
| Focus management | Met | F6 jumps to the toasts and their buttons take focus; the timeout pauses while focus or the pointer is inside (`data-paused`). |
| Reduced motion | Met | Animations off (theme rule). |

## Tooltip (3)

| Item | Status | How |
|---|---|---|
| Positioning | Met | CSS anchor positioning flips it when it would leave the screen. |
| Timeout | Met | `delay_ms` (400 by default) before opening on hover; keyboard focus opens at once. |
| Keyboard support | Met | Opens on focus, Escape closes. |
