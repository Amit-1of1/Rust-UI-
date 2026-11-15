//! `atoll port`: turn shadcn/ui JSX (a `.tsx` file or a snippet) into an Atoll `rsx!` starting
//! point, and say exactly what it could not carry over.
//!
//! It is a translator for markup, not for logic: every JSX tree in the file becomes one
//! `rsx!` block. shadcn components map to the kit's (with `variant`/`size` strings turned into
//! the kit's enums), Lucide icons keep their names (the kit generates the same set), and the
//! Tailwind classes it knows become inline styles on the kit's tokens. Hooks, handlers,
//! `.map()` lists and unknown classes are left as `// port:` comments and listed in the report,
//! so nothing is dropped silently. docs/PORTING.md has the walkthrough.

use std::collections::BTreeSet;
use std::fmt::Write as _;

/// A string prop that is an enum in the kit: (prop, enum type, [(shadcn value, variant)]).
type EnumProp = (
    &'static str,
    &'static str,
    &'static [(&'static str, &'static str)],
);

/// One shadcn component and the kit component it becomes.
struct Map {
    /// The JSX tag (shadcn's export name).
    from: &'static str,
    /// The kit's component (or `""` to drop the tag and keep its children).
    to: &'static str,
    /// The `atoll add` name of the kit component.
    add: &'static str,
    /// String props that are enums in the kit: (prop, enum type, [(shadcn value, variant)]).
    enums: &'static [EnumProp],
}

const BUTTON_ENUMS: &[EnumProp] = &[
    (
        "variant",
        "ButtonVariant",
        &[
            ("default", "Primary"),
            ("secondary", "Secondary"),
            ("destructive", "Destructive"),
            ("outline", "Outline"),
            ("ghost", "Ghost"),
            ("link", "Link"),
        ],
    ),
    (
        "size",
        "ButtonSize",
        &[
            ("default", "Default"),
            ("sm", "Sm"),
            ("lg", "Lg"),
            ("xs", "Xs"),
        ],
    ),
];

const BADGE_ENUMS: &[EnumProp] = &[(
    "variant",
    "BadgeVariant",
    &[
        ("default", "Primary"),
        ("secondary", "Secondary"),
        ("destructive", "Destructive"),
        ("outline", "Outline"),
    ],
)];

const ALERT_ENUMS: &[EnumProp] = &[(
    "variant",
    "AlertVariant",
    &[("default", "Default"), ("destructive", "Destructive")],
)];

const fn m(from: &'static str, to: &'static str, add: &'static str) -> Map {
    Map {
        from,
        to,
        add,
        enums: &[],
    }
}

/// shadcn's component names and the kit's. Checked against the kit's sources by a test.
const MAPS: &[Map] = &[
    Map {
        from: "Button",
        to: "Button",
        add: "button",
        enums: BUTTON_ENUMS,
    },
    Map {
        from: "Badge",
        to: "Badge",
        add: "badge",
        enums: BADGE_ENUMS,
    },
    Map {
        from: "Alert",
        to: "Alert",
        add: "alert",
        enums: ALERT_ENUMS,
    },
    m("AlertTitle", "AlertTitle", "alert"),
    m("AlertDescription", "AlertDescription", "alert"),
    m("Card", "Card", "card"),
    m("CardHeader", "CardHeader", "card"),
    m("CardTitle", "CardTitle", "card"),
    m("CardDescription", "CardDescription", "card"),
    m("CardAction", "CardAction", "card"),
    m("CardContent", "CardContent", "card"),
    m("CardFooter", "CardFooter", "card"),
    m("Input", "Input", "input"),
    m("Label", "Label", "label"),
    m("Textarea", "Textarea", "textarea"),
    m("Checkbox", "Checkbox", "checkbox"),
    m("Switch", "Switch", "switch"),
    m("Separator", "Separator", "separator"),
    m("Skeleton", "Skeleton", "skeleton"),
    m("Progress", "Progress", "progress"),
    m("Avatar", "Avatar", "avatar"),
    m("AvatarImage", "AvatarImage", "avatar"),
    m("AvatarFallback", "AvatarFallback", "avatar"),
    m("Tabs", "Tabs", "tabs"),
    m("TabsList", "TabList", "tabs"),
    m("TabsTrigger", "TabTrigger", "tabs"),
    m("TabsContent", "TabContent", "tabs"),
    m("Accordion", "Accordion", "accordion"),
    m("AccordionItem", "AccordionItem", "accordion"),
    m("AccordionTrigger", "AccordionTrigger", "accordion"),
    m("AccordionContent", "AccordionContent", "accordion"),
    m("Tooltip", "Tooltip", "tooltip"),
    m("TooltipTrigger", "TooltipTrigger", "tooltip"),
    m("TooltipContent", "TooltipContent", "tooltip"),
    m("TooltipProvider", "", "tooltip"),
    m("Popover", "PopoverRoot", "popover"),
    m("PopoverTrigger", "PopoverTrigger", "popover"),
    m("PopoverContent", "PopoverContent", "popover"),
    m("DropdownMenu", "DropdownMenu", "dropdown_menu"),
    m(
        "DropdownMenuTrigger",
        "DropdownMenuTrigger",
        "dropdown_menu",
    ),
    m(
        "DropdownMenuContent",
        "DropdownMenuContent",
        "dropdown_menu",
    ),
    m("DropdownMenuItem", "DropdownMenuItem", "dropdown_menu"),
    m("Table", "Table", "table"),
    m("TableCell", "TableCell", "table"),
    m("TableHead", "TableHead", "table"),
    m("Kbd", "Kbd", "kbd"),
    m("Toggle", "Toggle", "toggle"),
    m("Slider", "Slider", "slider"),
    m("ScrollArea", "ScrollArea", "scroll_area"),
    m("AspectRatio", "AspectRatio", "aspect_ratio"),
    m("Collapsible", "Collapsible", "collapsible"),
    m("HoverCard", "HoverCard", "hover_card"),
    m("Spinner", "Spinner", "spinner"),
];

