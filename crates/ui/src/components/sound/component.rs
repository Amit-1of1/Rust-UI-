//! Interface sounds: short sounds for presses, switches, dialogs, notices, success and
//! errors, in three sets (Ripple, Glass, Driftwood) that each person can mix per sound. Off
//! until the person using the app turns them on.

use std::collections::HashMap;

use dioxus::prelude::*;

use crate::components::hooks::use_unique_id;
use crate::components::label::Label;
use crate::components::native_select::{NativeSelect, NativeSelectOption};
use crate::components::switch::Switch;

#[css_module("/src/components/sound/style.css")]
struct Styles;

/// A moment that can make a sound.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SoundEvent {
    /// A button was pressed.
    Tap,
    /// A switch or checkbox was turned on.
    ToggleOn,
    /// A switch or checkbox was turned off.
    ToggleOff,
    /// A dialog, sheet or alert dialog opened.
    Open,
    /// A dialog, sheet or alert dialog closed.
    Close,
    /// Something finished well: a success toast, a saved form.
    Success,
    /// Something went wrong: an error toast, a form that can't be sent.
    Error,
    /// A notice appeared: an info or warning toast, a new message.
    Notice,
}

impl SoundEvent {
    /// Every event, in the order the docs and [`SoundSettings`] list them.
    pub const ALL: [SoundEvent; 8] = [
        SoundEvent::Success,
        SoundEvent::Error,
        SoundEvent::Notice,
        SoundEvent::Tap,
        SoundEvent::ToggleOn,
        SoundEvent::ToggleOff,
        SoundEvent::Open,
        SoundEvent::Close,
    ];

    /// Taps, toggles, opening and closing: feedback for the person's own action. These only
    /// play when the app passes `taps: true` to [`SoundProvider`], and never when the person
    /// asked their system for reduced motion.
    pub fn is_minor(self) -> bool {
        matches!(
            self,
            SoundEvent::Tap
                | SoundEvent::ToggleOn
                | SoundEvent::ToggleOff
                | SoundEvent::Open
                | SoundEvent::Close
        )
    }

    /// A short machine name (`"toggle-on"`), used where the choice is stored.
    pub fn name(self) -> &'static str {
        match self {
            SoundEvent::Tap => "tap",
            SoundEvent::ToggleOn => "toggle-on",
            SoundEvent::ToggleOff => "toggle-off",
            SoundEvent::Open => "open",
            SoundEvent::Close => "close",
            SoundEvent::Success => "success",
            SoundEvent::Error => "error",
            SoundEvent::Notice => "notice",
        }
    }

    /// The English name [`SoundSettings`] shows ("Switch on").
    pub fn label(self) -> &'static str {
        match self {
            SoundEvent::Tap => "Button press",
            SoundEvent::ToggleOn => "Switch on",
            SoundEvent::ToggleOff => "Switch off",
            SoundEvent::Open => "Dialog opens",
            SoundEvent::Close => "Dialog closes",
            SoundEvent::Success => "Success",
            SoundEvent::Error => "Error",
            SoundEvent::Notice => "Notice",
        }
    }

    fn from_name(name: &str) -> Option<SoundEvent> {
        SoundEvent::ALL.into_iter().find(|e| e.name() == name)
    }

    /// The notes that make this event's sound in `style`.
    pub fn notes(self, style: SoundStyle) -> Vec<Note> {
        match style {
            SoundStyle::Ripple => ripple(self),
            SoundStyle::Glass => glass(self),
            SoundStyle::Driftwood => driftwood(self),
        }
    }
}

/// A sound set. Each person can pick one per [`SoundEvent`] in [`SoundSettings`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SoundStyle {
    /// Soft water drops: rising for "yes" and "open", falling for "no" and "close".
    #[default]
    Ripple,
    /// Clear, bell-like chimes: more noticeable.
    Glass,
    /// Warm marimba and wood-block sounds.
    Driftwood,
}

impl SoundStyle {
    /// Every set.
    pub const ALL: [SoundStyle; 3] = [SoundStyle::Ripple, SoundStyle::Glass, SoundStyle::Driftwood];

