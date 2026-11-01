# Spinner

A loading indicator for waits of unknown length. For a known amount of work, use `Progress`, which can show how much is left.

`size` is `Sm`, `Default`, `Lg` or `Xl`. The ring takes the text colour of its parent, so it follows the colour of the button, alert or text it sits in. Give a spinner that stands alone a `label` that says what is loading ("Loading invoices"). Inside a button, use the button's `loading` instead.

## Accessibility

- Role and name: `role="status"` with an `aria-label` (default "Loading")
- Keyboard: none (not interactive)
- Reduced motion: the spin slows to 3s instead of stopping, so it still shows that work is happening
