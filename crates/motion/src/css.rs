//! Springs as CSS easing curves.
//!
//! Overlays animate in CSS (docs/GUARDRAILS.md), so they can't use the spring engine.
//! CSS `linear()` easing can trace any curve, though, so we simulate each
//! [`SpringPreset`] here and write it out as `linear(...)` plus the time it takes to
//! settle. The result is `assets/motion.css` in the kit: real spring feel, no JavaScript,
//! no per-frame work, and the same presets the Rust side uses.

use crate::SpringPreset;

/// Simulation step in seconds (1 ms).
const DT: f64 = 0.001;
/// Settled once within this distance of the target and this slow.
const REST_DISTANCE: f64 = 0.001;
const REST_SPEED: f64 = 0.01;
/// Largest allowed gap between the spring and the straight lines we draw instead.
const TOLERANCE: f64 = 0.002;

/// A spring traced from 0 to 1, ready to use as a CSS easing.
#[derive(Clone, Debug, PartialEq)]
pub struct CssSpring {
    /// Time until the spring settles, in milliseconds. Use it as the animation duration.
    pub duration_ms: u32,
    /// `(progress through the duration, value)` pairs, both starting at 0 and ending at 1
    /// in progress. Values can pass 1 while the spring overshoots.
    pub points: Vec<(f64, f64)>,
}

impl CssSpring {
    /// Simulates `preset` from rest at 0 to rest at 1.
    pub fn from_preset(preset: SpringPreset) -> Self {
        let spring = preset.spring();
        let (k, c, m) = (
            f64::from(spring.stiffness),
            f64::from(spring.damping),
            f64::from(spring.mass),
        );
        let (mut x, mut v) = (0.0_f64, 0.0_f64);
        let mut samples = vec![(0.0, 0.0)];
        let mut t = 0.0;
        // Semi-implicit Euler; ten simulated seconds is far past any preset settling.
        while t < 10.0 {
            let a = (-k * (x - 1.0) - c * v) / m;
            v += a * DT;
            x += v * DT;
            t += DT;
            samples.push((t, x));
            if (x - 1.0).abs() < REST_DISTANCE && v.abs() < REST_SPEED {
                break;
            }
        }
        let total = t;
        let mut normalized: Vec<(f64, f64)> =
            samples.into_iter().map(|(t, x)| (t / total, x)).collect();
        // The last sample is within REST_DISTANCE of 1; land on 1 exactly.
        if let Some(last) = normalized.last_mut() {
            *last = (1.0, 1.0);
        }
        Self {
            duration_ms: (total * 1000.0).round() as u32,
            points: simplify(&normalized, TOLERANCE),
        }
    }

    /// The curve as a CSS `linear()` easing function.
    pub fn easing(&self) -> String {
        let last = self.points.len().saturating_sub(1);
        let stops: Vec<String> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, &(t, x))| {
                let x = trim(format!("{x:.3}"));
                if i == 0 || i == last {
                    x
                } else {
                    format!("{x} {}%", trim(format!("{:.1}", t * 100.0)))
                }
            })
            .collect();
        format!("linear({})", stops.join(", "))
    }

    /// Largest value reached, e.g. `1.12` for 12% overshoot.
    pub fn peak(&self) -> f64 {
        self.points.iter().map(|p| p.1).fold(f64::MIN, f64::max)
    }
}

/// First and last line of the generated block inside the kit's `theme.css`.
pub const BEGIN_MARKER: &str = "/* motion tokens: begin (generated, do not edit) */";
pub const END_MARKER: &str = "/* motion tokens: end */";

/// The kit's motion tokens as a CSS block: one easing and one duration per preset,
/// wrapped in [`BEGIN_MARKER`] and [`END_MARKER`].
pub fn stylesheet() -> String {
    let mut out = format!(
        "{BEGIN_MARKER}\n\
         /* Each spring preset traced as a CSS easing, so CSS animations feel like the Rust\n \
         * springs. Use: animation: name var(--spring-snappy-duration) var(--spring-snappy);\n \
         * Regenerate: UPDATE_MOTION_CSS=1 cargo test -p rust-ui-kit --test motion_css */\n\
         :root {{\n",
    );
    for (name, preset) in [
        ("gentle", SpringPreset::Gentle),
        ("snappy", SpringPreset::Snappy),
        ("bouncy", SpringPreset::Bouncy),
    ] {
        let spring = CssSpring::from_preset(preset);
        out.push_str(&format!(
            "  --spring-{name}: {};\n  --spring-{name}-duration: {}ms;\n",
            spring.easing(),
            spring.duration_ms
        ));
    }
    out.push_str("}\n");
    out.push_str(END_MARKER);
    out
}

