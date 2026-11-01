//! [`Screen`]: one rendered app, driven like a user drives it.

use crate::css;
use crate::keys;
use crate::query::{self, Query};
use blitz_dom::{Document as _, LocalName};
use blitz_traits::events::{
    BlitzKeyEvent, BlitzMouseButtonEvent, KeyState, MouseEventButton, MouseEventButtons, UiEvent,
};
use blitz_traits::shell::{ColorScheme, Viewport};
use dioxus::prelude::*;
use dioxus_native_dom::{DioxusDocument, DocumentConfig};
use keyboard_types::{Code, Key, Modifiers};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

/// The default window: a laptop screen, in CSS pixels.
pub const WIDTH: u32 = 1024;
/// The default window height, in CSS pixels.
pub const HEIGHT: u32 = 768;

/// blitz-paint keeps its layer counters in process-wide statics, so two tests painting at
/// once can trip each other up. Paint one frame at a time.
static PAINT: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// One element on the screen, as a query found it. Pass it back to any [`Screen`] method.
/// It stays valid while the element is in the document.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct El(pub(crate) usize);

impl El {
    /// The renderer's node id (for reaching into [`Screen::document`]).
    pub fn id(self) -> usize {
        self.0
    }
}

impl From<El> for Query {
    fn from(el: El) -> Self {
        Query::node(el.0)
    }
}

impl From<&El> for Query {
    fn from(el: &El) -> Self {
        Query::node(el.0)
    }
}

/// Where an element is, in window coordinates (CSS pixels).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    /// The middle point, where [`Screen::click`] presses.
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
}

#[derive(Clone)]
struct RootProps(Rc<dyn Fn() -> Element>);

impl PartialEq for RootProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[allow(non_snake_case)]
fn Root(props: RootProps) -> Element {
    (props.0)()
}

/// Settings for a [`Screen`]: window size, colour scheme and extra stylesheets.
/// Start with [`Screen::builder`] or [`Screen::builder_with`].
#[must_use = "call .render() to draw the screen"]
pub struct ScreenBuilder {
    app: Rc<dyn Fn() -> Element>,
    width: u32,
    height: u32,
    scheme: ColorScheme,
    reset: bool,
    kit: bool,
    app_components: bool,
    css: Vec<String>,
}

impl ScreenBuilder {
    fn new(app: Rc<dyn Fn() -> Element>) -> Self {
        Self {
            app,
            width: WIDTH,
            height: HEIGHT,
            scheme: ColorScheme::Light,
            reset: true,
            kit: true,
            app_components: true,
            css: Vec::new(),
        }
    }

    /// Window size in CSS pixels (`390, 844` for a phone).
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Render as if the system were in dark mode (`prefers-color-scheme: dark`).
    pub fn dark(mut self) -> Self {
        self.scheme = ColorScheme::Dark;
        self
    }

    /// Leave out the kit's `reset.css` (on by default, as `ThemeStyles { reset: true }` in
    /// the `atoll create` template loads it).
    pub fn without_reset(mut self) -> Self {
        self.reset = false;
        self
    }

    /// Leave out the kit's CSS entirely (for screens that don't use the kit).
    pub fn without_kit_css(mut self) -> Self {
        self.kit = false;
        self
    }

    /// Don't load `src/components/*/style.css` of the package under test. By default they are
    /// loaded, hashed like `#[css_module]`, so components copied in with `atoll add` look
    /// the way they do in the app.
    pub fn without_app_components(mut self) -> Self {
        self.app_components = false;
        self
    }

    /// Add a stylesheet, as written.
    pub fn css(mut self, css: impl Into<String>) -> Self {
        self.css.push(css.into());
        self
    }