/// shadcn's table parts that are plain elements in the kit's `Table`.
const TABLE_ELEMENTS: &[(&str, &str)] = &[
    ("TableHeader", "thead"),
    ("TableBody", "tbody"),
    ("TableFooter", "tfoot"),
    ("TableRow", "tr"),
    ("TableCaption", "caption"),
];

/// What `port` produced for one file.
#[derive(Debug, Default)]
pub struct Ported {
    /// Rust source: `use` lines, then one `rsx!` block per JSX tree.
    pub code: String,
    /// The kit components to `atoll add`, sorted.
    pub add: BTreeSet<String>,
    /// Tailwind classes with no token mapping (left as comments).
    pub unmapped_classes: BTreeSet<String>,
    /// Everything else a person has to finish by hand, in source order.
    pub todo: Vec<String>,
    /// How many JSX trees were found.
    pub trees: usize,
}

impl Ported {
    /// The human report printed after the code.
    pub fn report(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "Ported {} JSX tree(s).", self.trees);
        if !self.add.is_empty() {
            let names: Vec<_> = self.add.iter().map(String::as_str).collect();
            let _ = writeln!(out, "Copy the components in: atoll add {}", names.join(" "));
        }
        if !self.unmapped_classes.is_empty() {
            let names: Vec<_> = self.unmapped_classes.iter().map(String::as_str).collect();
            let _ = writeln!(
                out,
                "Classes with no token mapping (left as comments): {}",
                names.join(" ")
            );
        }
        if self.todo.is_empty() {
            let _ = writeln!(out, "Nothing left to finish by hand.");
        } else {
            let _ = writeln!(out, "Finish by hand ({}):", self.todo.len());
            for t in &self.todo {
                let _ = writeln!(out, "  - {t}");
            }
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq)]
enum AttrValue {
    Str(String),
    Expr(String),
    Flag,
}

#[derive(Debug, Clone, PartialEq)]
enum Node {
    Element {
        name: String,
        attrs: Vec<(String, AttrValue)>,
        children: Vec<Node>,
    },
    Text(String),
    Expr(String),
    Comment(String),
}