    /// A short machine name (`"glass"`), used where the choice is stored.
    pub fn name(self) -> &'static str {
        match self {
            SoundStyle::Ripple => "ripple",
            SoundStyle::Glass => "glass",
            SoundStyle::Driftwood => "driftwood",
        }
    }

    /// The name people see ("Glass").
    pub fn label(self) -> &'static str {
        match self {
            SoundStyle::Ripple => "Ripple",
            SoundStyle::Glass => "Glass",
            SoundStyle::Driftwood => "Driftwood",
        }
    }

    /// The set named `name` (as [`name`](Self::name) gives it).
    pub fn from_name(name: &str) -> Option<SoundStyle> {
        SoundStyle::ALL.into_iter().find(|s| s.name() == name)
    }
}

/// An oscillator's wave shape.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Wave {
    /// Pure and soft.
    Sine,
    /// A little brighter, woody.
    Triangle,
}

/// One tone: it glides from `from_hz` to `to_hz`, then fades out.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Note {
    /// Start, in seconds after the event.
    pub at: f64,
    /// Starting pitch.
    pub from_hz: f32,
    /// Pitch at the end of the glide (equal to `from_hz` for a steady tone).
    pub to_hz: f32,
    /// How long the glide takes, in seconds.
    pub glide: f64,
    /// Length including the fade, in seconds.
    pub length: f64,
    /// Peak loudness, 0 to 1, before the provider's `volume`.
    pub gain: f32,
    /// Wave shape.
    pub wave: Wave,
}

fn glide_note(at: f64, from_hz: f32, to_hz: f32, glide: f64, length: f64, gain: f32) -> Note {
    Note {
        at,
        from_hz,
        to_hz,
        glide,
        length,
        gain,
        wave: Wave::Sine,
    }
}

fn steady(at: f64, hz: f32, length: f64, gain: f32, wave: Wave) -> Note {
    Note {
        at,
        from_hz: hz,
        to_hz: hz,
        glide: 0.0,
        length,
        gain,
        wave,
    }
}

/// Ripple: water drops.
fn ripple(event: SoundEvent) -> Vec<Note> {
    match event {
        SoundEvent::Tap => vec![glide_note(0.0, 520.0, 880.0, 0.03, 0.07, 0.18)],
        SoundEvent::ToggleOn => vec![glide_note(0.0, 600.0, 1100.0, 0.05, 0.09, 0.16)],
        SoundEvent::ToggleOff => vec![glide_note(0.0, 900.0, 500.0, 0.05, 0.09, 0.14)],
        SoundEvent::Open => vec![glide_note(0.0, 300.0, 700.0, 0.12, 0.16, 0.12)],
        SoundEvent::Close => vec![glide_note(0.0, 700.0, 300.0, 0.10, 0.14, 0.10)],
        SoundEvent::Success => vec![
            glide_note(0.0, 660.0, 990.0, 0.03, 0.12, 0.16),
            glide_note(0.09, 990.0, 1320.0, 0.03, 0.18, 0.15),
        ],
        SoundEvent::Error => vec![
            glide_note(0.0, 420.0, 300.0, 0.06, 0.14, 0.20),
            glide_note(0.12, 330.0, 220.0, 0.08, 0.20, 0.20),
        ],
        SoundEvent::Notice => vec![
            glide_note(0.0, 880.0, 1180.0, 0.04, 0.12, 0.13),
            glide_note(0.11, 1180.0, 1480.0, 0.04, 0.12, 0.11),
            glide_note(0.22, 990.0, 1320.0, 0.04, 0.20, 0.10),
        ],
    }
}

/// A glassy bell: three inharmonic partials, the higher ones dying faster.
fn bell(out: &mut Vec<Note>, hz: f32, at: f64, length: f64, gain: f32) {
    for (ratio, level) in [(1.0_f32, 1.0_f32), (2.76, 0.35), (5.4, 0.12)] {
        let decay = length / (f64::from(ratio) * 0.6 + 0.4);
        out.push(steady(at, hz * ratio, decay, gain * level, Wave::Sine));
    }
}

