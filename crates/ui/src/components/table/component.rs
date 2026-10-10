use dioxus::prelude::*;

#[css_module("/src/components/table/style.css")]
struct Styles;

macro_rules! table_part {
    ($(#[$meta:meta])* $name:ident, $tag:ident, $class:ident) => {
        $(#[$meta])*
        #[component]
        pub fn $name(
            #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
            children: Element,
        ) -> Element {
            rsx! {
                $tag { class: Styles::$class, ..attributes, {children} }
            }
        }
    };
}

/// A scrollable wrapper around a native `<table>`, so wide tables don't break the page.
#[component]
pub fn Table(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_table_container,
            table { class: Styles::dx_table, ..attributes, {children} }
        }
    }
}

table_part!(TableHeader, thead, dx_table_header);
table_part!(TableBody, tbody, dx_table_body);
table_part!(TableFooter, tfoot, dx_table_footer);
table_part!(TableRow, tr, dx_table_row);

#[component]
pub fn TableCell(
    #[props(extends=GlobalAttributes, extends=td)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        td { class: Styles::dx_table_cell, ..attributes, {children} }
    }
}
table_part!(TableCaption, caption, dx_table_caption);

/// The direction a column is sorted in.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    /// Next direction when a header is activated: none, ascending, descending, none.
    pub fn cycle(self) -> Self {
        match self {
            Self::None => Self::Ascending,
            Self::Ascending => Self::Descending,
            Self::Descending => Self::None,
        }
    }

    fn aria(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Ascending => "ascending",
            Self::Descending => "descending",
        }
    }

    fn arrow(self) -> &'static str {
        match self {
            Self::None => "↕",
            Self::Ascending => "↑",
            Self::Descending => "↓",
        }
    }
}

/// A column header. Pass `on_sort` to make it a sort button (with `aria-sort`).
#[component]
pub fn TableHead(
    #[props(default)] sort: SortDirection,
    #[props(default)] on_sort: Option<EventHandler<()>>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        th {
            class: Styles::dx_table_head,
            scope: "col",
            aria_sort: if on_sort.is_some() { sort.aria() },
            ..attributes,
            if let Some(on_sort) = on_sort {
                button {
                    class: Styles::dx_table_sort,
                    r#type: "button",
                    onclick: move |_| on_sort.call(()),
                    {children}
                    span { aria_hidden: "true", class: Styles::dx_table_sort_icon, {sort.arrow()} }
                }
            } else {
                {children}
            }
        }
    }
}
