//! `#[derive(Form)]` for the Atoll kit: one struct gives the form's fields, their validation
//! and their accessible error messages, on top of the kit's `use_form`.
//!
//! Use it through the kit (`use ui::prelude::*;`), not directly: the generated code names the
//! kit's form module (`::ui::components::form` unless `#[form(path = "...")]` says otherwise).
//! The attributes are documented in the kit's `form` docs (`crates/ui/src/components/form/docs.md`).
//!
//! # Stability
//!
//! Experimental: may change in any release (docs/STABILITY.md).
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Expr, Fields, GenericArgument, LitInt,
    LitStr, Path, PathArguments, Type,
};

/// Derive a form from a struct. See the kit's `form` docs for the attributes.
#[proc_macro_derive(Form, attributes(form))]
pub fn derive_form(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// What a field's Rust type means for the form.
enum Kind {
    /// `String`: free text.
    Text,
    /// `bool`: a checkbox, `"true"` while ticked.
    Bool,
    /// `Option<String>`: empty is `None`.
    OptText,
    /// Anything `FromStr + Display` (numbers included): must parse, so it is always required.
    Parsed(Type, Num),
    /// `Option<T>` of the above: empty is `None`, anything else must parse.
    OptParsed(Type, Num),
}

#[derive(Clone, Copy)]
enum Num {
    Int,
    Float,
    No,
}

enum Widget {
    Auto,
    Textarea,
    Select(Path),
    Combobox(Path),
}

struct FieldSpec {
    ident: syn::Ident,
    kind: Kind,
    label: String,
    description: Option<String>,
    placeholder: Option<String>,
    autocomplete: Option<String>,
    input: Option<String>,
    /// `None`: not set. `Some(None)`: the default message.
    required: Option<Option<String>>,
    email: Option<Option<String>>,
    min_len: Option<LitInt>,
    max_len: Option<LitInt>,
    min: Option<Expr>,
    max: Option<Expr>,
    invalid: Option<String>,
    validate: Vec<Path>,
    widget: Widget,
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "#[derive(Form)] does not support generic structs",
        ));
    }
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new(
            input.ident.span(),
            "#[derive(Form)] needs a struct with named fields",
        ));
    };
    let Fields::Named(named) = &data.fields else {
        return Err(syn::Error::new(
            input.ident.span(),
            "#[derive(Form)] needs a struct with named fields",
        ));
    };

    let mut module: Path = syn::parse_quote!(::ui::components::form);
    for attr in input.attrs.iter().filter(|a| a.path().is_ident("form")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("path") {
                let lit: LitStr = meta.value()?.parse()?;
                module = lit.parse()?;
                Ok(())
            } else {
                Err(meta.error("unknown container attribute; expected `path = \"...\"`"))
            }
        })?;
    }

    let specs = named
        .named
        .iter()
        .map(field_spec)
        .collect::<syn::Result<Vec<_>>>()?;

    let vis = &input.vis;
    let name = &input.ident;
    let form_name = format_ident!("{}Form", name);
    let k = &module;
    let d = quote!(#k::__derive);

    let idents: Vec<_> = specs.iter().map(|s| &s.ident).collect();
    let declare = specs.iter().map(|s| {
        let id = &s.ident;
        let rules = rules(s, &d, k);
        let to_field = to_field(&s.kind, quote!(&initial.#id));
        quote! {
            let #id = state.field_with(
                initial.map(|initial| #to_field).unwrap_or_default(),
                ::std::vec![#(#rules),*],
            );
        }
    });
    let read = specs.iter().map(|s| {
        let id = &s.ident;
        let from = from_field(&s.kind);
        quote! { let #id = { let __v = self.#id.value(); #from }?; }
    });
    let load = specs.iter().map(|s| {
        let id = &s.ident;
        let to_field = to_field(&s.kind, quote!(&value.#id));
        quote! { self.#id.load(#to_field); }
    });
    let render = specs.iter().map(render);

    let field_docs = specs.iter().map(|s| format!("The `{}` field.", s.ident));
    let form_doc = format!(
        "The fields of [`{name}`], made by [`{name}::use_form`] (`#[derive(Form)]`). \
         Pass it as `Form`'s `state`, render it with [`fields`](Self::fields), and read the \
         result with [`value`](Self::value)."
    );

    Ok(quote! {
        #[doc = #form_doc]
        #[derive(Clone, Copy, PartialEq)]
        #vis struct #form_name {
            #[doc(hidden)]
            pub __state: #k::FormState,
            #(
                #[doc = #field_docs]
                pub #idents: #k::FormField,
            )*
        }

        impl #name {
            /// Declare the form's fields, empty. A hook: call it at the top of a component.
            #vis fn use_form() -> #form_name {
                Self::__use_form(::std::option::Option::None)
            }

            /// Declare the form's fields, starting from `initial` (what `reset` returns to). A
            /// hook; `initial` is read on the first render only, use `load` to change it later.
            #vis fn use_form_with(initial: &Self) -> #form_name {
                Self::__use_form(::std::option::Option::Some(initial))
            }

            fn __use_form(initial: ::std::option::Option<&Self>) -> #form_name {
                let state = #k::use_form();
                #(#declare)*
                #form_name { __state: state, #(#idents),* }
            }
        }

        impl #form_name {
            /// The underlying `FormState`, for `Form`'s `state` (or pass the form itself: it converts).
            #vis fn state(&self) -> #k::FormState {
                self.__state
            }

            /// The typed value, or `None` while a field does not parse. After a valid submit
            /// (`Form`'s `on_valid`) it is always `Some`.
            #vis fn value(&self) -> ::std::option::Option<#name> {
                #(#read)*
                ::std::option::Option::Some(#name { #(#idents),* })
            }

            /// Load a saved value: every field takes it as its value and its reset point, with
            /// no errors. Safe to call from an effect.
            #vis fn load(&self, value: &#name) {
                #(#load)*
            }

            /// Back to the initial values, with no errors.
            #vis fn reset(&self) {
                self.__state.reset();
            }

            /// Validate every field now (all messages show). `true` when all are valid.
            #vis fn validate(&self) -> bool {
                self.__state.validate_all()
            }

            /// Every field, labelled and wired, in the struct's order.
            #vis fn fields(&self) -> ::dioxus::prelude::Element {
                use #k as __form;
                ::dioxus::prelude::rsx! { #(#render)* }
            }
        }

        impl ::std::convert::From<#form_name> for #k::FormState {
            fn from(form: #form_name) -> Self {
                form.__state
            }
        }
    })
}