struct Parser<'a> {
    s: &'a [u8],
    src: &'a str,
    i: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }
    fn ws(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_ascii_whitespace()) {
            self.i += 1;
        }
    }
    fn eat(&mut self, t: &str) -> bool {
        if self.src[self.i..].starts_with(t) {
            self.i += t.len();
            true
        } else {
            false
        }
    }
    fn ident(&mut self) -> Option<String> {
        let start = self.i;
        while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || b"_-.:$".contains(&c)) {
            self.i += 1;
        }
        (self.i > start).then(|| self.src[start..self.i].to_string())
    }

    /// Skips a JS string or template literal starting at `self.i`.
    fn skip_string(&mut self) {
        let q = self.s[self.i];
        self.i += 1;
        while let Some(c) = self.peek() {
            self.i += 1;
            if c == b'\\' {
                self.i += 1;
            } else if c == q {
                return;
            }
        }
    }

    /// Reads a balanced `{ ... }` and returns what is inside.
    fn braced(&mut self) -> Option<String> {
        if self.peek() != Some(b'{') {
            return None;
        }
        let start = self.i + 1;
        let mut depth = 0usize;
        while let Some(c) = self.peek() {
            match c {
                b'"' | b'\'' | b'`' => {
                    self.skip_string();
                    continue;
                }
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        self.i += 1;
                        return Some(self.src[start..self.i - 1].trim().to_string());
                    }
                }
                _ => {}
            }
            self.i += 1;
        }
        None
    }

    fn element(&mut self) -> Option<Node> {
        if !self.eat("<") {
            return None;
        }
        // Fragment.
        if self.eat(">") {
            let children = self.children("")?;
            return Some(Node::Element {
                name: String::new(),
                attrs: vec![],
                children,
            });
        }
        let name = self.ident()?;
        let mut attrs = Vec::new();
        loop {
            self.ws();
            if self.eat("/>") {
                return Some(Node::Element {
                    name,
                    attrs,
                    children: vec![],
                });
            }
            if self.eat(">") {
                break;
            }
            if self.peek() == Some(b'{') {
                let e = self.braced()?;
                attrs.push((String::from("{spread}"), AttrValue::Expr(e)));
                continue;
            }
            let key = self.ident()?;
            self.ws();
            if self.eat("=") {
                self.ws();
                match self.peek()? {
                    b'"' | b'\'' => {
                        let start = self.i + 1;
                        self.skip_string();
                        attrs.push((key, AttrValue::Str(self.src[start..self.i - 1].to_string())));
                    }
                    b'{' => {
                        let e = self.braced()?;
                        attrs.push((key, AttrValue::Expr(e)));
                    }
                    _ => return None,
                }
            } else {
                attrs.push((key, AttrValue::Flag));
            }
        }
        let children = self.children(&name)?;
        Some(Node::Element {
            name,
            attrs,
            children,
        })
    }

    fn children(&mut self, name: &str) -> Option<Vec<Node>> {
        let mut out = Vec::new();
        let close = format!("</{name}>");
        loop {
            let start = self.i;
            while matches!(self.peek(), Some(c) if c != b'<' && c != b'{') {
                self.i += 1;
            }
            let text = collapse(&self.src[start..self.i]);
            if !text.is_empty() {
                out.push(Node::Text(text));
            }
            match self.peek()? {
                b'{' => {
                    let e = self.braced()?;
                    if let Some(c) = e.strip_prefix("/*").and_then(|c| c.strip_suffix("*/")) {
                        out.push(Node::Comment(c.trim().to_string()));
                    } else if let Some(lit) = js_string(&e) {
                        // `{" "}` and `{"{"}`: literal text.
                        match out.last_mut() {
                            Some(Node::Text(t)) => t.push_str(&lit),
                            _ => out.push(Node::Text(lit)),
                        }
                    } else if !e.is_empty() {
                        out.push(Node::Expr(e));
                    }
                }
                _ => {
                    if self.src[self.i..].starts_with("</") {
                        // Accept `</ Name >` spacing.
                        let rest: String = self.src[self.i..]
                            .chars()
                            .take_while(|&c| c != '>')
                            .filter(|c| !c.is_whitespace())
                            .collect();
                        if format!("{rest}>") != close {
                            return None;
                        }
                        self.i += self.src[self.i..].find('>')? + 1;
                        return Some(out);
                    }
                    out.push(self.element()?);
                }
            }
        }
    }
}

/// The value of a plain JS string literal (no escapes or interpolation), if `e` is one.
fn js_string(e: &str) -> Option<String> {
    let q = e.chars().next()?;
    if !matches!(q, '"' | '\'' | '`') || e.len() < 2 || !e.ends_with(q) {
        return None;
    }
    let inner = &e[1..e.len() - 1];
    (!inner.contains(q) && !inner.contains('\\') && !inner.contains("${"))
        .then(|| inner.to_string())
}

fn collapse(t: &str) -> String {
    let words: Vec<&str> = t.split_whitespace().collect();
    if words.is_empty() {
        return String::new();
    }
    let mut s = words.join(" ");
    // Keep a single space where the text touches an expression or element.
    if t.starts_with(char::is_whitespace) && !t.trim_start().is_empty() && !t.contains('\n') {
        s.insert(0, ' ');
    }
    if t.ends_with(char::is_whitespace) && !t.contains('\n') {
        s.push(' ');
    }
    s
}

/// Finds the JSX trees in a file: a `<` that starts an element where an expression can start.
fn trees(src: &str) -> Vec<Node> {
    let bytes = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'/' && bytes.get(i + 1) == Some(&b'/') {
            i += src[i..].find('\n').unwrap_or(src.len() - i);
            continue;
        }
        if c == b'"' || c == b'\'' || c == b'`' {
            let mut p = Parser { s: bytes, src, i };
            p.skip_string();
            i = p.i;
            continue;
        }
        if c == b'<' && starts_expression(src, i) {
            let next = bytes.get(i + 1).copied().unwrap_or(b' ');
            if next.is_ascii_alphabetic() || next == b'>' {
                let mut p = Parser { s: bytes, src, i };
                if let Some(node) = p.element() {
                    out.push(node);
                    i = p.i;
                    continue;
                }
            }
        }
        i += 1;
    }
    out
}