    /// Add a plain stylesheet file (what `asset!("/assets/main.css")` points at). A relative
    /// path is relative to the package under test.
    #[track_caller]
    pub fn stylesheet(mut self, path: impl AsRef<Path>) -> Self {
        let path = css::app_path(path);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("atoll-test: cannot read {}: {e}", path.display()));
        self.css.push(text);
        self
    }

    /// Add a CSS module file, hashed like `#[css_module("/path/style.css")]`. A relative path
    /// is relative to the package under test.
    pub fn css_module(mut self, path: impl AsRef<Path>) -> Self {
        self.css.push(css::css_module(&css::app_path(path)));
        self
    }

    /// Draw the app and run its first effects.
    pub fn render(self) -> Screen {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .start_paused(true)
            .build()
            .expect("atoll-test: tokio runtime");
        let handle = rt.handle().clone();
        let _enter = handle.enter();

        let vdom = VirtualDom::new_with_props(Root, RootProps(self.app));
        let mut doc = DioxusDocument::new(vdom, DocumentConfig::default());
        doc.set_viewport(Viewport::new(self.width, self.height, 1.0, self.scheme));
        let mut sheet = String::new();
        if self.kit {
            sheet.push_str(&css::kit_css(self.reset));
        }
        if self.app_components {
            sheet.push_str(&css::css_modules_in(&css::app_path("src/components")));
        }
        for extra in &self.css {
            sheet.push('\n');
            sheet.push_str(extra);
        }
        if !sheet.is_empty() {
            doc.create_head_element("style", &[], &Some(sheet));
        }
        doc.initial_build();
        let mut screen = Screen {
            doc,
            width: self.width,
            height: self.height,
            time: 0.0,
            rt,
        };
        screen.settle();
        screen
    }
}

/// An app rendered headless on Blitz (the engine behind `dioxus-native`): real layout and
/// CSS, real event dispatch to your handlers, no window, GPU or browser.
///
/// ```no_run
/// use atoll_test::prelude::*;
/// use dioxus::prelude::*;
///
/// fn app() -> Element {
///     let mut n = use_signal(|| 0);
///     rsx! { button { onclick: move |_| n += 1, "Clicked {n}" } }
/// }
///
/// let mut screen = Screen::render(app);
/// screen.click(role("button"));
/// screen.assert_text(role("button"), "Clicked 1");
/// ```
pub struct Screen {
    doc: DioxusDocument,
    width: u32,
    height: u32,
    /// CSS animation clock in seconds; every `settle` moves it a second on, so enter
    /// animations have finished by the time a test looks.
    time: f64,
    /// A paused Tokio clock for the app's timers; [`Screen::advance`] moves it.
    rt: tokio::runtime::Runtime,
}

impl Screen {
    /// Render `app` in a 1024 x 768 light window with the kit's CSS.
    pub fn render(app: fn() -> Element) -> Self {
        Self::builder(app).render()
    }

    /// Render a closure, so the test can pass data in:
    /// `Screen::render_with(move || rsx! { Greeting { name: name.clone() } })`.
    pub fn render_with(app: impl Fn() -> Element + 'static) -> Self {
        Self::builder_with(app).render()
    }

    /// Settings first (size, dark mode, stylesheets), then [`ScreenBuilder::render`].
    pub fn builder(app: fn() -> Element) -> ScreenBuilder {
        ScreenBuilder::new(Rc::new(app))
    }

    /// [`Screen::builder`] for a closure.
    pub fn builder_with(app: impl Fn() -> Element + 'static) -> ScreenBuilder {
        ScreenBuilder::new(Rc::new(app))
    }

    /// The Blitz document, for anything this API doesn't cover.
    pub fn document(&self) -> &DioxusDocument {
        &self.doc
    }

    /// The Blitz document, mutably.
    pub fn document_mut(&mut self) -> &mut DioxusDocument {
        &mut self.doc
    }

    /// Run pending work (effects, spawned tasks, re-renders) and lay the page out again.
    /// Every action below already does this.
    pub fn settle(&mut self) {
        let handle = self.rt.handle().clone();
        let _enter = handle.enter();
        for _ in 0..50 {
            if !self.doc.poll(None) {
                break;
            }
        }
        self.doc.resolve(self.time);
        self.time += 1.0;
        self.doc.resolve(self.time);
    }

    /// Let `by` pass on the app's clock: timers (`tokio::time::sleep`,
    /// `dioxus_sdk_time`, toast timeouts) fire, in order, without the test waiting.
    pub fn advance(&mut self, by: Duration) {
        let step = Duration::from_millis(50);
        let mut left = by;
        while !left.is_zero() {
            let d = left.min(step);
            self.rt.block_on(async move { tokio::time::sleep(d).await });
            self.settle();
            left -= d;
        }
    }

