### Added

- **ui:** `#[derive(Form)]` (new crate `rust-ui-form-derive`, re-exported in `ui::prelude`): a struct's fields become a `use_form` form with typed values, validation attributes (`required`, `email`, `min_len`, `max_len`, `min`, `max`, `validate`) and error messages that name the field. It generates `<Name>Form` with `fields()`, `value()`, `load()`, `reset()` and `validate()`; `bool` fields are checkboxes, numbers must parse, `Option` fields are optional (experimental).
- **ui:** `SelectField`, `ComboboxField` and `CheckboxField` take `required` to mark their control `aria-required`; `Form`'s `state` accepts anything that converts into a `FormState`.