fn starts_expression(src: &str, i: usize) -> bool {
    let before = src[..i].trim_end();
    before.is_empty()
        || before.ends_with("return")
        || before.ends_with("=>")
        || matches!(
            before.chars().last(),
            Some('(' | '?' | ':' | ',' | '=' | '{' | '[' | ';')
        )
        || before.ends_with("&&")
        || before.ends_with("||")
}

/// Tailwind spacing step `n` as a CSS length on the kit's unit tokens.
fn space(n: &str) -> Option<String> {
    if n == "px" {
        return Some("1px".into());
    }
    let v: f64 = n.parse().ok()?;
    let tokens = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 16, 20, 24];
    if v == 0.0 {
        Some("0".into())
    } else if v.fract() == 0.0 && tokens.contains(&(v as i64)) {
        Some(format!("var(--unit-x{})", v as i64))
    } else {
        Some(format!("{}rem", v * 0.25))
    }
}

/// One Tailwind class as CSS declarations on the kit's tokens, or `None` if unknown.
fn class_css(c: &str) -> Option<String> {
    let fixed = match c {
        "flex" => "display: flex",
        "inline-flex" => "display: inline-flex",
        "grid" => "display: grid",
        "block" => "display: block",
        "inline-block" => "display: inline-block",
        "hidden" => "display: none",
        "flex-col" => "flex-direction: column",
        "flex-row" => "flex-direction: row",
        "flex-wrap" => "flex-wrap: wrap",
        "flex-1" => "flex: 1 1 0%",
        "shrink-0" => "flex-shrink: 0",
        "items-center" => "align-items: center",
        "items-start" => "align-items: flex-start",
        "items-end" => "align-items: flex-end",
        "justify-between" => "justify-content: space-between",
        "justify-center" => "justify-content: center",
        "justify-end" => "justify-content: flex-end",
        "justify-start" => "justify-content: flex-start",
        "w-full" => "width: 100%",
        "h-full" => "height: 100%",
        "min-h-screen" => "min-height: 100vh",
        "mx-auto" => "margin-inline: auto",
        "ml-auto" => "margin-inline-start: auto",
        "text-center" => "text-align: center",
        "text-left" | "text-start" => "text-align: start",
        "text-right" | "text-end" => "text-align: end",
        "text-xs" => "font-size: var(--font-size-caption-1); line-height: var(--line-height-caption-1)",
        "text-sm" => "font-size: var(--font-size-body-2); line-height: var(--line-height-body-2)",
        "text-base" => "font-size: var(--font-size-body-1); line-height: var(--line-height-body-1)",
        "text-lg" => "font-size: var(--font-size-title-4); line-height: var(--line-height-title-4)",
        "text-xl" => "font-size: var(--font-size-title-3); line-height: var(--line-height-title-3)",
        "text-2xl" => "font-size: var(--font-size-title-2); line-height: var(--line-height-title-2)",
        "text-3xl" | "text-4xl" => {
            "font-size: var(--font-size-title-1); line-height: var(--line-height-title-1)"
        }
        "font-normal" => "font-weight: var(--font-weight-regular)",
        "font-medium" => "font-weight: var(--font-weight-medium)",
        "font-semibold" => "font-weight: var(--font-weight-semibold)",
        "font-bold" => "font-weight: var(--font-weight-bold)",
        "font-mono" => "font-family: var(--font-mono)",
        "tracking-tight" => "letter-spacing: var(--heading-tracking)",
        "leading-none" => "line-height: 1",
        "underline" => "text-decoration: underline",
        "truncate" => "overflow: hidden; text-overflow: ellipsis; white-space: nowrap",
        "text-muted-foreground" => "color: var(--color-foreground-neutral-faded)",
        "text-foreground" => "color: var(--color-foreground-neutral)",
        "text-primary" => "color: var(--color-foreground-primary)",
        "text-destructive" => "color: var(--color-foreground-critical)",
        "bg-background" => "background: var(--color-background-page)",
        "bg-muted" => "background: var(--color-background-neutral-faded)",
        "bg-card" => "background: var(--color-background-elevation-base)",
        "bg-primary" => "background: var(--color-background-primary)",
        "border" => "border: 1px solid var(--color-border-neutral-faded)",
        "border-t" => "border-top: 1px solid var(--color-border-neutral-faded)",
        "border-b" => "border-bottom: 1px solid var(--color-border-neutral-faded)",
        "rounded" | "rounded-sm" => "border-radius: var(--radius-small)",
        "rounded-md" => "border-radius: var(--radius-medium)",
        "rounded-lg" | "rounded-xl" => "border-radius: var(--radius-large)",
        "rounded-full" => "border-radius: var(--radius-full)",
        "shadow" | "shadow-sm" => "box-shadow: var(--shadow-raised)",
        "shadow-md" | "shadow-lg" => "box-shadow: var(--shadow-overlay)",
        "relative" => "position: relative",
        "absolute" => "position: absolute",
        "sticky" => "position: sticky",
        "overflow-hidden" => "overflow: hidden",
        "overflow-auto" => "overflow: auto",
        "sr-only" => {
            "position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap"
        }
        _ => "",
    };
    if !fixed.is_empty() {
        return Some(fixed.to_string());
    }
    let (prefix, n) = c.rsplit_once('-')?;
    let prop = match prefix {
        "gap" => "gap",
        "gap-x" => "column-gap",
        "gap-y" => "row-gap",
        "p" => "padding",
        "px" => "padding-inline",
        "py" => "padding-block",
        "pt" => "padding-top",
        "pb" => "padding-bottom",
        "pl" | "ps" => "padding-inline-start",
        "pr" | "pe" => "padding-inline-end",
        "m" => "margin",
        "mx" => "margin-inline",
        "my" => "margin-block",
        "mt" => "margin-top",
        "mb" => "margin-bottom",
        "ml" | "ms" => "margin-inline-start",
        "mr" | "me" => "margin-inline-end",
        "w" => "width",
        "h" => "height",
        "size" => "size",
        "max-w" => {
            let w = match n {
                "sm" => "24rem",
                "md" => "28rem",
                "lg" => "32rem",
                "xl" => "36rem",
                "2xl" => "42rem",
                "prose" => "var(--text-measure)",
                _ => return None,
            };
            return Some(format!("max-width: {w}"));
        }
        "grid-cols" => {
            let k: u32 = n.parse().ok()?;
            return Some(format!(
                "grid-template-columns: repeat({k}, minmax(0, 1fr))"
            ));
        }
        "space-y" => {
            // Tailwind adds margin between children; a column flex with a gap reads the same.
            return Some(format!(
                "display: flex; flex-direction: column; gap: {}",
                space(n)?
            ));
        }
        "space-x" => return Some(format!("display: flex; gap: {}", space(n)?)),
        _ => return None,
    };
    let v = space(n)?;
    if prop == "size" {
        return Some(format!("width: {v}; height: {v}"));
    }
    Some(format!("{prop}: {v}"))
}