/// Glass: chimes.
fn glass(event: SoundEvent) -> Vec<Note> {
    let mut n = Vec::new();
    let mut b = |hz, at, length, gain| bell(&mut n, hz, at, length, gain);
    match event {
        SoundEvent::Tap => b(2400.0, 0.0, 0.06, 0.07),
        SoundEvent::ToggleOn => b(1760.0, 0.0, 0.18, 0.09),
        SoundEvent::ToggleOff => b(1318.0, 0.0, 0.16, 0.08),
        SoundEvent::Open => {
            b(1046.0, 0.0, 0.3, 0.07);
            b(1568.0, 0.05, 0.3, 0.05);
        }
        SoundEvent::Close => {
            b(1568.0, 0.0, 0.25, 0.05);
            b(1046.0, 0.05, 0.25, 0.06);
        }
        SoundEvent::Success => {
            b(1318.0, 0.0, 0.5, 0.12);
            b(1661.0, 0.09, 0.5, 0.11);
            b(1976.0, 0.18, 0.8, 0.12);
        }
        SoundEvent::Error => {
            b(440.0, 0.0, 0.5, 0.16);
            b(466.0, 0.0, 0.5, 0.12);
            b(349.0, 0.16, 0.6, 0.15);
        }
        SoundEvent::Notice => {
            b(1568.0, 0.0, 0.6, 0.12);
            b(2093.0, 0.14, 0.8, 0.11);
        }
    }
    n
}

/// A wooden bar: the tone, a quick fourfold overtone and a soft body an octave down.
fn mallet(out: &mut Vec<Note>, hz: f32, at: f64, length: f64, gain: f32) {
    out.push(steady(at, hz, length, gain, Wave::Sine));
    out.push(steady(at, hz * 4.0, length * 0.25, gain * 0.25, Wave::Sine));
    out.push(steady(at, hz * 0.5, length * 0.6, gain * 0.3, Wave::Triangle));
}

/// Driftwood: marimba and wood block.
fn driftwood(event: SoundEvent) -> Vec<Note> {
    let mut n = Vec::new();
    if event == SoundEvent::Tap {
        // A wood-block knock.
        n.push(steady(0.0, 1800.0, 0.03, 0.10, Wave::Triangle));
        n.push(steady(0.0, 880.0, 0.04, 0.08, Wave::Sine));
        return n;
    }
    let mut m = |hz, at, length, gain| mallet(&mut n, hz, at, length, gain);
    match event {
        SoundEvent::Tap => {}
        SoundEvent::ToggleOn => m(784.0, 0.0, 0.18, 0.18),
        SoundEvent::ToggleOff => m(587.0, 0.0, 0.16, 0.16),
        SoundEvent::Open => {
            m(523.0, 0.0, 0.18, 0.14);
            m(784.0, 0.06, 0.2, 0.12);
        }
        SoundEvent::Close => {
            m(784.0, 0.0, 0.16, 0.12);
            m(523.0, 0.06, 0.18, 0.13);
        }
        SoundEvent::Success => {
            m(523.0, 0.0, 0.25, 0.18);
            m(659.0, 0.09, 0.25, 0.17);
            m(784.0, 0.18, 0.4, 0.18);
        }
        SoundEvent::Error => {
            m(220.0, 0.0, 0.25, 0.24);
            m(207.0, 0.13, 0.35, 0.24);
        }
        SoundEvent::Notice => {
            m(1046.0, 0.0, 0.25, 0.15);
            m(784.0, 0.12, 0.35, 0.15);
        }
    }
    n
}

/// The person's choice per sound: a set, or `None` for silence.
type Choices = HashMap<SoundEvent, Option<SoundStyle>>;

fn parse_choices(text: &str) -> Choices {
    text.split(';')
        .filter_map(|pair| {
            let (event, style) = pair.split_once(':')?;
            let event = SoundEvent::from_name(event.trim())?;
            let style = match style.trim() {
                "off" => None,
                name => Some(SoundStyle::from_name(name)?),
            };
            Some((event, style))
        })
        .collect()
}

fn format_choices(choices: &Choices) -> String {
    SoundEvent::ALL
        .into_iter()
        .filter_map(|e| {
            let style = choices.get(&e)?;
            Some(format!("{}:{}", e.name(), style.map_or("off", |s| s.name())))
        })
        .collect::<Vec<_>>()
        .join(";")
}

#[derive(Copy, Clone, PartialEq)]
struct SoundState {
    /// The person's choice. Starts off; read from storage after the first render.
    on: Signal<bool>,
    /// The person's set (or silence) per sound, where they changed it.
    choices: Signal<Choices>,
    style: ReadSignal<SoundStyle>,
    taps: ReadSignal<bool>,
    volume: ReadSignal<f32>,
    storage_key: &'static str,
}

/// The app's sounds: [`play`](Sound::play) them, and read or change the person's choices.
/// Get it with [`use_sound`]. Outside a [`SoundProvider`] it is silent and does nothing.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq)]
pub struct Sound(Option<SoundState>);

