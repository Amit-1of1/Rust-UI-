# Sound

- Copy into your app: `atoll add sound` or `dx components add sound --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `label`, `native_select`, `switch`).
- Import: `use crate::components::sound::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

Interface sounds for presses, switches, dialogs, notices, success and errors, in three sets:
**Ripple** (soft water drops), **Glass** (bright chimes) and **Driftwood** (warm marimba and
wood). **Off until the person using the app turns them on**, and each person can mix and
match: a different set, or silence, for each sound. Their choices are remembered on that
device.

Wrap the app (or the part that should make sounds) in a `SoundProvider` and put
`SoundSettings` (the switch plus a set per sound, each with a Play button) or just
`SoundToggle` where people change settings. Kit components then play sounds by themselves:

| Sound | Plays when | Needs `taps: true` |
|---|---|---|
| Success | A success toast appears | no |
| Error | An error toast appears, or a `Form` is sent with invalid fields | no |
| Notice | An info or warning toast appears | no |
| Tap | A `Button` or `IconButton` is pressed | yes |
| Toggle on / off | A `Switch` or `Checkbox` changes | yes |
| Open / close | A `Dialog`, `Sheet` or `AlertDialog` opens or closes | yes |

Play one yourself with `use_sound().play(SoundEvent::Success)`, for example after a save
finishes.

## Component Structure

```rust
SoundProvider {
    // The set every sound uses until the person picks another (default Ripple).
    style: SoundStyle::Glass,
    // Also play taps, toggles and dialog sounds (off by default).
    taps: true,
    // Overall loudness, 0 to 1.
    volume: 0.8,
    // The person's controls: the on/off switch and, while on, a set or "Off" per sound.
    SoundSettings {}
    // Or only the on/off switch.
    SoundToggle {}
    // ... the app ...
}
```

## Accessibility

- Role and name: `SoundToggle` is a `Switch` (`role="switch"`) named by its visible label
  ("Sounds", change it with `label`).
- Keyboard: Tab to the switch, Space or Enter to turn sounds on or off.
- Sound never carries meaning alone: every sound goes with something visible and announced
  (the toast, the field error). People who can't hear it lose nothing.
- Starts off, and only the person's own action turns it on: apps can't switch it on for them.
- `SoundSettings` rows are a labelled native `<select>` each, plus a "Play" button named
  "Play <sound>"; they appear only while sounds are on.
- Short and quiet: in every set minor sounds last at most 0.35 s and major ones at most 1 s
  (`tests/components/sound.rs` checks this), so they don't sit on top of a screen reader.
- Reduced motion: when the system asks for reduced motion, only success, error and notice
  sounds play, never taps, toggles or dialog sounds.
- The first sound starts the browser's audio after a click or key press, which browsers
  require.

## Platforms

Web only for now. The sounds are made by the browser's Web Audio API from Rust (web-sys);
there are no audio files, so they add almost nothing to the download. On desktop and native
the components stay silent.

## Stability

Experimental: may change in any release (docs/STABILITY.md).

## Source

Original to this repo. The three sets were chosen by the owner on 2026-11-01, mixed per sound by each person.

## API

```rust
/// A moment that can make a sound.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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

/// A sound set. Each person can pick one per [`SoundEvent`] in [`SoundSettings`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum SoundStyle {
    /// Soft water drops: rising for "yes" and "open", falling for "no" and "close".
    #[default]
    Ripple,
    /// Clear, bell-like chimes: more noticeable.
    Glass,
    /// Warm marimba and wood-block sounds.
    Driftwood,
}

/// An oscillator's wave shape.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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

/// The app's sounds: [`play`](Sound::play) them, and read or change the person's choices.
/// Get it with [`use_sound`]. Outside a [`SoundProvider`] it is silent and does nothing.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct Sound(Option<SoundState>);

/// The nearest [`SoundProvider`]'s [`Sound`], or a silent one when there is none.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_sound() -> Sound

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
) -> Element

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
) -> Element

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
) -> Element

/// Plays [`SoundEvent::Open`] when it mounts and [`SoundEvent::Close`] when it goes away.
/// Dialogs, sheets and alert dialogs render one inside their open panel.
///
/// Internal: not part of the public API (docs/STABILITY.md).
#[component]
pub fn OpenCloseSound() -> Element

/// Plays `event` once when it mounts. Toasts render one.
///
/// Internal: not part of the public API (docs/STABILITY.md).
#[component]
pub fn MountSound(event: SoundEvent) -> Element

/// Wraps a switch's change callback so it plays [`SoundEvent::ToggleOn`] or `ToggleOff` first.
///
/// Internal: not part of the public API (docs/STABILITY.md).
pub fn use_toggle_sound(inner: Callback<bool>) -> Callback<bool>

/// [`use_toggle_sound`] for any change value (a checkbox's state): `is_on` says which sound.
///
/// Internal: not part of the public API (docs/STABILITY.md).
pub fn use_toggle_sound_with<T: 'static>(inner: Callback<T>, is_on: fn(&T) -> bool) -> Callback<T>
```

## Example

```rust
use crate::components::sound::*;
use crate::components::switch::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut alerts = use_signal(|| false);
    rsx! {
        SoundProvider { taps: true,
            div { style: "display: flex; flex-direction: column; gap: 1rem; max-width: 28rem;",
                p { style: "margin: 0; font-size: 0.875rem;",
                    "Off until you turn it on. Then pick Ripple, Glass, Driftwood or Off for each sound, and try the switch below."
                }
                SoundSettings {}
                div { style: "display: flex; align-items: center; gap: 0.5rem;",
                    Switch {
                        checked: alerts(),
                        aria_label: "Email alerts",
                        on_checked_change: move |v| alerts.set(v),
                    }
                    span { style: "font-size: 0.875rem;", "Email alerts" }
                }
            }
        }
    }
}
```
