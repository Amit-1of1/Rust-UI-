//! Headless Blitz harness: renders a component with `dioxus-native-dom` (the renderer
//! `dioxus-native` uses, without a window or GPU), injects the kit's CSS the way `dx` would
//! bundle it, and lets a test click things and read back the computed layout.

#![allow(dead_code)]

use blitz_dom::Document as _;
use blitz_traits::events::{
    BlitzKeyEvent, BlitzMouseButtonEvent, KeyState, MouseEventButton, MouseEventButtons, UiEvent,
};
use blitz_traits::shell::{ColorScheme, Viewport};
use dioxus::prelude::*;
use dioxus_native_dom::{DioxusDocument, DocumentConfig};
pub use keyboard_types::{Code, Key, Modifiers};
use std::path::{Path, PathBuf};

pub const WIDTH: u32 = 1024;
pub const HEIGHT: u32 = 768;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
    /// Has a size and lies (at least partly) inside the window.
    pub fn on_screen(&self) -> bool {
        self.w > 0.0
            && self.h > 0.0
            && self.x < WIDTH as f32
            && self.y < HEIGHT as f32
            && self.x + self.w > 0.0
            && self.y + self.h > 0.0
    }
    /// Lies entirely inside the window.
    pub fn inside_window(&self) -> bool {
        self.w > 0.0
            && self.h > 0.0
            && self.x >= -0.5
            && self.y >= -0.5
            && self.x + self.w <= WIDTH as f32 + 0.5
            && self.y + self.h <= HEIGHT as f32 + 0.5
    }
}

/// blitz-paint keeps its layer counters in process-wide statics, so two tests painting at
/// once can trip each other up. Paint one frame at a time.
static PAINT: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub struct Harness {
    pub doc: DioxusDocument,
    /// Animation clock in seconds. Every `settle` moves it a second on, so CSS enter
    /// animations have finished by the time a test looks (the real shell ticks the same clock).
    pub time: f64,
}

fn ui_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/ui")
        .canonicalize()
        .expect("crates/ui")
}

/// The kit's CSS as `dx` serves it: `theme.css` as is, every component's `style.css`
/// with its class names hashed exactly like `#[css_module]` does.
pub fn kit_css() -> String {
    let root = ui_src();
    let mut css = std::fs::read_to_string(root.join("assets/theme.css")).expect("theme.css");
    let mut dirs: Vec<_> = std::fs::read_dir(root.join("src/components"))
        .expect("components")
        .flatten()
        .map(|e| e.path())
        .collect();
    dirs.sort();
    for dir in dirs {
        let file = dir.join("style.css");
        let Ok(src) = std::fs::read_to_string(&file) else {
            continue;
        };
        let hash = manganis_core::create_module_hash(&file);
        // Files the macro never sees (no `#[css_module]`, comment only) may not parse.
        let Ok(out) = manganis_core::transform_css(&src, &hash) else {
            continue;
        };
        css.push('\n');
        css.push_str(&out);
    }
    css
}

impl Harness {
    pub fn new(app: fn() -> Element) -> Self {
        Self::with_scheme(app, ColorScheme::Light)
    }

    pub fn with_scheme(app: fn() -> Element, scheme: ColorScheme) -> Self {
        let vdom = VirtualDom::new(app);
        let mut doc = DioxusDocument::new(vdom, DocumentConfig::default());
        doc.set_viewport(Viewport::new(WIDTH, HEIGHT, 1.0, scheme));
        doc.create_head_element("style", &[], &Some(kit_css()));
        doc.initial_build();
        let mut h = Self { doc, time: 0.0 };
        h.settle();
        h
    }

    /// Run pending work (effects, spawned tasks, re-renders) and re-layout.
    pub fn settle(&mut self) {
        for _ in 0..20 {
            if !self.doc.poll(None) {
                break;
            }
        }
        self.doc.resolve(self.time);
        self.time += 1.0;
        self.doc.resolve(self.time);
    }

    pub fn find(&self, selector: &str) -> Option<usize> {
        self.doc.query_selector(selector).expect("valid selector")
    }

    pub fn rect(&self, selector: &str) -> Option<Rect> {
        let id = self.find(selector)?;
        let node = self.doc.get_node(id)?;
        let pos = node.absolute_position(0.0, 0.0);
        let size = node.final_layout.size;
        Some(Rect {
            x: pos.x,
            y: pos.y,
            w: size.width,
            h: size.height,
        })
    }

    /// Mouse down and up in the middle of the first element matching `selector`.
    pub fn click(&mut self, selector: &str) {
        let r = self
            .rect(selector)
            .unwrap_or_else(|| panic!("no element for {selector}"));
        assert!(r.on_screen(), "{selector} is not on screen: {r:?}");
        let (x, y) = r.center();
        self.click_at(x, y);
    }

    /// Press and release a key (sent to the focused element, as the shell does).
    pub fn key(&mut self, key: Key, code: Code, modifiers: Modifiers) {
        for state in [KeyState::Pressed, KeyState::Released] {
            let ev = BlitzKeyEvent {
                key: key.clone(),
                code,
                modifiers,
                location: keyboard_types::Location::Standard,
                is_auto_repeating: false,
                is_composing: false,
                state,
                text: None,
            };
            self.doc.handle_ui_event(match state {
                KeyState::Pressed => UiEvent::KeyDown(ev),
                KeyState::Released => UiEvent::KeyUp(ev),
            });
        }
        self.settle();
    }