    /// Advance the clock in small steps until `query` finds a visible element, for up to
    /// `timeout`. Panics if it never shows.
    #[track_caller]
    pub fn wait_for(&mut self, query: impl Into<Query>, timeout: Duration) -> El {
        let q = query.into();
        let step = Duration::from_millis(50);
        let mut waited = Duration::ZERO;
        loop {
            if let Ok(id) = q.one(&self.doc) {
                if self.visible_id(id) {
                    return El(id);
                }
            }
            if waited >= timeout {
                let why = q
                    .one(&self.doc)
                    .err()
                    .unwrap_or_else(|| "it is hidden".into());
                panic!("atoll-test: waited {timeout:?} for {q}: {why}");
            }
            self.advance(step);
            waited += step;
        }
    }

    // ---- Finding ----

    /// The one element `query` finds. Panics, listing what is on the screen, when none or
    /// several match.
    #[track_caller]
    pub fn get(&self, query: impl Into<Query>) -> El {
        let q = query.into();
        match q.one(&self.doc) {
            Ok(id) => El(id),
            Err(e) => panic!("atoll-test: {e}"),
        }
    }

    /// The one element `query` finds, or `None` when nothing matches. Panics when several do.
    #[track_caller]
    pub fn query(&self, query: impl Into<Query>) -> Option<El> {
        let q = query.into();
        let all = self.all_of(&q);
        match all.as_slice() {
            [] => None,
            [one] => Some(*one),
            _ => panic!("atoll-test: {}", q.one(&self.doc).unwrap_err()),
        }
    }

    /// Every element `query` finds, in document order.
    #[track_caller]
    pub fn all(&self, query: impl Into<Query>) -> Vec<El> {
        self.all_of(&query.into())
    }

    #[track_caller]
    fn all_of(&self, q: &Query) -> Vec<El> {
        match q.all(&self.doc) {
            Ok(ids) => ids.into_iter().map(El).collect(),
            Err(e) => panic!("atoll-test: {e}"),
        }
    }

    /// Whether `query` finds at least one element (hidden ones count for CSS queries).
    #[track_caller]
    pub fn exists(&self, query: impl Into<Query>) -> bool {
        !self.all(query).is_empty()
    }

    // ---- Reading ----

    /// The element's text, whitespace collapsed.
    #[track_caller]
    pub fn text(&self, query: impl Into<Query>) -> String {
        let id = self.get(query).0;
        query::normalize(&self.node(id).text_content())
    }

    /// The current value of a text field (what the user typed), else its `value` attribute.
    #[track_caller]
    pub fn value(&self, query: impl Into<Query>) -> String {
        let n = self.node(self.get(query).0);
        if let Some(input) = n.element_data().and_then(|e| e.text_input_data()) {
            return input.editor.raw_text().to_string();
        }
        n.attr(LocalName::from("value")).unwrap_or("").to_string()
    }

    /// An attribute's value.
    #[track_caller]
    pub fn attr(&self, query: impl Into<Query>, name: &str) -> Option<String> {
        self.node(self.get(query).0)
            .attr(LocalName::from(name))
            .map(str::to_string)
    }

    /// Checked or pressed: `aria-checked`, `aria-pressed`, `aria-selected` or `data-state`
    /// (`checked`, `on`), else a native checkbox's `checked`.
    #[track_caller]
    pub fn is_checked(&self, query: impl Into<Query>) -> bool {
        let n = self.node(self.get(query).0);
        for a in ["aria-checked", "aria-pressed", "aria-selected"] {
            if let Some(v) = n.attr(LocalName::from(a)) {
                return v == "true" || v == "mixed";
            }
        }
        if let Some(v) = n.attr(LocalName::from("data-state")) {
            return v == "checked" || v == "on";
        }
        n.element_data()
            .and_then(|e| e.checkbox_input_checked())
            .unwrap_or(false)
    }

    /// The element's accessible name (what a screen reader announces it as).
    #[track_caller]
    pub fn name(&self, query: impl Into<Query>) -> String {
        let id = self.get(query).0;
        query::accessible_name(&self.doc, self.node(id))
    }

    /// The element's role, explicit or implied by its tag.
    #[track_caller]
    pub fn role(&self, query: impl Into<Query>) -> Option<String> {
        let id = self.get(query).0;
        query::role_of(&self.doc, self.node(id))
    }

    /// Where the element is, in window coordinates.
    #[track_caller]
    pub fn rect(&self, query: impl Into<Query>) -> Rect {
        self.rect_of(self.get(query).0)
    }

