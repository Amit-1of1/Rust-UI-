//! Passing `class` to a component adds to the kit's class instead of replacing it
//! (apps/dashboard/GAPS.md, gap 10). Every case renders a component with `class: "caller-x"`
//! and checks that no element ends up with two `class` attributes (the browser keeps one and
//! drops the other) and that the caller's class sits next to a kit (`dx-`) class.

use dioxus::prelude::*;
use ui::prelude::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

/// The value of every `class` attribute, per opening tag.
fn tags(html: &str) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    for raw in html.split('<').skip(1) {
        if raw.starts_with('/') || raw.starts_with('!') {
            continue;
        }
        let tag = raw.split('>').next().unwrap_or_default();
        let classes = tag
            .split(" class=\"")
            .skip(1)
            .map(|rest| rest.split('"').next().unwrap_or_default().to_string())
            .collect();
        out.push((tag.to_string(), classes));
    }
    out
}

fn check(name: &str, app: fn() -> Element) -> Result<(), String> {
    let html = render(app);
    for (tag, classes) in tags(&html) {
        if classes.len() > 1 {
            return Err(format!("{name}: two class attributes on <{tag}>"));
        }
    }
    let merged = tags(&html).into_iter().any(|(_, classes)| {
        classes.first().is_some_and(|c| {
            let words: Vec<&str> = c.split_whitespace().collect();
            words.contains(&"caller-x") && words.iter().any(|w| w.starts_with("dx-"))
        })
    });
    if merged {
        Ok(())
    } else {
        Err(format!(
            "{name}: caller-x is not next to a kit class in {html}"
        ))
    }
}

macro_rules! cases {
    ($($name:literal => $body:expr),* $(,)?) => {
        &[$(($name, || $body)),*]
    };
}