    /// Type `text` into the focused element, one key per character.
    pub fn type_text(&mut self, text: &str) {
        for c in text.chars() {
            for state in [KeyState::Pressed, KeyState::Released] {
                let ev = BlitzKeyEvent {
                    key: Key::Character(c.to_string()),
                    code: Code::Unidentified,
                    modifiers: Modifiers::empty(),
                    location: keyboard_types::Location::Standard,
                    is_auto_repeating: false,
                    is_composing: false,
                    state,
                    text: (state == KeyState::Pressed).then(|| c.to_string().into()),
                };
                self.doc.handle_ui_event(match state {
                    KeyState::Pressed => UiEvent::KeyDown(ev),
                    KeyState::Released => UiEvent::KeyUp(ev),
                });
            }
            self.settle();
        }
    }

    /// The element that has keyboard focus.
    pub fn focused(&self) -> Option<usize> {
        self.doc.focus_node_id()
    }

    /// Move the mouse to the middle of the first element matching `selector`.
    pub fn hover(&mut self, selector: &str) {
        let r = self
            .rect(selector)
            .unwrap_or_else(|| panic!("no element for {selector}"));
        let (x, y) = r.center();
        self.move_to(x, y);
    }

    /// Move the mouse to (x, y) without pressing anything.
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.doc
            .handle_ui_event(UiEvent::MouseMove(BlitzMouseButtonEvent {
                x,
                y,
                button: MouseEventButton::Main,
                buttons: MouseEventButtons::None,
                mods: Default::default(),
            }));
        self.settle();
    }

    /// Press the main button at (x, y), move to each point with it held, release at the last.
    pub fn drag(&mut self, from: (f32, f32), to: &[(f32, f32)]) {
        let ev = |(x, y): (f32, f32), buttons| BlitzMouseButtonEvent {
            x,
            y,
            button: MouseEventButton::Main,
            buttons,
            mods: Default::default(),
        };
        self.doc
            .handle_ui_event(UiEvent::MouseMove(ev(from, MouseEventButtons::None)));
        self.doc
            .handle_ui_event(UiEvent::MouseDown(ev(from, MouseEventButtons::Primary)));
        self.settle();
        for &point in to {
            self.doc
                .handle_ui_event(UiEvent::MouseMove(ev(point, MouseEventButtons::Primary)));
            self.settle();
        }
        let last = to.last().copied().unwrap_or(from);
        self.doc
            .handle_ui_event(UiEvent::MouseUp(ev(last, MouseEventButtons::None)));
        self.settle();
    }

    /// Press and release the secondary (right) button at (x, y).
    pub fn right_click_at(&mut self, x: f32, y: f32) {
        let ev = |buttons| BlitzMouseButtonEvent {
            x,
            y,
            button: MouseEventButton::Secondary,
            buttons,
            mods: Default::default(),
        };
        self.doc
            .handle_ui_event(UiEvent::MouseMove(ev(MouseEventButtons::None)));
        self.doc
            .handle_ui_event(UiEvent::MouseDown(ev(MouseEventButtons::Secondary)));
        self.doc
            .handle_ui_event(UiEvent::MouseUp(ev(MouseEventButtons::None)));
        self.settle();
    }

    pub fn click_at(&mut self, x: f32, y: f32) {
        let ev = |buttons| BlitzMouseButtonEvent {
            x,
            y,
            button: MouseEventButton::Main,
            buttons,
            mods: Default::default(),
        };
        self.doc
            .handle_ui_event(UiEvent::MouseMove(ev(MouseEventButtons::None)));
        self.doc
            .handle_ui_event(UiEvent::MouseDown(ev(MouseEventButtons::Primary)));
        self.doc
            .handle_ui_event(UiEvent::MouseUp(ev(MouseEventButtons::None)));
        self.settle();
    }

    /// Paint the current frame on the CPU and save it as a PNG (for a human to look at).
    pub fn screenshot(&self, path: impl AsRef<Path>) {
        use anyrender::ImageRenderer as _;
        let mut renderer = anyrender_vello_cpu::VelloCpuImageRenderer::new(WIDTH, HEIGHT);
        let mut buf = Vec::new();
        let _paint = PAINT.lock().unwrap_or_else(|e| e.into_inner());
        renderer.render_to_vec(
            |scene| blitz_paint::paint_scene(scene, &self.doc, 1.0, WIDTH, HEIGHT),
            &mut buf,
        );
        if let Some(dir) = path.as_ref().parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        image::save_buffer(path, &buf, WIDTH, HEIGHT, image::ColorType::Rgba8).expect("png");
    }

    /// RGBA of the pixel at (x, y) in a CPU-painted frame.
    pub fn pixel(&self, x: u32, y: u32) -> [u8; 4] {
        use anyrender::ImageRenderer as _;
        let mut renderer = anyrender_vello_cpu::VelloCpuImageRenderer::new(WIDTH, HEIGHT);
        let mut buf = Vec::new();
        let _paint = PAINT.lock().unwrap_or_else(|e| e.into_inner());
        renderer.render_to_vec(
            |scene| blitz_paint::paint_scene(scene, &self.doc, 1.0, WIDTH, HEIGHT),
            &mut buf,
        );
        let i = ((y * WIDTH + x) * 4) as usize;
        [buf[i], buf[i + 1], buf[i + 2], buf[i + 3]]
    }

    pub fn dump(&self) {
        self.doc.print_tree();
    }
}