    fn rect_of(&self, id: usize) -> Rect {
        let node = self.node(id);
        let pos = node.absolute_position(0.0, 0.0);
        let size = node.final_layout.size;
        let scroll = self.doc.viewport_scroll();
        Rect {
            x: pos.x - scroll.x as f32,
            y: pos.y - scroll.y as f32,
            w: size.width,
            h: size.height,
        }
    }

    /// Present, not hidden (CSS, `hidden`, `aria-hidden`), with a size, and at least partly
    /// inside the window. Missing elements are not visible.
    #[track_caller]
    pub fn is_visible(&self, query: impl Into<Query>) -> bool {
        let q = query.into().hidden();
        match self.all_of(&q).as_slice() {
            [] => false,
            [one] => self.visible_id(one.0),
            _ => panic!("atoll-test: {}", q.one(&self.doc).unwrap_err()),
        }
    }

    fn visible_id(&self, id: usize) -> bool {
        if query::is_hidden(&self.doc, self.node(id)) {
            return false;
        }
        let r = self.rect_of(id);
        r.w > 0.0
            && r.h > 0.0
            && r.x < self.width as f32
            && r.y < self.height as f32
            && r.x + r.w > 0.0
            && r.y + r.h > 0.0
    }

    /// The element with keyboard focus.
    pub fn focused(&self) -> Option<El> {
        self.doc.focus_node_id().map(El)
    }

    /// Whether the element `query` finds has keyboard focus.
    #[track_caller]
    pub fn is_focused(&self, query: impl Into<Query>) -> bool {
        Some(self.get(query)) == self.focused()
    }

    /// The element's HTML (for debugging or a snapshot).
    #[track_caller]
    pub fn html(&self, query: impl Into<Query>) -> String {
        self.node(self.get(query).0).outer_html()
    }

    /// One line describing the element: tag, id, role and name.
    pub fn describe(&self, el: El) -> String {
        query::describe(&self.doc, el.0)
    }

    /// Print the document tree (for debugging a test).
    pub fn debug(&self) {
        self.doc.print_tree();
    }

    // ---- Acting ----

    /// Mouse down and up in the middle of the element. Panics if it isn't on screen (scroll
    /// with [`Screen::scroll_into_view`] first).
    #[track_caller]
    pub fn click(&mut self, query: impl Into<Query>) {
        let q = query.into();
        let id = self.get(&q).0;
        let r = self.rect_of(id);
        assert!(
            self.visible_id(id),
            "atoll-test: can't click {q}: it is not visible ({r:?}; window {}x{})",
            self.width,
            self.height
        );
        let (x, y) = r.center();
        self.click_at(x, y);
    }

    /// Press and release the main mouse button at a window point.
    pub fn click_at(&mut self, x: f32, y: f32) {
        self.mouse(x, y, MouseEventButton::Main, true);
    }

    /// Press and release the right mouse button in the middle of the element.
    #[track_caller]
    pub fn right_click(&mut self, query: impl Into<Query>) {
        let (x, y) = self.rect(query).center();
        self.mouse(x, y, MouseEventButton::Secondary, true);
    }

    /// Move the mouse to the middle of the element (hover).
    #[track_caller]
    pub fn hover(&mut self, query: impl Into<Query>) {
        let (x, y) = self.rect(query).center();
        self.mouse(x, y, MouseEventButton::Main, false);
    }

    fn mouse(&mut self, x: f32, y: f32, button: MouseEventButton, press: bool) {
        let held = match button {
            MouseEventButton::Secondary => MouseEventButtons::Secondary,
            _ => MouseEventButtons::Primary,
        };
        let ev = |buttons| BlitzMouseButtonEvent {
            x,
            y,
            button,
            buttons,
            mods: Default::default(),
        };
        let handle = self.rt.handle().clone();
        let _enter = handle.enter();
        self.doc
            .handle_ui_event(UiEvent::MouseMove(ev(MouseEventButtons::None)));
        if press {
            self.doc.handle_ui_event(UiEvent::MouseDown(ev(held)));
            self.doc
                .handle_ui_event(UiEvent::MouseUp(ev(MouseEventButtons::None)));
        }
        self.settle();
    }