fn field_spec(field: &syn::Field) -> syn::Result<FieldSpec> {
    let ident = field.ident.clone().expect("named field");
    let kind = kind(&field.ty);
    let mut spec = FieldSpec {
        label: humanize(&ident.to_string()),
        ident,
        kind,
        description: None,
        placeholder: None,
        autocomplete: None,
        input: None,
        required: None,
        email: None,
        min_len: None,
        max_len: None,
        min: None,
        max: None,
        invalid: None,
        validate: Vec::new(),
        widget: Widget::Auto,
    };
    for attr in field.attrs.iter().filter(|a| a.path().is_ident("form")) {
        attr.parse_nested_meta(|meta| {
            let p = &meta.path;
            let text = |meta: &syn::meta::ParseNestedMeta| -> syn::Result<String> {
                Ok(meta.value()?.parse::<LitStr>()?.value())
            };
            let optional_text = |meta: &syn::meta::ParseNestedMeta| -> syn::Result<Option<String>> {
                if meta.input.peek(syn::Token![=]) {
                    text(meta).map(Some)
                } else {
                    Ok(None)
                }
            };
            if p.is_ident("label") {
                spec.label = text(&meta)?;
            } else if p.is_ident("description") {
                spec.description = Some(text(&meta)?);
            } else if p.is_ident("placeholder") {
                spec.placeholder = Some(text(&meta)?);
            } else if p.is_ident("autocomplete") {
                spec.autocomplete = Some(text(&meta)?);
            } else if p.is_ident("input") {
                spec.input = Some(text(&meta)?);
            } else if p.is_ident("required") {
                spec.required = Some(optional_text(&meta)?);
            } else if p.is_ident("email") {
                spec.email = Some(optional_text(&meta)?);
            } else if p.is_ident("min_len") {
                spec.min_len = Some(meta.value()?.parse()?);
            } else if p.is_ident("max_len") {
                spec.max_len = Some(meta.value()?.parse()?);
            } else if p.is_ident("min") {
                spec.min = Some(meta.value()?.parse()?);
            } else if p.is_ident("max") {
                spec.max = Some(meta.value()?.parse()?);
            } else if p.is_ident("invalid") {
                spec.invalid = Some(text(&meta)?);
            } else if p.is_ident("validate") {
                spec.validate.push(meta.value()?.parse()?);
            } else if p.is_ident("textarea") {
                spec.widget = Widget::Textarea;
            } else if p.is_ident("select") {
                spec.widget = Widget::Select(meta.value()?.parse()?);
            } else if p.is_ident("combobox") {
                spec.widget = Widget::Combobox(meta.value()?.parse()?);
            } else {
                return Err(meta.error(
                    "unknown form attribute; expected one of label, description, placeholder, \
                     autocomplete, input, required, email, min_len, max_len, min, max, invalid, \
                     validate, textarea, select, combobox",
                ));
            }
            Ok(())
        })?;
    }
    check(field, &spec)?;
    Ok(spec)
}