impl Sound {
    /// Whether a [`SoundProvider`] is above this component.
    pub fn is_available(&self) -> bool {
        self.0.is_some()
    }

    /// Whether the person has turned sounds on.
    pub fn is_on(&self) -> bool {
        self.0.is_some_and(|s| (s.on)())
    }

    /// Whether the app allows the minor sounds (taps, toggles, open and close).
    pub fn allows_taps(&self) -> bool {
        self.0.is_some_and(|s| (s.taps)())
    }

    /// Turn sounds on or off and remember the choice (in `localStorage` on the web). Call it
    /// only from the person's own action, such as [`SoundToggle`]: the choice is theirs.
    pub fn set_on(&self, on: bool) {
        if let Some(mut s) = self.0 {
            s.on.set(on);
            backend::store(s.storage_key, if on { "on" } else { "off" });
        }
    }

    /// The set `event` plays in: the person's pick, else the app's `style`. `None` when the
    /// person silenced that sound.
    pub fn style_for(&self, event: SoundEvent) -> Option<SoundStyle> {
        let s = self.0?;
        let picked = s.choices.read().get(&event).copied();
        match picked {
            Some(choice) => choice,
            None => Some((s.style)()),
        }
    }

    /// Set the set for one sound, or `None` to silence it, and remember it. Call it only from
    /// the person's own action, such as [`SoundSettings`].
    pub fn set_style(&self, event: SoundEvent, style: Option<SoundStyle>) {
        if let Some(mut s) = self.0 {
            s.choices.write().insert(event, style);
            let key = format!("{}-styles", s.storage_key);
            backend::store(&key, &format_choices(&s.choices.peek()));
        }
    }

    /// Whether `event` would make a sound right now.
    pub fn would_play(&self, event: SoundEvent) -> bool {
        let Some(s) = self.0 else { return false };
        if !*s.on.peek() {
            return false;
        }
        if event.is_minor() && (!*s.taps.peek() || backend::reduced_motion()) {
            return false;
        }
        let silenced = s.choices.peek().get(&event) == Some(&None);
        if silenced {
            return false;
        }
        let volume = *s.volume.peek();
        volume > 0.0
    }

    /// Play `event`'s sound, if the person turned sounds on (and, for minor events, the app
    /// allows them). Never the only signal: show and announce the same thing too.
    pub fn play(&self, event: SoundEvent) {
        if !self.would_play(event) {
            return;
        }
        if let (Some(style), Some(s)) = (self.style_for(event), self.0) {
            backend::play(&event.notes(style), *s.volume.peek());
        }
    }

    /// Play `event` in `style` at once, for a "Play" button in settings. Plays whenever a
    /// provider is present: the person asked for it.
    pub fn preview(&self, event: SoundEvent, style: SoundStyle) {
        if let Some(s) = self.0 {
            backend::play(&event.notes(style), *s.volume.peek());
        }
    }
}

/// The nearest [`SoundProvider`]'s [`Sound`], or a silent one when there is none.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_sound() -> Sound {
    Sound(try_use_context::<SoundState>())
}

/// Turns on interface sounds for everything inside it, once the person using the app agrees.
///
/// Sounds start **off**. They play only after the person turns them on with a
/// [`SoundToggle`] or [`SoundSettings`] (put one in your settings); their choices are
/// remembered. In [`SoundSettings`] they can also pick a set (Ripple, Glass, Driftwood) or
/// silence for each sound. Kit components then play [`SoundEvent`]s: toasts, invalid form
/// submits and, with `taps: true`, buttons, switches, checkboxes and dialogs.
///
/// Web only: on desktop and native it is silent for now.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn SoundProvider(
    /// The set each sound uses until the person picks another.
    #[props(default = ReadSignal::new(Signal::new(SoundStyle::Ripple)))]
    style: ReadSignal<SoundStyle>,
    /// Also play the minor sounds: taps, toggles, opening and closing. Off by default, since
    /// many people find a click on every press tiring.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    taps: ReadSignal<bool>,
    /// Overall loudness, 0 to 1.
    #[props(default = ReadSignal::new(Signal::new(1.0)))]
    volume: ReadSignal<f32>,
    /// Where the person's choices are stored (`localStorage` keys `<key>` and `<key>-styles`).
    #[props(default = "atoll-sound")]
    storage_key: &'static str,
    children: Element,
) -> Element {
    let mut on = use_signal(|| false);
    let mut choices = use_signal(Choices::new);
    // Read the stored choices after the first render, so a server-rendered page and the
    // browser's first render agree (the settings' markup depends on them).
    use_effect(move || {
        if backend::load(storage_key).as_deref() == Some("on") {
            on.set(true);
        }
        if let Some(text) = backend::load(&format!("{storage_key}-styles")) {
            choices.set(parse_choices(&text));
        }
    });
    use_context_provider(|| SoundState {
        on,
        choices,
        style,
        taps,
        volume,
        storage_key,
    });
    rsx! { {children} }
}