type Case = (&'static str, fn() -> Element);

const CASES: &[Case] = cases![
    "Accordion" => rsx! { Accordion { class: "caller-x" } },
    "AccordionItem" => rsx! { Accordion { AccordionItem { index: 0usize, class: "caller-x" } } },
    "AccordionTrigger" => rsx! { Accordion { AccordionItem { index: 0usize, AccordionTrigger { class: "caller-x", "Q" } } } },
    "Alert" => rsx! { Alert { class: "caller-x" } },
    "AlertTitle" => rsx! { AlertTitle { class: "caller-x" } },
    "AlertDescription" => rsx! { AlertDescription { class: "caller-x" } },
    "Badge" => rsx! { Badge { class: "caller-x", "b" } },
    "BottomNav" => rsx! { BottomNav { label: "Tabs", class: "caller-x" } },
    "Breadcrumb" => rsx! { Breadcrumb { class: "caller-x" } },
    "BreadcrumbItem" => rsx! { BreadcrumbItem { class: "caller-x" } },
    "BreadcrumbLink" => rsx! { BreadcrumbLink { class: "caller-x" } },
    "BreadcrumbPage" => rsx! { BreadcrumbPage { class: "caller-x" } },
    "Button" => rsx! { Button { class: "caller-x" } },
    "ButtonGroup" => rsx! { ButtonGroup { class: "caller-x" } },
    "ButtonGroupText" => rsx! { ButtonGroupText { class: "caller-x" } },
    "Callout" => rsx! { Callout { class: "caller-x", "c" } },
    "Card" => rsx! { Card { class: "caller-x" } },
    "CardTitle" => rsx! { CardTitle { class: "caller-x" } },
    "Carousel" => rsx! { Carousel { label: "c", class: "caller-x" } },
    "Checkbox" => rsx! { Checkbox { class: "caller-x" } },
    "Chips" => rsx! { Chips { label: "c", class: "caller-x" } },
    "CollapsibleTrigger" => rsx! { Collapsible { CollapsibleTrigger { class: "caller-x" } } },
    "Combobox" => rsx! { Combobox::<String> { class: "caller-x" } },
    "Empty" => rsx! { Empty { class: "caller-x" } },
    "EmptyHeader" => rsx! { EmptyHeader { class: "caller-x" } },
    "EmptyMedia" => rsx! { EmptyMedia { class: "caller-x" } },
    "EmptyTitle" => rsx! { EmptyTitle { class: "caller-x" } },
    "EmptyDescription" => rsx! { EmptyDescription { class: "caller-x" } },
    "EmptyContent" => rsx! { EmptyContent { class: "caller-x" } },
    "Field" => rsx! { Field { class: "caller-x" } },
    "FieldLabel" => rsx! { FieldLabel { html_for: "x", class: "caller-x" } },
    "FieldDescription" => rsx! { FieldDescription { id: "d", class: "caller-x" } },
    "InputGroup" => rsx! { InputGroup { class: "caller-x" } },
    "InputGroupAddon" => rsx! { InputGroupAddon { class: "caller-x" } },
    "InputGroupInput" => rsx! { InputGroup { InputGroupInput { class: "caller-x" } } },
    "Input" => rsx! { Input { class: "caller-x" } },
    "Item" => rsx! { Item { class: "caller-x" } },
    "ItemTitle" => rsx! { ItemTitle { class: "caller-x" } },
    "Kbd" => rsx! { Kbd { class: "caller-x" } },
    "KbdGroup" => rsx! { KbdGroup { class: "caller-x" } },
    "Label" => rsx! { Label { html_for: "x", class: "caller-x" } },
    "TextLink" => rsx! { TextLink { href: "#", class: "caller-x" } },
    "Pagination" => rsx! { Pagination { class: "caller-x" } },
    "PaginationContent" => rsx! { PaginationContent { class: "caller-x" } },
    "PaginationLink" => rsx! { PaginationLink { class: "caller-x" } },
    "Progress" => rsx! { Progress { class: "caller-x" } },
    "RadioGroup" => rsx! { RadioGroup { class: "caller-x" } },
    "RadioItem" => rsx! { RadioGroup { RadioItem { value: "a", index: 0usize, class: "caller-x" } } },
    "Select" => rsx! { Select::<String> { class: "caller-x" } },
    "Separator" => rsx! { Separator { class: "caller-x" } },
    "SheetHeader" => rsx! { SheetHeader { class: "caller-x" } },
    "SheetFooter" => rsx! { SheetFooter { class: "caller-x" } },
    "Skeleton" => rsx! { Skeleton { class: "caller-x" } },
    "Slider" => rsx! { Slider { class: "caller-x" } },
    "Spinner" => rsx! { Spinner { class: "caller-x" } },
    "Status" => rsx! { Status { class: "caller-x", "s" } },
    "Switch" => rsx! { Switch { class: "caller-x" } },
    "Table" => rsx! { Table { class: "caller-x" } },
    "TableRow" => rsx! { TableRow { class: "caller-x" } },
    "TableCell" => rsx! { TableCell { class: "caller-x" } },
    "TableHead" => rsx! { TableHead { class: "caller-x" } },
    "Tabs" => rsx! { Tabs { class: "caller-x" } },
    "TabList" => rsx! { Tabs { TabList { class: "caller-x" } } },
    "Textarea" => rsx! { Textarea { class: "caller-x" } },
    "Toggle" => rsx! { Toggle { class: "caller-x" } },
    "ToggleGroup" => rsx! { ToggleGroup { class: "caller-x" } },
    "Toolbar" => rsx! { Toolbar { aria_label: "t", class: "caller-x" } },
    "Form" => rsx! { Form { state: use_form(), class: "caller-x" } },
    "SidebarInset" => rsx! { SidebarProvider { SidebarInset { class: "caller-x" } } },
    "SidebarContent" => rsx! { SidebarProvider { SidebarContent { class: "caller-x" } } },
    "SidebarMenuButton" => rsx! { SidebarProvider { SidebarMenuButton { class: "caller-x" } } },
    "PopoverRoot" => rsx! { PopoverRoot { class: "caller-x" } },
    "PopoverTrigger" => rsx! { PopoverRoot { PopoverTrigger { class: "caller-x" } } },
    "DropdownMenuTrigger" => rsx! { DropdownMenu { DropdownMenuTrigger { class: "caller-x" } } },
    "TooltipTrigger" => rsx! { Tooltip { TooltipTrigger { class: "caller-x" } } },
    "HoverCardTrigger" => rsx! { HoverCard { HoverCardTrigger { class: "caller-x" } } },
    "Stepper" => rsx! { Stepper { total: 1, class: "caller-x" } },
];

#[test]
fn class_adds_to_the_kit_class() {
    let failures: Vec<String> = CASES
        .iter()
        .filter_map(|(name, app)| check(name, *app).err())
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}