    /// Press the main button at `from`, move through `to` with it held, release at the last
    /// point (sliders, drag-and-drop, swipe to dismiss).
    pub fn drag(&mut self, from: (f32, f32), to: &[(f32, f32)]) {
        let ev = |(x, y): (f32, f32), buttons| BlitzMouseButtonEvent {
            x,
            y,
            button: MouseEventButton::Main,
            buttons,
            mods: Default::default(),
        };
        let handle = self.rt.handle().clone();
        let _enter = handle.enter();
        self.doc
            .handle_ui_event(UiEvent::MouseMove(ev(from, MouseEventButtons::None)));
        self.doc
            .handle_ui_event(UiEvent::MouseDown(ev(from, MouseEventButtons::Primary)));
        self.settle();
        for &p in to {
            self.doc
                .handle_ui_event(UiEvent::MouseMove(ev(p, MouseEventButtons::Primary)));
            self.settle();
        }
        let last = to.last().copied().unwrap_or(from);
        self.doc
            .handle_ui_event(UiEvent::MouseUp(ev(last, MouseEventButtons::None)));
        self.settle();
    }

    /// Move keyboard focus to the element, as a script's `.focus()` would (focus and focusin
    /// handlers run).
    #[track_caller]
    pub fn focus(&mut self, query: impl Into<Query>) {
        let id = self.get(query).0;
        self.doc.set_focus_to(id);
        // Blitz moved focus without an event; a no-op key release makes the renderer report
        // it, as a real focus change would.
        self.key_event(
            Key::Unidentified,
            Code::Unidentified,
            Modifiers::empty(),
            None,
        );
    }

    /// Press and release a key on the focused element: `"Enter"`, `"Escape"`, `"Tab"`,
    /// `"Shift+Tab"`, `"ArrowDown"`, `"Space"`, `"Control+a"`, `"x"`.
    #[track_caller]
    pub fn press(&mut self, key: &str) {
        let (k, c, m) = keys::parse(key);
        let text = match &k {
            Key::Character(s) if m.is_empty() || m == Modifiers::SHIFT => Some(s.clone()),
            _ => None,
        };
        self.key_event(k, c, m, text);
    }

    /// Press a key `times` times.
    #[track_caller]
    pub fn press_times(&mut self, key: &str, times: usize) {
        for _ in 0..times {
            self.press(key);
        }
    }

    /// Tab to the next focusable element.
    pub fn tab(&mut self) {
        self.press("Tab");
    }

    /// Shift+Tab to the previous focusable element.
    pub fn shift_tab(&mut self) {
        self.press("Shift+Tab");
    }

    /// Type `text` into the focused element, one key per character.
    pub fn type_text(&mut self, text: &str) {
        for c in text.chars() {
            let s = c.to_string();
            let code = if c == ' ' {
                Code::Space
            } else {
                Code::Unidentified
            };
            self.key_event(Key::Character(s.clone()), code, Modifiers::empty(), Some(s));
        }
    }

    /// Focus the field `query` finds, then type `text` at the end of what it holds.
    #[track_caller]
    pub fn type_into(&mut self, query: impl Into<Query>, text: &str) {
        let q = query.into();
        self.focus(&q);
        self.press("End");
        self.type_text(text);
    }

    /// Focus the field, select everything in it, and type `text` over it (empty clears it).
    #[track_caller]
    pub fn fill(&mut self, query: impl Into<Query>, text: &str) {
        let q = query.into();
        self.focus(&q);
        let select_all = if cfg!(target_os = "macos") {
            "Meta+a"
        } else {
            "Control+a"
        };
        if !self.value(&q).is_empty() {
            self.press(select_all);
            self.press("Backspace");
        }
        self.type_text(text);
    }

    fn key_event(&mut self, key: Key, code: Code, modifiers: Modifiers, text: Option<String>) {
        let handle = self.rt.handle().clone();
        let _enter = handle.enter();
        for state in [KeyState::Pressed, KeyState::Released] {
            let ev = BlitzKeyEvent {
                key: key.clone(),
                code,
                modifiers,
                location: keyboard_types::Location::Standard,
                is_auto_repeating: false,
                is_composing: false,
                state,
                text: if state == KeyState::Pressed {
                    text.clone().map(Into::into)
                } else {
                    None
                },
            };
            self.doc.handle_ui_event(match state {
                KeyState::Pressed => UiEvent::KeyDown(ev),
                KeyState::Released => UiEvent::KeyUp(ev),
            });
        }
        self.settle();
    }

