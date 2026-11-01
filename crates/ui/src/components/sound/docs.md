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