/// Splits a `className` into CSS for the classes it knows and the classes it doesn't.
/// Responsive and state prefixes (`md:`, `hover:`, `dark:`) are never mapped: the kit's
/// components and theme already handle those, and inline styles can't express them.
fn classes(cls: &str) -> (Vec<String>, Vec<String>) {
    let mut css = Vec::new();
    let mut left = Vec::new();
    for c in cls.split_whitespace() {
        match (!c.contains(':')).then(|| class_css(c)).flatten() {
            Some(d) => css.push(d),
            None => left.push(c.to_string()),
        }
    }
    (css, left)
}

/// Lucide sizes from `h-4 w-4` / `size-4` on an icon.
fn icon_size(cls: &str) -> Option<String> {
    cls.split_whitespace().find_map(|c| {
        let n = c.strip_prefix("size-").or_else(|| c.strip_prefix("h-"))?;
        let v: f64 = n.parse().ok()?;
        Some(format!("{}rem", v * 0.25))
    })
}

fn pascal_to_snake(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn rust_str(s: &str) -> String {
    let escaped = s
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('{', "{{")
        .replace('}', "}}");
    format!("\"{escaped}\"")
}

fn is_ident_path(e: &str) -> bool {
    !e.is_empty()
        && e.chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && e.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// HTML attribute names in JSX and in Dioxus.
fn html_attr(k: &str) -> String {
    match k {
        "htmlFor" => "r#for".into(),
        "type" => "r#type".into(),
        "tabIndex" => "tabindex".into(),
        "readOnly" => "readonly".into(),
        "autoComplete" => "autocomplete".into(),
        "autoFocus" => "autofocus".into(),
        "maxLength" => "maxlength".into(),
        "minLength" => "minlength".into(),
        "srcSet" => "srcset".into(),
        k if k.contains('-') => format!("\"{k}\""),
        k => pascal_to_snake(k),
    }
}

/// A JSX prop name as a Dioxus field: the kit's components take `html_for`, elements `r#for`.
fn attr_key(k: &str, is_component: bool) -> String {
    if is_component && k == "htmlFor" {
        "html_for".into()
    } else {
        html_attr(k)
    }
}

struct Out<'a> {
    p: &'a mut Ported,
    icons: bool,
    uses_icons: BTreeSet<String>,
    uses_enums: BTreeSet<String>,
}

impl Out<'_> {
    fn node(&mut self, n: &Node, depth: usize, out: &mut String) {
        let pad = "    ".repeat(depth);
        match n {
            Node::Text(t) => {
                let _ = writeln!(out, "{pad}{}", rust_str(t));
            }
            Node::Comment(c) => {
                for line in c.lines() {
                    let _ = writeln!(out, "{pad}// {}", line.trim());
                }
            }
            Node::Expr(e) if is_ident_path(e) => {
                let _ = writeln!(out, "{pad}\"{{{e}}}\"");
            }
            Node::Expr(e) => {
                let short = one_line(e);
                if e.contains(".map(") {
                    self.p
                        .todo
                        .push(format!("list `{short}`: write it as a `for` loop in rsx!"));
                } else {
                    self.p.todo.push(format!("expression `{short}`"));
                }
                let _ = writeln!(out, "{pad}// port: {{{short}}}");
            }
            Node::Element {
                name,
                attrs,
                children,
            } => self.element(name, attrs, children, depth, out),
        }
    }

    fn element(
        &mut self,
        name: &str,
        attrs: &[(String, AttrValue)],
        children: &[Node],
        depth: usize,
        out: &mut String,
    ) {
        let pad = "    ".repeat(depth);
        let first = name.chars().next().unwrap_or('a');
        let map = MAPS.iter().find(|m| m.from == name);
        let table_el = TABLE_ELEMENTS
            .iter()
            .find(|(f, _)| *f == name)
            .map(|(_, t)| *t);
        let class = attrs.iter().find_map(|(k, v)| match (k.as_str(), v) {
            ("className" | "class", AttrValue::Str(s)) => Some(s.as_str()),
            _ => None,
        });

        // Fragments and wrappers the kit doesn't need: keep the children.
        if name.is_empty() || map.is_some_and(|m| m.to.is_empty()) {
            if let Some(m) = map {
                self.p.add.insert(m.add.to_string());
            }
            for c in children {
                self.node(c, depth, out);
            }
            return;
        }

        // Lucide icons keep their names: the kit generates the same set.
        if self.icons
            && first.is_ascii_uppercase()
            && map.is_none()
            && table_el.is_none()
            && children.is_empty()
        {
            self.uses_icons.insert(name.to_string());
            let mut fields = Vec::new();
            if let Some(size) = class.and_then(icon_size) {
                fields.push(format!("size: \"{size}\""));
            }
            fields.push("\"aria-hidden\": \"true\"".into());
            let _ = writeln!(out, "{pad}{name} {{ {} }}", fields.join(", "));
            return;
        }

        let tag = if let Some(m) = map {
            self.p.add.insert(m.add.to_string());
            m.to.to_string()
        } else if let Some(t) = table_el {
            self.p.add.insert("table".into());
            t.to_string()
        } else if first.is_ascii_uppercase() {
            self.p.todo.push(format!(
                "<{name}> has no Atoll counterpart; ported as a div"
            ));
            "div".to_string()
        } else {
            name.to_string()
        };
        let is_component = first.is_ascii_uppercase() && table_el.is_none();

        let mut fields: Vec<String> = Vec::new();
        let mut notes: Vec<String> = Vec::new();
        for (k, v) in attrs {
            match (k.as_str(), v) {
                ("className" | "class", AttrValue::Str(s)) => {
                    let (css, left) = classes(s);
                    if !css.is_empty() {
                        fields.push(format!("style: {}", rust_str(&css.join("; "))));
                    }
                    for c in &left {
                        self.p.unmapped_classes.insert(c.clone());
                    }
                    if !left.is_empty() {
                        notes.push(format!("unmapped classes: {}", left.join(" ")));
                    }
                }
                ("className" | "class", AttrValue::Expr(e)) => {
                    self.p
                        .todo
                        .push(format!("className expression `{}`", one_line(e)));
                    notes.push(format!("className={{{}}}", one_line(e)));
                }
                ("asChild", _) => {
                    notes
                        .push("asChild: put the child's content straight in this component".into());
                    self.p.todo.push(format!("<{name} asChild>: the kit's {tag} renders its own element; move the child's props onto it"));
                }
                ("key", _) => {}
                ("{spread}", AttrValue::Expr(e)) => {
                    self.p
                        .todo
                        .push(format!("spread `{{{}}}` on <{name}>", one_line(e)));
                    notes.push(format!("{{{}}}", one_line(e)));
                }
                (k, v) if k.starts_with("on") && k.len() > 2 => {
                    let ev = k.to_ascii_lowercase();
                    let body = match v {
                        AttrValue::Expr(e) => one_line(e),
                        _ => String::new(),
                    };
                    self.p
                        .todo
                        .push(format!("handler {k} on <{name}>: `{body}`"));
                    fields.push(format!(
                        "{ev}: move |_| {{ /* port: {} */ }}",
                        body.replace("*/", "* /")
                    ));
                }
                (k, AttrValue::Str(s)) => {
                    if let Some((_, ty, values)) =
                        map.and_then(|m| m.enums.iter().find(|(p, _, _)| *p == k))
                    {
                        if let Some((_, variant)) = values.iter().find(|(f, _)| *f == s) {
                            self.uses_enums.insert(ty.to_string());
                            if *variant != "Default" && *variant != "Primary" {
                                fields.push(format!("{k}: {ty}::{variant}"));
                            }
                        } else if name == "Button" && k == "size" && s == "icon" {
                            self.p.todo.push("<Button size=\"icon\">: use IconButton with a `label` (its accessible name)".into());
                        } else {
                            self.p
                                .todo
                                .push(format!("{k}=\"{s}\" on <{name}> has no {ty} variant"));
                        }
                        continue;
                    }
                    let key = attr_key(k, is_component);
                    fields.push(format!("{key}: {}", rust_str(s)));
                }
                (k, AttrValue::Flag) => {
                    let key = attr_key(k, is_component);
                    fields.push(format!("{key}: true"));
                }
                (k, AttrValue::Expr(e)) => {
                    let key = attr_key(k, is_component);
                    let e1 = one_line(e);
                    if e1 == "true"
                        || e1 == "false"
                        || e1.parse::<f64>().is_ok()
                        || is_ident_path(&e1)
                    {
                        fields.push(format!("{key}: {e1}"));
                    } else {
                        self.p.todo.push(format!("prop {k}={{{e1}}} on <{name}>"));
                        notes.push(format!("{k}={{{e1}}}"));
                    }
                }
            }
        }

        for n in &notes {
            let _ = writeln!(out, "{pad}// port: {n}");
        }
        let inline =
            children.len() == 1 && matches!(children[0], Node::Text(_)) && fields.len() <= 2;
        let mut head = format!("{pad}{tag} {{");
        if !fields.is_empty() {
            head.push(' ');
            head.push_str(&fields.join(", "));
        }
        if children.is_empty() {
            let _ = writeln!(out, "{head} }}");
        } else if inline {
            let Node::Text(t) = &children[0] else {
                unreachable!()
            };
            let sep = if fields.is_empty() { " " } else { ", " };
            let _ = writeln!(out, "{head}{sep}{} }}", rust_str(t));
        } else {
            if !fields.is_empty() {
                head.push(',');
            }
            let _ = writeln!(out, "{head}");
            for c in children {
                self.node(c, depth + 1, out);
            }
            let _ = writeln!(out, "{pad}}}");
        }
    }
}

