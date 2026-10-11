# Field

Wires a label, description and error to a control with matching ids. `use_field_ids()` returns the ids; for most forms use `TextField` from `form`.

## Accessibility

- Role and name: the control gets `aria-describedby` pointing at the description and error; `FieldError` renders `role="alert"` only when there is a message
- Keyboard: none of its own
- Reduced motion: none