    /// Scroll the page (as the mouse wheel does) until the element is inside the window.
    #[track_caller]
    pub fn scroll_into_view(&mut self, query: impl Into<Query>) {
        let r = self.rect(query);
        let margin = 16.0;
        let dy = if r.y < 0.0 {
            r.y - margin
        } else if r.y + r.h > self.height as f32 {
            r.y + r.h - self.height as f32 + margin
        } else {
            0.0
        };
        if dy != 0.0 {
            self.doc.scroll_viewport_by(0.0, -dy as f64);
            self.settle();
        }
    }

    // ---- Asserting (each panics with what it found, pointing at the test line) ----

    /// The element exists and is visible.
    #[track_caller]
    pub fn assert_visible(&self, query: impl Into<Query>) {
        let q = query.into();
        if !self.is_visible(&q) {
            let why = match q.clone().hidden().one(&self.doc) {
                Ok(id) => format!(
                    "{} is there but hidden or off screen",
                    self.describe(El(id))
                ),
                Err(e) => e,
            };
            panic!("atoll-test: expected {q} to be visible: {why}");
        }
    }

    /// Nothing `query` finds is visible (missing counts).
    #[track_caller]
    pub fn assert_not_visible(&self, query: impl Into<Query>) {
        let q = query.into().hidden();
        let shown: Vec<El> = self
            .all_of(&q)
            .into_iter()
            .filter(|e| self.visible_id(e.0))
            .collect();
        if let Some(e) = shown.first() {
            panic!(
                "atoll-test: expected {q} not to be visible, but {} is",
                self.describe(*e)
            );
        }
    }

    /// The element's text is `expected` (whitespace collapsed on both sides).
    #[track_caller]
    pub fn assert_text(&self, query: impl Into<Query>, expected: &str) {
        let q = query.into();
        let got = self.text(&q);
        assert_eq!(
            got,
            query::normalize(expected),
            "atoll-test: text of {q} differs"
        );
    }

    /// The field holds `expected`.
    #[track_caller]
    pub fn assert_value(&self, query: impl Into<Query>, expected: &str) {
        let q = query.into();
        assert_eq!(self.value(&q), expected, "atoll-test: value of {q} differs");
    }

    /// The element has keyboard focus.
    #[track_caller]
    pub fn assert_focused(&self, query: impl Into<Query>) {
        let q = query.into();
        let want = self.get(&q);
        let got = self.focused();
        if got != Some(want) {
            let where_ = got.map_or("nothing".into(), |e| self.describe(e));
            panic!("atoll-test: expected {q} to have focus, but focus is on {where_}");
        }
    }

    /// The element is checked, pressed or selected ([`Screen::is_checked`]).
    #[track_caller]
    pub fn assert_checked(&self, query: impl Into<Query>, checked: bool) {
        let q = query.into();
        assert_eq!(
            self.is_checked(&q),
            checked,
            "atoll-test: checked state of {q} differs"
        );
    }

    // ---- Pictures ----

    /// Paint the current frame on the CPU and save it as a PNG (for a human to look at).
    pub fn screenshot(&self, path: impl AsRef<Path>) -> PathBuf {
        let path = css::app_path(path);
        let buf = self.paint();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        image::save_buffer(
            &path,
            &buf,
            self.width,
            self.height,
            image::ColorType::Rgba8,
        )
        .expect("atoll-test: write png");
        path
    }

    /// RGBA of the pixel at (x, y) in a CPU-painted frame.
    pub fn pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let buf = self.paint();
        let i = ((y * self.width + x) * 4) as usize;
        [buf[i], buf[i + 1], buf[i + 2], buf[i + 3]]
    }

    fn paint(&self) -> Vec<u8> {
        use anyrender::ImageRenderer as _;
        let (w, h) = (self.width, self.height);
        let mut renderer = anyrender_vello_cpu::VelloCpuImageRenderer::new(w, h);
        let mut buf = Vec::new();
        let _paint = PAINT.lock().unwrap_or_else(|e| e.into_inner());
        renderer.render_to_vec(
            |scene| blitz_paint::paint_scene(scene, &self.doc, 1.0, w, h),
            &mut buf,
        );
        buf
    }

    fn node(&self, id: usize) -> &blitz_dom::Node {
        self.doc
            .get_node(id)
            .unwrap_or_else(|| panic!("atoll-test: element #{id} is no longer in the document"))
    }
}