/// The switch that lets the person using the app turn sounds on or off. Renders nothing
/// outside a [`SoundProvider`]. Turning it on plays the success sound as a preview.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn SoundToggle(
    /// The visible label.
    #[props(default = "Sounds".to_string(), into)]
    label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let sound = use_sound();
    let id = use_unique_id("dx-sound");
    if !sound.is_available() {
        return rsx! {};
    }
    let label_id = format!("{id}-label");
    rsx! {
        div { class: Styles::dx_sound_toggle, ..attributes,
            Switch {
                checked: sound.is_on(),
                aria_labelledby: label_id.clone(),
                on_checked_change: move |on: bool| {
                    sound.set_on(on);
                    sound.play(SoundEvent::Success);
                },
            }
            span { id: label_id, class: Styles::dx_sound_label, {label} }
        }
    }
}

/// Sound settings for the person using the app: the on/off switch and, while sounds are on,
/// a set (Ripple, Glass, Driftwood) or "Off" for each sound, with a button to hear it. Minor
/// sounds are listed only when the app allows them (`taps: true`). Renders nothing outside a
/// [`SoundProvider`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn SoundSettings(
    /// The switch's visible label.
    #[props(default = "Sounds".to_string(), into)]
    label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let sound = use_sound();
    let id = use_unique_id("dx-sound-settings");
    if !sound.is_available() {
        return rsx! {};
    }
    let events = SoundEvent::ALL
        .into_iter()
        .filter(|e| !e.is_minor() || sound.allows_taps());
    rsx! {
        div { class: Styles::dx_sound_settings, ..attributes,
            SoundToggle { label }
            if sound.is_on() {
                div { class: Styles::dx_sound_rows,
                    for event in events {
                        SoundRow { key: "{event.name()}", event, id: format!("{id}-{}", event.name()) }
                    }
                }
            }
        }
    }
}

#[component]
fn SoundRow(event: SoundEvent, id: String) -> Element {
    let sound = use_sound();
    let current = sound.style_for(event);
    let value = current.map_or("off", |s| s.name());
    let label = event.label();
    rsx! {
        div { class: Styles::dx_sound_row,
            Label { html_for: id.clone(), {label} }
            NativeSelect {
                id,
                value,
                onchange: move |e: FormEvent| {
                    let style = SoundStyle::from_name(&e.value());
                    sound.set_style(event, style);
                    if let Some(style) = style {
                        sound.preview(event, style);
                    }
                },
                for style in SoundStyle::ALL {
                    NativeSelectOption { value: style.name(), selected: current == Some(style), {style.label()} }
                }
                NativeSelectOption { value: "off", selected: current.is_none(), "Off" }
            }
            button {
                r#type: "button",
                class: Styles::dx_sound_play,
                aria_label: "Play {label}",
                disabled: current.is_none(),
                onclick: move |_| {
                    if let Some(style) = current {
                        sound.preview(event, style);
                    }
                },
                "Play"
            }
        }
    }
}

/// Plays [`SoundEvent::Open`] when it mounts and [`SoundEvent::Close`] when it goes away.
/// Dialogs, sheets and alert dialogs render one inside their open panel.
///
/// Internal: not part of the public API (docs/STABILITY.md).
#[component]
pub fn OpenCloseSound() -> Element {
    let sound = use_sound();
    use_hook(move || sound.play(SoundEvent::Open));
    use_drop(move || sound.play(SoundEvent::Close));
    rsx! {}
}

/// Plays `event` once when it mounts. Toasts render one.
///
/// Internal: not part of the public API (docs/STABILITY.md).
#[component]
pub fn MountSound(event: SoundEvent) -> Element {
    let sound = use_sound();
    use_hook(move || sound.play(event));
    rsx! {}
}