/// Reject attribute combinations that would silently do nothing.
fn check(field: &syn::Field, spec: &FieldSpec) -> syn::Result<()> {
    let err = |msg: &str| Err(syn::Error::new(field.span(), msg));
    let text = matches!(spec.kind, Kind::Text | Kind::OptText);
    let parsed = matches!(spec.kind, Kind::Parsed(..) | Kind::OptParsed(..));
    if matches!(spec.kind, Kind::Bool) && !matches!(spec.widget, Widget::Auto) {
        return err("a `bool` field is always a checkbox");
    }
    if !text && (spec.email.is_some() || spec.min_len.is_some() || spec.max_len.is_some()) {
        return err("`email`, `min_len` and `max_len` need a `String` or `Option<String>` field");
    }
    if !parsed && (spec.min.is_some() || spec.max.is_some() || spec.invalid.is_some()) {
        return err(
            "`min`, `max` and `invalid` need a parsed field (a number, or any `FromStr` type)",
        );
    }
    if matches!(spec.kind, Kind::OptText | Kind::OptParsed(..)) && spec.required.is_some() {
        return err("an `Option` field is optional; drop `required` or the `Option`");
    }
    Ok(())
}

fn kind(ty: &Type) -> Kind {
    if let Some(inner) = option_inner(ty) {
        return match last_ident(inner).as_deref() {
            Some("String") => Kind::OptText,
            _ => Kind::OptParsed(inner.clone(), num(inner)),
        };
    }
    match last_ident(ty).as_deref() {
        Some("String") => Kind::Text,
        Some("bool") => Kind::Bool,
        _ => Kind::Parsed(ty.clone(), num(ty)),
    }
}

fn num(ty: &Type) -> Num {
    match last_ident(ty).as_deref() {
        Some(
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128"
            | "isize",
        ) => Num::Int,
        Some("f32" | "f64") => Num::Float,
        _ => Num::No,
    }
}

fn last_ident(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(p) if p.qself.is_none() => p.path.segments.last().map(|s| s.ident.to_string()),
        _ => None,
    }
}

fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(p) = ty else { return None };
    let seg = p.path.segments.last()?;
    if seg.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    match args.args.first()? {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

/// `first_name` -> `First name`.
fn humanize(ident: &str) -> String {
    let words = ident.trim_start_matches("r#").replace('_', " ");
    let words = words.trim();
    let mut chars = words.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// The field's rules, in the order they are checked.
fn rules(s: &FieldSpec, d: &TokenStream2, k: &Path) -> Vec<TokenStream2> {
    let label = &s.label;
    if matches!(s.kind, Kind::Bool) {
        let mut out = Vec::new();
        if let Some(msg) = &s.required {
            let msg = msg
                .clone()
                .unwrap_or_else(|| format!("Tick \u{201c}{label}\u{201d} to continue."));
            out.push(quote!(#d::checked(#msg)));
        }
        out.extend(s.validate.iter().map(|p| quote!(#k::custom(#p))));
        return out;
    }
    // Parsed non-`Option` types need a value to parse, so they are required even without the flag.
    let required = s
        .required
        .clone()
        .or_else(|| matches!(s.kind, Kind::Parsed(..)).then_some(None));
    let mut checks = Vec::new();
    if let Some(msg) = &s.email {
        checks.push(match msg {
            Some(msg) => quote!(#d::with_message(#k::email(), #msg)),
            None => quote!(#k::email()),
        });
    }
    if let Some(n) = &s.min_len {
        checks.push(quote!(#k::min_len(#n)));
    }
    if let Some(n) = &s.max_len {
        checks.push(quote!(#k::max_len(#n)));
    }
    if let Kind::Parsed(ty, num) | Kind::OptParsed(ty, num) = &s.kind {
        let msg = s.invalid.clone().unwrap_or_else(|| {
            match num {
                Num::Int => "Enter a whole number.",
                Num::Float => "Enter a number.",
                Num::No => "Enter a valid value.",
            }
            .to_string()
        });
        checks.push(quote!(#d::parses::<#ty>(#msg)));
        if let Some(min) = &s.min {
            checks.push(quote!(#d::at_least::<#ty>(#min)));
        }
        if let Some(max) = &s.max {
            checks.push(quote!(#d::at_most::<#ty>(#max)));
        }
    }
    checks.extend(s.validate.iter().map(|p| quote!(#k::custom(#p))));

    let mut out = Vec::new();
    match required {
        Some(msg) => {
            let msg = msg.unwrap_or_else(|| format!("{label} is required."));
            out.push(quote!(#d::required(#msg)));
            out.extend(checks);
        }
        // Optional: an empty value is fine, the other rules only check what was typed.
        None => out.extend(checks.into_iter().map(|c| quote!(#d::when_filled(#c)))),
    }
    out
}

/// `&T` -> the field's string.
fn to_field(kind: &Kind, v: TokenStream2) -> TokenStream2 {
    match kind {
        Kind::Text => quote!(::std::clone::Clone::clone(#v)),
        Kind::Bool => {
            quote!(if *#v { ::std::string::String::from("true") } else { ::std::string::String::new() })
        }
        Kind::OptText => quote!(::std::clone::Clone::clone(#v).unwrap_or_default()),
        Kind::Parsed(..) => quote!(::std::string::ToString::to_string(#v)),
        Kind::OptParsed(..) => {
            quote!((#v).as_ref().map(::std::string::ToString::to_string).unwrap_or_default())
        }
    }
}

/// `__v: String` -> `Option<T>`.
fn from_field(kind: &Kind) -> TokenStream2 {
    match kind {
        Kind::Text => quote!(::std::option::Option::Some(__v)),
        Kind::Bool => quote!(::std::option::Option::Some(__v == "true")),
        Kind::OptText => quote!(::std::option::Option::Some(
            (!__v.trim().is_empty()).then_some(__v)
        )),
        Kind::Parsed(ty, _) => quote!(__v.trim().parse::<#ty>().ok()),
        Kind::OptParsed(ty, _) => quote! {
            if __v.trim().is_empty() {
                ::std::option::Option::Some(::std::option::Option::None)
            } else {
                __v.trim().parse::<#ty>().ok().map(::std::option::Option::Some)
            }
        },
    }
}

/// A string for an rsx prop: rsx treats `{...}` in string literals as formatting.
fn lit(text: &str) -> LitStr {
    LitStr::new(
        &text.replace('{', "{{").replace('}', "}}"),
        Span::call_site(),
    )
}

fn opt_prop(name: &str, value: &Option<String>) -> TokenStream2 {
    match value {
        Some(v) => {
            let name = format_ident!("{}", name);
            let v = lit(v);
            quote!(#name: #v,)
        }
        None => quote!(),
    }
}

fn render(s: &FieldSpec) -> TokenStream2 {
    let id = &s.ident;
    let label = lit(&s.label);
    let description = opt_prop("description", &s.description);
    let placeholder = opt_prop("placeholder", &s.placeholder);
    let required = s.required.is_some() || matches!(s.kind, Kind::Parsed(..));
    let (aria_required, required) = if required {
        (quote!(aria_required: "true",), quote!(required: true,))
    } else {
        (quote!(), quote!())
    };
    if matches!(s.kind, Kind::Bool) {
        return quote! {
            __form::CheckboxField { field: self.#id, label: #label, #description #required }
        };
    }
    match &s.widget {
        Widget::Textarea => quote! {
            __form::TextareaField { field: self.#id, label: #label, #description #placeholder #aria_required }
        },
        Widget::Select(options) => quote! {
            __form::SelectField { field: self.#id, label: #label, options: #options(), #description #placeholder #required }
        },
        Widget::Combobox(options) => quote! {
            __form::ComboboxField { field: self.#id, label: #label, options: #options(), #description #placeholder #required }
        },
        Widget::Auto => {
            let input = s
                .input
                .clone()
                .or_else(|| s.email.is_some().then(|| "email".to_string()));
            let input = match input {
                Some(t) => {
                    let t = lit(&t);
                    quote!(r#type: #t,)
                }
                None => quote!(),
            };
            let inputmode = match &s.kind {
                Kind::Parsed(_, Num::Int) | Kind::OptParsed(_, Num::Int) => {
                    quote!(inputmode: "numeric",)
                }
                Kind::Parsed(_, Num::Float) | Kind::OptParsed(_, Num::Float) => {
                    quote!(inputmode: "decimal",)
                }
                _ => quote!(),
            };
            let autocomplete = opt_prop("autocomplete", &s.autocomplete);
            quote! {
                __form::TextField {
                    field: self.#id,
                    label: #label,
                    #input #description #placeholder #autocomplete #inputmode #aria_required
                }
            }
        }
    }
}