fn one_line(e: &str) -> String {
    e.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Ports a `.tsx`/`.jsx` file or snippet.
pub fn port(src: &str) -> Ported {
    let mut p = Ported::default();
    let found = trees(src);
    p.trees = found.len();
    let icons = src.contains("lucide-react") || !src.contains("import ");
    let mut o = Out {
        p: &mut p,
        icons,
        uses_icons: BTreeSet::new(),
        uses_enums: BTreeSet::new(),
    };
    let mut body = String::new();
    for (i, t) in found.iter().enumerate() {
        if i > 0 {
            body.push('\n');
        }
        body.push_str("rsx! {\n");
        o.node(t, 1, &mut body);
        body.push_str("}\n");
    }
    let (icons_used, enums) = (o.uses_icons, o.uses_enums);
    for hook in ["useState", "useEffect", "useMemo", "useRef", "useForm"] {
        if src.contains(&format!("{hook}(")) || src.contains(&format!("{hook}<")) {
            p.todo.push(format!(
                "{hook}: rewrite with Dioxus hooks (use_signal, use_effect, use_memo)"
            ));
        }
    }
    let mut code = String::from("use dioxus::prelude::*;\n");
    for c in &p.add {
        let _ = writeln!(code, "use crate::components::{c}::*;");
    }
    if !icons_used.is_empty() {
        let names: Vec<_> = icons_used.iter().map(String::as_str).collect();
        let _ = writeln!(
            code,
            "use crate::components::icons::{{{}}};",
            names.join(", ")
        );
    }
    let _ = enums; // Enum types come in with their component's glob import.
    code.push('\n');
    code.push_str(&body);
    p.code = code;
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOGIN: &str = r#"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Mail } from "lucide-react"

export function LoginForm() {
  const [email, setEmail] = useState("")
  return (
    <Card className="w-full max-w-sm">
      <CardHeader>
        <CardTitle className="text-2xl">Log in</CardTitle>
        <CardDescription>Enter your email below to log in.</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <div className="grid gap-2">
          <Label htmlFor="email">Email</Label>
          <Input id="email" type="email" placeholder="m@example.com" required />
        </div>
      </CardContent>
      <CardFooter className="flex-col gap-2 lg:flex-row">
        <Button className="w-full" onClick={() => signIn(email)}>
          <Mail className="mr-2 h-4 w-4" /> Sign in
        </Button>
        <Button variant="outline" size="sm">Cancel</Button>
      </CardFooter>
    </Card>
  )
}
"#;

    #[test]
    fn login_card_ports() {
        let p = port(LOGIN);
        assert_eq!(p.trees, 1, "{}", p.code);
        let names: Vec<_> = p.add.iter().map(String::as_str).collect();
        assert_eq!(names, ["button", "card", "input", "label"]);
        assert!(
            p.code.contains("use crate::components::icons::{Mail};"),
            "{}",
            p.code
        );
        assert!(
            p.code
                .contains("Card { style: \"width: 100%; max-width: 24rem\","),
            "{}",
            p.code
        );
        assert!(
            p.code.contains("Label { html_for: \"email\", \"Email\" }"),
            "{}",
            p.code
        );
        assert!(
            p.code.contains("Input { id: \"email\", r#type: \"email\""),
            "{}",
            p.code
        );
        assert!(p.code.contains("required: true"), "{}", p.code);
        assert!(
            p.code.contains(
                "Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, \"Cancel\" }"
            ),
            "{}",
            p.code
        );
        assert!(
            p.code
                .contains("Mail { size: \"1rem\", \"aria-hidden\": \"true\" }"),
            "{}",
            p.code
        );
        assert!(p.code.contains("onclick: move |_|"), "{}", p.code);
        assert!(p.code.contains("gap: var(--unit-x4)"), "{}", p.code);
        assert!(p.unmapped_classes.contains("lg:flex-row"));
        assert!(p.todo.iter().any(|t| t.starts_with("useState")));
        assert!(p.todo.iter().any(|t| t.contains("onClick")));
    }

    #[test]
    fn lists_and_fragments_are_reported_not_dropped() {
        let p = port(
            r#"const Items = () => (
  <>
    {/* the list */}
    <ul className="space-y-2">{items.map((i) => <li key={i.id}>{i.name}</li>)}</ul>
    <p>{count}</p>
    <TabsList><TabsTrigger value="a">A</TabsTrigger></TabsList>
  </>
)"#,
        );
        assert_eq!(p.trees, 1);
        assert!(p.code.contains("// the list"), "{}", p.code);
        assert!(p.code.contains("\"{count}\""), "{}", p.code);
        assert!(p.code.contains("TabList {"), "{}", p.code);
        assert!(
            p.code.contains("TabTrigger { value: \"a\", \"A\" }"),
            "{}",
            p.code
        );
        assert!(
            p.todo.iter().any(|t| t.starts_with("list `items.map")),
            "{:?}",
            p.todo
        );
    }

    #[test]
    fn text_with_braces_and_quotes_is_escaped() {
        let p = port(r#"<p>Use "x" or {"{"}</p>"#);
        assert!(p.code.contains(r#"p { "Use \"x\" or {{" }"#), "{}", p.code);
    }

    #[test]
    fn generics_and_comparisons_are_not_jsx() {
        let p = port("const a = useState<string>(\"\"); if (a < b) {}\nconst x = 1;");
        assert_eq!(p.trees, 0, "{}", p.code);
    }

    #[test]
    fn unknown_variants_and_icon_buttons_are_flagged() {
        let p = port(
            r#"<div><Button size="icon"><X /></Button><Badge variant="success">ok</Badge></div>"#,
        );
        assert!(
            p.todo.iter().any(|t| t.contains("IconButton")),
            "{:?}",
            p.todo
        );
        assert!(p.todo.iter().any(|t| t.contains("success")), "{:?}", p.todo);
    }

    /// Every mapping points at a component, enum and variant that exist in the kit.
    #[test]
    fn mappings_match_the_kit() {
        // This file is also built into the docs site (`apps/docs/src/port_page.rs`).
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .map(|d| d.join("crates/ui/src/components"))
            .find(|d| d.is_dir())
            .expect("the kit's components folder");
        for m in MAPS {
            let dir = root.join(m.add);
            let mut src = String::new();
            for e in std::fs::read_dir(&dir).unwrap_or_else(|_| panic!("no component {}", m.add)) {
                let path = e.unwrap().path();
                if path.extension().is_some_and(|x| x == "rs") {
                    src.push_str(&std::fs::read_to_string(path).unwrap());
                }
            }
            if !m.to.is_empty() {
                let found = src.contains(&format!("pub fn {}(", m.to))
                    || src.contains(&format!("pub fn {}<", m.to))
                    || src.contains(&format!("::{};", m.to))
                    || src.contains(&format!("::{},", m.to))
                    || src.contains(&format!(" {},", m.to))
                    || src.contains(&format!("{{{}", m.to));
                assert!(found, "{} is not exported by component {}", m.to, m.add);
            }
            for (_, ty, values) in m.enums {
                assert!(
                    src.contains(&format!("pub enum {ty}")),
                    "{ty} missing in {}",
                    m.add
                );
                let body = &src[src.find(&format!("pub enum {ty}")).unwrap()..];
                let body = &body[..body.find('}').unwrap()];
                for (_, v) in *values {
                    assert!(body.contains(v), "{ty}::{v} missing");
                }
            }
        }
        let lucide = std::fs::read_to_string(root.join("icons/lucide.rs")).unwrap();
        assert!(lucide.contains("pub fn Mail("));
    }
}