/// Wraps a switch's change callback so it plays [`SoundEvent::ToggleOn`] or `ToggleOff` first.
///
/// Internal: not part of the public API (docs/STABILITY.md).
pub fn use_toggle_sound(inner: Callback<bool>) -> Callback<bool> {
    use_toggle_sound_with(inner, |on| *on)
}

/// [`use_toggle_sound`] for any change value (a checkbox's state): `is_on` says which sound.
///
/// Internal: not part of the public API (docs/STABILITY.md).
pub fn use_toggle_sound_with<T: 'static>(inner: Callback<T>, is_on: fn(&T) -> bool) -> Callback<T> {
    let sound = use_sound();
    use_callback(move |value: T| {
        sound.play(if is_on(&value) {
            SoundEvent::ToggleOn
        } else {
            SoundEvent::ToggleOff
        });
        inner.call(value);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choices_round_trip_and_skip_junk() {
        let mut c = Choices::new();
        c.insert(SoundEvent::Tap, Some(SoundStyle::Glass));
        c.insert(SoundEvent::Error, None);
        let text = format_choices(&c);
        assert_eq!(text, "error:off;tap:glass");
        assert_eq!(parse_choices(&text), c);
        assert_eq!(parse_choices("tap:nope;bogus:glass;;x").len(), 0);
    }
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod backend {
    use super::{Note, Wave};
    use std::cell::RefCell;
    // Every type named, so `atoll registry` lists each web-sys feature a copied file needs.
    use web_sys::{
        AudioContext, AudioContextState, AudioDestinationNode, AudioNode, AudioParam,
        AudioScheduledSourceNode, BaseAudioContext, GainNode, OscillatorNode, OscillatorType,
    };

    thread_local! {
        // Made on the first sound, which follows a click or key press, so the browser's
        // autoplay rules let it start.
        static CONTEXT: RefCell<Option<AudioContext>> = const { RefCell::new(None) };
    }

    fn storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }

    pub fn load(key: &str) -> Option<String> {
        storage()?.get_item(key).ok().flatten()
    }

    pub fn store(key: &str, value: &str) {
        if let Some(s) = storage() {
            let _ = s.set_item(key, value);
        }
    }

    pub fn reduced_motion() -> bool {
        web_sys::window()
            .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
            .is_some_and(|m| m.matches())
    }

    pub fn play(notes: &[Note], volume: f32) {
        CONTEXT.with(|cell| {
            let mut cell = cell.borrow_mut();
            if cell.is_none() {
                *cell = AudioContext::new().ok();
            }
            let Some(ctx) = cell.as_ref() else { return };
            if ctx.state() == AudioContextState::Suspended {
                let _ = ctx.resume();
            }
            for n in notes {
                let _ = note(ctx, n, volume.clamp(0.0, 1.0));
            }
        });
    }

    fn note(ctx: &AudioContext, n: &Note, volume: f32) -> Result<(), wasm_bindgen::JsValue> {
        let base: &BaseAudioContext = ctx;
        let start = base.current_time() + n.at;
        let osc: OscillatorNode = base.create_oscillator()?;
        osc.set_type(match n.wave {
            Wave::Triangle => OscillatorType::Triangle,
            _ => OscillatorType::Sine,
        });
        let freq: AudioParam = osc.frequency();
        freq.set_value_at_time(n.from_hz, start)?;
        if n.to_hz != n.from_hz && n.glide > 0.0 {
            freq.exponential_ramp_to_value_at_time(n.to_hz, start + n.glide)?;
        }
        let gain: GainNode = base.create_gain()?;
        let level: AudioParam = gain.gain();
        level.set_value_at_time(0.0, start)?;
        level.linear_ramp_to_value_at_time((n.gain * volume).max(0.0001), start + 0.004)?;
        level.exponential_ramp_to_value_at_time(0.0001, start + n.length)?;
        let output: AudioDestinationNode = base.destination();
        let source: &AudioNode = &osc;
        source.connect_with_audio_node(&gain)?;
        let fader: &AudioNode = &gain;
        fader.connect_with_audio_node(&output)?;
        let timed: &AudioScheduledSourceNode = &osc;
        timed.start_with_when(start)?;
        timed.stop_with_when(start + n.length + 0.05)?;
        Ok(())
    }
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
mod backend {
    use super::Note;

    pub fn load(_key: &str) -> Option<String> {
        None
    }

    pub fn store(_key: &str, _value: &str) {}

    pub fn reduced_motion() -> bool {
        false
    }

    pub fn play(_notes: &[Note], _volume: f32) {}
}