fn trim(s: String) -> String {
    if s.contains('.') {
        let t = s.trim_end_matches('0').trim_end_matches('.');
        if t == "-0" {
            "0".into()
        } else {
            t.into()
        }
    } else {
        s
    }
}

/// Ramer–Douglas–Peucker: keep only the points needed to stay within `tol` of the curve.
fn simplify(points: &[(f64, f64)], tol: f64) -> Vec<(f64, f64)> {
    if points.len() < 3 {
        return points.to_vec();
    }
    let (first, last) = (points[0], points[points.len() - 1]);
    let (mut worst, mut worst_dist) = (0, 0.0);
    for (i, &p) in points.iter().enumerate().take(points.len() - 1).skip(1) {
        // Vertical distance: CSS interpolates value linearly in time between stops.
        let span = last.0 - first.0;
        let line = if span == 0.0 {
            first.1
        } else {
            first.1 + (last.1 - first.1) * (p.0 - first.0) / span
        };
        let d = (p.1 - line).abs();
        if d > worst_dist {
            (worst, worst_dist) = (i, d);
        }
    }
    if worst_dist <= tol {
        return vec![first, last];
    }
    let mut left = simplify(&points[..=worst], tol);
    let right = simplify(&points[worst..], tol);
    left.pop();
    left.extend(right);
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curves_start_at_zero_and_end_at_one() {
        for preset in [
            SpringPreset::Gentle,
            SpringPreset::Snappy,
            SpringPreset::Bouncy,
        ] {
            let s = CssSpring::from_preset(preset);
            assert_eq!(s.points.first(), Some(&(0.0, 0.0)));
            assert_eq!(s.points.last(), Some(&(1.0, 1.0)));
            assert!(s.points.windows(2).all(|w| w[0].0 < w[1].0), "{preset:?}");
            assert!(s.easing().starts_with("linear(0, "));
            assert!(s.easing().ends_with(", 1)"));
        }
    }

    #[test]
    fn overshoot_matches_the_feel() {
        let bouncy = CssSpring::from_preset(SpringPreset::Bouncy).peak();
        let snappy = CssSpring::from_preset(SpringPreset::Snappy).peak();
        let gentle = CssSpring::from_preset(SpringPreset::Gentle).peak();
        assert!(
            bouncy > 1.2,
            "bouncy should overshoot visibly, got {bouncy}"
        );
        assert!(
            snappy < 1.03,
            "snappy should barely overshoot, got {snappy}"
        );
        assert!(
            gentle < 1.01,
            "gentle should not visibly overshoot, got {gentle}"
        );
    }

    #[test]
    fn curves_stay_short_enough_to_read() {
        for preset in [
            SpringPreset::Gentle,
            SpringPreset::Snappy,
            SpringPreset::Bouncy,
        ] {
            let s = CssSpring::from_preset(preset);
            assert!(s.points.len() <= 64, "{preset:?}: {} stops", s.points.len());
            assert!((200..=2000).contains(&s.duration_ms), "{preset:?}");
        }
    }

    #[test]
    fn simplified_curve_stays_within_tolerance() {
        let s = CssSpring::from_preset(SpringPreset::Bouncy);
        // Re-simulate and compare every millisecond against the piecewise-linear easing.
        let spring = SpringPreset::Bouncy.spring();
        let (k, c, m) = (
            f64::from(spring.stiffness),
            f64::from(spring.damping),
            f64::from(spring.mass),
        );
        let total = f64::from(s.duration_ms) / 1000.0;
        let (mut x, mut v, mut t) = (0.0_f64, 0.0_f64, 0.0_f64);
        while t + DT < total {
            let a = (-k * (x - 1.0) - c * v) / m;
            v += a * DT;
            x += v * DT;
            t += DT;
            let p = t / total;
            let seg = s.points.windows(2).find(|w| w[0].0 <= p && p <= w[1].0);
            let Some(w) = seg else { continue };
            let y = w[0].1 + (w[1].1 - w[0].1) * (p - w[0].0) / (w[1].0 - w[0].0);
            // Tolerance plus the rounding to three decimals in the CSS output.
            assert!(
                (y - x).abs() < 0.01,
                "at {p:.3}: curve {y:.4} vs spring {x:.4}"
            );
        }
    }
}
