//! Spring, glide and gesture maths with no UI in it, so every curve can be unit tested.
//!
//! * [`SpringConfig`] and [`SpringState`]: a damped spring solved in closed form, so a value
//!   lands in the same place whatever the frame rate, and keeps its speed when retargeted.
//! * [`Glide`]: momentum after a throw. The value coasts to a stop, bounces off bounds with a
//!   spring, or springs to the nearest snap point from where it would have landed.
//! * [`Animation`]: either of the above behind one `advance(dt)`.
//! * [`VelocityTracker`], [`rubber_band`], [`zoom_about`]: the helpers drag and pinch need.
//!
//! Units are up to the caller (pixels, a scale factor, 0..1 progress); time is in seconds and
//! speeds are units per second. The formulas are the standard damped harmonic oscillator and
//! exponential decay; the rubber band is the curve iOS scroll views use.
//!
//! # Stability
//!
//! Experimental: may change in any release (docs/STABILITY.md).

use crate::SpringPreset;

/// Physical constants of a spring: how stiff it is, how much it is damped, and the mass on it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringConfig {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
    /// Close enough to the target to stop (in the value's units).
    pub rest_delta: f64,
    /// Slow enough to stop (units per second).
    pub rest_speed: f64,
}

impl SpringConfig {
    /// A spring with the default rest thresholds (0.01 units, 0.1 units/s), which suit pixels.
    /// For values that run 0..1 lower them with [`SpringConfig::with_rest`].
    pub fn new(stiffness: f64, damping: f64, mass: f64) -> Self {
        Self {
            stiffness: stiffness.max(f64::EPSILON),
            damping: damping.max(0.0),
            mass: mass.max(f64::EPSILON),
            rest_delta: 0.01,
            rest_speed: 0.1,
        }
    }

    /// Change when the spring counts as settled.
    pub fn with_rest(mut self, delta: f64, speed: f64) -> Self {
        self.rest_delta = delta.abs();
        self.rest_speed = speed.abs();
        self
    }

    /// Undamped angular frequency, `sqrt(k / m)`.
    pub fn natural_frequency(&self) -> f64 {
        (self.stiffness / self.mass).sqrt()
    }

    /// `c / (2 * sqrt(k * m))`: below 1 the spring overshoots, at 1 or above it doesn't.
    pub fn damping_ratio(&self) -> f64 {
        self.damping / (2.0 * (self.stiffness * self.mass).sqrt())
    }

    /// Offset from the target and speed `t` seconds after starting `offset` away at `velocity`.
    pub fn solve(&self, offset: f64, velocity: f64, t: f64) -> (f64, f64) {
        let w0 = self.natural_frequency();
        let zeta = self.damping_ratio();
        if (zeta - 1.0).abs() < 1e-6 {
            // Critically damped.
            let b = velocity + w0 * offset;
            let e = (-w0 * t).exp();
            (e * (offset + b * t), e * (velocity - w0 * b * t))
        } else if zeta < 1.0 {
            // Under-damped: a decaying sine wave.
            let a = zeta * w0;
            let wd = w0 * (1.0 - zeta * zeta).sqrt();
            let e = (-a * t).exp();
            let (sin, cos) = (wd * t).sin_cos();
            let b = (velocity + a * offset) / wd;
            let x = e * (offset * cos + b * sin);
            let v = e * (velocity * cos - ((a * velocity + w0 * w0 * offset) / wd) * sin);
            (x, v)
        } else {
            // Over-damped: two decaying exponentials.
            let root = (zeta * zeta - 1.0).sqrt();
            let r1 = -w0 * (zeta - root);
            let r2 = -w0 * (zeta + root);
            let c1 = (velocity - r2 * offset) / (r1 - r2);
            let c2 = offset - c1;
            let (e1, e2) = ((r1 * t).exp(), (r2 * t).exp());
            (c1 * e1 + c2 * e2, r1 * c1 * e1 + r2 * c2 * e2)
        }
    }

    /// Seconds until a spring started `offset` away at `velocity` settles (capped at 10 s).
    pub fn settle_time(&self, offset: f64, velocity: f64) -> f64 {
        let mut state = SpringState::new(*self, offset, 0.0, velocity);
        let mut t = 0.0;
        while t < 10.0 && !state.is_done() {
            state.advance(1.0 / 240.0);
            t += 1.0 / 240.0;
        }
        t
    }
}

impl From<SpringPreset> for SpringConfig {
    fn from(preset: SpringPreset) -> Self {
        let s = preset.spring();
        Self::new(
            f64::from(s.stiffness),
            f64::from(s.damping),
            f64::from(s.mass),
        )
    }
}

/// A spring moving one value toward a target, solved exactly at any time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringState {
    config: SpringConfig,
    target: f64,
    /// Offset from the target and speed when this leg started.
    offset0: f64,
    velocity0: f64,
    elapsed: f64,
}

impl SpringState {
    /// Start at `from`, moving at `velocity`, toward `to`.
    pub fn new(config: SpringConfig, from: f64, to: f64, velocity: f64) -> Self {
        Self {
            config,
            target: to,
            offset0: from - to,
            velocity0: velocity,
            elapsed: 0.0,
        }
    }

    pub fn target(&self) -> f64 {
        self.target
    }

    pub fn position(&self) -> f64 {
        self.target
            + self
                .config
                .solve(self.offset0, self.velocity0, self.elapsed)
                .0
    }

    pub fn velocity(&self) -> f64 {
        self.config
            .solve(self.offset0, self.velocity0, self.elapsed)
            .1
    }

    /// Move time forward by `dt` seconds.
    pub fn advance(&mut self, dt: f64) {
        if dt.is_finite() && dt > 0.0 {
            self.elapsed += dt;
        }
    }

    /// Aim somewhere else without losing the current position or speed.
    pub fn retarget(&mut self, to: f64) {
        let (x, v) = (self.position(), self.velocity());
        *self = Self::new(self.config, x, to, v);
    }

    /// Settled: close to the target and slow.
    pub fn is_done(&self) -> bool {
        let (x, v) = self
            .config
            .solve(self.offset0, self.velocity0, self.elapsed);
        x.abs() <= self.config.rest_delta && v.abs() <= self.config.rest_speed
    }
}

/// How a thrown value coasts: exponential slow-down, optional bounds and snap points.
#[derive(Clone, Debug, PartialEq)]
pub struct GlideConfig {
    /// Seconds for the speed to fall to about 37%. Larger coasts further.
    pub time_constant: f64,
    /// Multiplies how far a throw travels (1.0 keeps the release speed exactly).
    pub power: f64,
    /// Lowest and highest resting value; a glide that reaches one bounces back with `bounce`.
    pub bounds: Option<(f64, f64)>,
    /// Resting places. The glide springs to the one nearest where it would have stopped.
    pub snap_points: Vec<f64>,
    /// The spring used for bounces and snapping.
    pub bounce: SpringConfig,
    /// Stop once this close to where the glide is heading.
    pub rest_delta: f64,
}

impl Default for GlideConfig {
    fn default() -> Self {
        Self {
            time_constant: 0.325,
            power: 1.0,
            bounds: None,
            snap_points: Vec::new(),
            bounce: SpringConfig::from(SpringPreset::Snappy),
            rest_delta: 0.5,
        }
    }
}

impl GlideConfig {
    /// Where a throw from `from` at `velocity` comes to rest, before bounds and snapping.
    pub fn projected(&self, from: f64, velocity: f64) -> f64 {
        from + velocity * self.time_constant * self.power
    }

    /// Where it really comes to rest: the nearest snap point, else clamped to the bounds.
    pub fn resting_place(&self, from: f64, velocity: f64) -> f64 {
        let ideal = self.projected(from, velocity);
        if let Some(snap) = nearest(&self.snap_points, ideal) {
            return snap;
        }
        self.clamp(ideal)
    }

    fn clamp(&self, x: f64) -> f64 {
        match self.bounds {
            Some((lo, hi)) if lo <= hi => x.clamp(lo, hi),
            _ => x,
        }
    }
}

fn nearest(points: &[f64], x: f64) -> Option<f64> {
    points
        .iter()
        .copied()
        .filter(|p| p.is_finite())
        .min_by(|a, b| (a - x).abs().total_cmp(&(b - x).abs()))
}

/// Momentum after a throw. Coasts with exponential decay; when it would leave its bounds, or
/// when it has snap points, a spring takes over with the speed it had at that moment.
#[derive(Clone, Debug, PartialEq)]
pub struct Glide {
    config: GlideConfig,
    phase: GlidePhase,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GlidePhase {
    Coast {
        origin: f64,
        amplitude: f64,
        elapsed: f64,
    },
    Spring(SpringState),
}

impl Glide {
    pub fn new(config: GlideConfig, from: f64, velocity: f64) -> Self {
        let bounce = config.bounce;
        let outside = config.clamp(from) != from;
        let phase = if !config.snap_points.is_empty() || outside {
            // Snapping, or released past an edge: spring straight to the resting place.
            let to = if outside {
                config.clamp(from)
            } else {
                config.resting_place(from, velocity)
            };
            GlidePhase::Spring(SpringState::new(bounce, from, to, velocity))
        } else {
            GlidePhase::Coast {
                origin: from,
                amplitude: velocity * config.time_constant * config.power,
                elapsed: 0.0,
            }
        };
        Self { config, phase }
    }

    /// Where it will come to rest.
    pub fn target(&self) -> f64 {
        match self.phase {
            GlidePhase::Coast {
                origin, amplitude, ..
            } => self.config.clamp(origin + amplitude),
            GlidePhase::Spring(s) => s.target(),
        }
    }

    pub fn position(&self) -> f64 {
        match self.phase {
            GlidePhase::Coast {
                origin,
                amplitude,
                elapsed,
            } => origin + amplitude * (1.0 - (-elapsed / self.config.time_constant).exp()),
            GlidePhase::Spring(s) => s.position(),
        }
    }

    pub fn velocity(&self) -> f64 {
        match self.phase {
            GlidePhase::Coast {
                amplitude, elapsed, ..
            } => {
                amplitude / self.config.time_constant * (-elapsed / self.config.time_constant).exp()
            }
            GlidePhase::Spring(s) => s.velocity(),
        }
    }

    pub fn advance(&mut self, dt: f64) {
        if !(dt.is_finite() && dt > 0.0) {
            return;
        }
        match &mut self.phase {
            GlidePhase::Coast { elapsed, .. } => {
                *elapsed += dt;
                let x = self.position();
                let clamped = self.config.clamp(x);
                if clamped != x {
                    // Hit an edge: bounce back with the speed it hit it at.
                    let v = self.velocity();
                    self.phase =
                        GlidePhase::Spring(SpringState::new(self.config.bounce, x, clamped, v));
                }
            }
            GlidePhase::Spring(s) => s.advance(dt),
        }
    }

    pub fn is_done(&self) -> bool {
        match self.phase {
            GlidePhase::Coast {
                amplitude, elapsed, ..
            } => {
                // Distance still to travel.
                let left = amplitude * (-elapsed / self.config.time_constant).exp();
                left.abs() <= self.config.rest_delta
            }
            GlidePhase::Spring(s) => s.is_done(),
        }
    }
}

/// A running animation of one value: a spring or a glide.
#[derive(Clone, Debug, PartialEq)]
pub enum Animation {
    Spring(SpringState),
    Glide(Glide),
}

impl Animation {
    pub fn position(&self) -> f64 {
        match self {
            Self::Spring(s) => s.position(),
            Self::Glide(g) => g.position(),
        }
    }
    pub fn velocity(&self) -> f64 {
        match self {
            Self::Spring(s) => s.velocity(),
            Self::Glide(g) => g.velocity(),
        }
    }
    /// Where the value ends up.
    pub fn target(&self) -> f64 {
        match self {
            Self::Spring(s) => s.target(),
            Self::Glide(g) => g.target(),
        }
    }
    pub fn advance(&mut self, dt: f64) {
        match self {
            Self::Spring(s) => s.advance(dt),
            Self::Glide(g) => g.advance(dt),
        }
    }
    pub fn is_done(&self) -> bool {
        match self {
            Self::Spring(s) => s.is_done(),
            Self::Glide(g) => g.is_done(),
        }
    }
}

/// Pointer speed from recent samples, the way native scroll views measure a throw: only the
/// last 100 ms count, so a pointer that slowed down before release isn't thrown hard.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VelocityTracker {
    /// `(time in ms, x, y)`, oldest first.
    samples: Vec<(f64, f64, f64)>,
}

/// Samples older than this (ms) don't count toward the speed.
const VELOCITY_WINDOW_MS: f64 = 100.0;

impl VelocityTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }

    /// Record where the pointer was at `t_ms`.
    pub fn push(&mut self, t_ms: f64, x: f64, y: f64) {
        if !(t_ms.is_finite() && x.is_finite() && y.is_finite()) {
            return;
        }
        if let Some(&(last, ..)) = self.samples.last() {
            if t_ms < last {
                self.samples.clear();
            }
        }
        self.samples.push((t_ms, x, y));
        let cutoff = t_ms - VELOCITY_WINDOW_MS;
        // Keep one sample at or before the window start, so the window is fully covered.
        while self.samples.len() > 2 && self.samples[1].0 <= cutoff {
            self.samples.remove(0);
        }
    }

    /// Speed in units per second at `now_ms`; zero if the pointer has been still for 100 ms.
    pub fn velocity(&self, now_ms: f64) -> (f64, f64) {
        let Some(&(t1, x1, y1)) = self.samples.last() else {
            return (0.0, 0.0);
        };
        if now_ms - t1 > VELOCITY_WINDOW_MS || self.samples.len() < 2 {
            return (0.0, 0.0);
        }
        let cutoff = t1 - VELOCITY_WINDOW_MS;
        let &(t0, x0, y0) = self
            .samples
            .iter()
            .find(|s| s.0 >= cutoff && s.0 < t1)
            .unwrap_or(&self.samples[0]);
        let dt = (t1 - t0) / 1000.0;
        if dt <= 0.0 {
            return (0.0, 0.0);
        }
        ((x1 - x0) / dt, (y1 - y0) / dt)
    }
}

/// How far an element follows a pointer dragged `overshoot` past an edge, with resistance that
/// grows the further it goes (the iOS curve). `dimension` is the size of the draggable area and
/// `elasticity` 0..1 how loose it feels (0 stops dead at the edge; 0.55 matches iOS).
pub fn rubber_band(overshoot: f64, dimension: f64, elasticity: f64) -> f64 {
    let dimension = dimension.abs().max(1.0);
    let c = elasticity.clamp(0.0, 1.0);
    if c == 0.0 || !overshoot.is_finite() {
        return 0.0;
    }
    let pulled = (1.0 - 1.0 / (overshoot.abs() * c / dimension + 1.0)) * dimension;
    pulled.copysign(overshoot)
}

/// Keep `x` inside `lo..=hi`, letting it stretch past either edge along [`rubber_band`].
pub fn rubber_band_clamp(x: f64, lo: f64, hi: f64, dimension: f64, elasticity: f64) -> f64 {
    if lo > hi {
        return x;
    }
    if x < lo {
        lo + rubber_band(x - lo, dimension, elasticity)
    } else if x > hi {
        hi + rubber_band(x - hi, dimension, elasticity)
    } else {
        x
    }
}

/// Zoom by `factor` keeping the point `anchor` fixed on screen.
///
/// The content is drawn with `transform: translate(offset) scale(scale)` and
/// `transform-origin: 0 0`; `anchor` is in the container's coordinates. Returns the new
/// `(offset, scale)`.
pub fn zoom_about(
    offset: (f64, f64),
    scale: f64,
    anchor: (f64, f64),
    factor: f64,
) -> ((f64, f64), f64) {
    if !(factor.is_finite() && factor > 0.0) {
        return (offset, scale);
    }
    let new_offset = (
        anchor.0 - (anchor.0 - offset.0) * factor,
        anchor.1 - (anchor.1 - offset.1) * factor,
    );
    (new_offset, scale * factor)
}

/// How far content `scale` times the size of a `size` container may pan in one axis so the
/// container stays covered: `(lowest offset, highest offset)`.
pub fn pan_bounds(size: f64, scale: f64) -> (f64, f64) {
    let overflow = size * (scale - 1.0);
    if overflow > 0.0 {
        (-overflow, 0.0)
    } else {
        // Smaller than the container: keep it centred.
        (-overflow / 2.0, -overflow / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(mut a: Animation, seconds: f64) -> Animation {
        let mut t = 0.0;
        while t < seconds && !a.is_done() {
            a.advance(1.0 / 60.0);
            t += 1.0 / 60.0;
        }
        a
    }

    /// Integrate the same spring numerically (1 µs semi-implicit Euler) as a reference.
    fn numeric(c: SpringConfig, x0: f64, v0: f64, t: f64) -> (f64, f64) {
        let (mut x, mut v) = (x0, v0);
        let dt = 1e-6;
        let steps = (t / dt).round() as usize;
        for _ in 0..steps {
            let a = (-c.stiffness * x - c.damping * v) / c.mass;
            v += a * dt;
            x += v * dt;
        }
        (x, v)
    }

    #[test]
    fn closed_form_matches_numeric_integration_for_every_damping_regime() {
        let configs = [
            SpringConfig::new(400.0, 14.0, 1.0), // bouncy, under-damped
            SpringConfig::new(100.0, 20.0, 1.0), // critical
            SpringConfig::new(100.0, 60.0, 2.0), // over-damped
            SpringConfig::from(SpringPreset::Gentle),
        ];
        for c in configs {
            for (x0, v0) in [(100.0, 0.0), (-40.0, 900.0), (0.0, -500.0)] {
                for t in [0.0, 0.016, 0.1, 0.35, 0.8] {
                    let (x, v) = c.solve(x0, v0, t);
                    let (nx, nv) = numeric(c, x0, v0, t);
                    assert!((x - nx).abs() < 0.01, "{c:?} x({t}) {x} vs {nx}");
                    assert!((v - nv).abs() < 0.5, "{c:?} v({t}) {v} vs {nv}");
                }
            }
        }
    }

    #[test]
    fn result_does_not_depend_on_frame_rate() {
        let c = SpringConfig::from(SpringPreset::Bouncy);
        let mut a = SpringState::new(c, 0.0, 100.0, 0.0);
        let mut b = a;
        for _ in 0..30 {
            a.advance(1.0 / 30.0);
        }
        for _ in 0..144 {
            b.advance(1.0 / 144.0);
        }
        assert!((a.position() - b.position()).abs() < 1e-6);
    }

    #[test]
    fn only_bouncy_overshoots_visibly() {
        for (preset, overshoots) in [
            (SpringPreset::Gentle, false),
            (SpringPreset::Snappy, false),
            (SpringPreset::Bouncy, true),
        ] {
            let mut s = SpringState::new(preset.into(), 0.0, 100.0, 0.0);
            let mut peak: f64 = 0.0;
            while !s.is_done() {
                s.advance(1.0 / 240.0);
                peak = peak.max(s.position());
            }
            // Snappy passes the target by about 1%; only Bouncy should go visibly past it.
            assert_eq!(peak > 105.0, overshoots, "{preset:?} peaked at {peak}");
            assert!((s.position() - 100.0).abs() <= 0.01);
        }
    }

    #[test]
    fn retargeting_keeps_position_and_speed() {
        let mut s = SpringState::new(SpringPreset::Snappy.into(), 0.0, 100.0, 0.0);
        s.advance(0.05);
        let (x, v) = (s.position(), s.velocity());
        assert!(v > 0.0);
        s.retarget(-50.0);
        assert!((s.position() - x).abs() < 1e-9);
        assert!((s.velocity() - v).abs() < 1e-9);
        assert_eq!(s.target(), -50.0);
    }

    #[test]
    fn settle_time_is_finite_and_grows_with_distance() {
        let c = SpringConfig::from(SpringPreset::Snappy);
        let short = c.settle_time(10.0, 0.0);
        let long = c.settle_time(1000.0, 0.0);
        assert!(short > 0.0 && long < 10.0 && long > short, "{short} {long}");
    }

    #[test]
    fn glide_starts_at_the_release_speed_and_stops_where_projected() {
        let config = GlideConfig::default();
        let g = Glide::new(config.clone(), 0.0, 1000.0);
        assert!((g.velocity() - 1000.0).abs() < 1e-9);
        let end = run(Animation::Glide(g), 5.0);
        let expected = config.projected(0.0, 1000.0);
        assert!(
            (end.position() - expected).abs() <= 0.5,
            "{}",
            end.position()
        );
        assert!((expected - 325.0).abs() < 1e-9);
    }

    #[test]
    fn glide_bounces_off_bounds_and_rests_on_the_edge() {
        let config = GlideConfig {
            bounds: Some((0.0, 100.0)),
            ..GlideConfig::default()
        };
        let g = Glide::new(config, 50.0, 2000.0);
        assert_eq!(g.target(), 100.0);
        let mut a = Animation::Glide(g);
        let mut peak: f64 = 0.0;
        let mut t = 0.0;
        while !a.is_done() && t < 5.0 {
            a.advance(1.0 / 120.0);
            peak = peak.max(a.position());
            t += 1.0 / 120.0;
        }
        assert!(
            peak > 100.0,
            "should pass the edge a little before bouncing back"
        );
        assert!((a.position() - 100.0).abs() <= 0.01, "{}", a.position());
    }

    #[test]
    fn glide_released_past_an_edge_springs_back_inside() {
        let config = GlideConfig {
            bounds: Some((0.0, 100.0)),
            ..GlideConfig::default()
        };
        let end = run(Animation::Glide(Glide::new(config, -30.0, 0.0)), 5.0);
        assert!(end.position().abs() <= 0.01, "{}", end.position());
    }

    #[test]
    fn glide_snaps_to_the_point_nearest_its_projected_landing() {
        let config = GlideConfig {
            snap_points: vec![0.0, 200.0, 400.0],
            ..GlideConfig::default()
        };
        // A gentle push from 0 projects to 97.5: nearest is 0, it springs back.
        assert_eq!(config.resting_place(0.0, 300.0), 0.0);
        // A harder throw projects to 292.5: nearest is 200.
        assert_eq!(config.resting_place(0.0, 900.0), 200.0);
        let end = run(Animation::Glide(Glide::new(config, 0.0, 900.0)), 5.0);
        assert!((end.position() - 200.0).abs() <= 0.01);
    }

    #[test]
    fn velocity_uses_only_the_last_100_ms() {
        let mut v = VelocityTracker::new();
        // Slow for a long time, then fast for the last 50 ms.
        for i in 0..20 {
            v.push(f64::from(i) * 20.0, f64::from(i), 0.0);
        }
        v.push(400.0, 20.0, 0.0);
        v.push(425.0, 45.0, 0.0);
        v.push(450.0, 70.0, 0.0);
        let (vx, vy) = v.velocity(450.0);
        // 100 ms window: from (360, 18) to (450, 70) => 52 px / 0.09 s.
        assert!((vx - 577.7).abs() < 1.0, "{vx}");
        assert_eq!(vy, 0.0);
    }

    #[test]
    fn velocity_is_zero_after_a_pause_or_with_one_sample() {
        let mut v = VelocityTracker::new();
        v.push(0.0, 0.0, 0.0);
        assert_eq!(v.velocity(0.0), (0.0, 0.0));
        v.push(10.0, 10.0, 5.0);
        assert!(v.velocity(15.0).0 > 0.0);
        assert_eq!(v.velocity(500.0), (0.0, 0.0));
    }

    #[test]
    fn rubber_band_resists_more_the_further_it_goes() {
        let d = 300.0;
        assert_eq!(rubber_band(0.0, d, 0.55), 0.0);
        let a = rubber_band(50.0, d, 0.55);
        let b = rubber_band(500.0, d, 0.55);
        assert!(a > 0.0 && a < 50.0);
        assert!(
            b > a && b < d,
            "never travels a full container past the edge"
        );
        assert_eq!(rubber_band(-50.0, d, 0.55), -a);
        assert_eq!(rubber_band(50.0, d, 0.0), 0.0);
        assert_eq!(rubber_band_clamp(50.0, 0.0, 100.0, d, 0.55), 50.0);
        assert_eq!(rubber_band_clamp(150.0, 0.0, 100.0, d, 0.0), 100.0);
    }

    #[test]
    fn zooming_keeps_the_anchor_point_still() {
        let (offset, scale) = ((10.0, -20.0), 1.5);
        let anchor = (120.0, 80.0);
        // The content point under the anchor before zooming...
        let content = ((anchor.0 - offset.0) / scale, (anchor.1 - offset.1) / scale);
        let (o2, s2) = zoom_about(offset, scale, anchor, 2.0);
        assert_eq!(s2, 3.0);
        // ...is still under it afterwards.
        assert!((o2.0 + content.0 * s2 - anchor.0).abs() < 1e-9);
        assert!((o2.1 + content.1 * s2 - anchor.1).abs() < 1e-9);
        assert_eq!(zoom_about(offset, scale, anchor, 0.0), (offset, scale));
    }

    #[test]
    fn pan_bounds_keep_the_container_covered() {
        assert_eq!(pan_bounds(200.0, 1.0), (0.0, 0.0));
        assert_eq!(pan_bounds(200.0, 2.0), (-200.0, 0.0));
        assert_eq!(pan_bounds(200.0, 0.5), (50.0, 50.0));
    }

    #[test]
    fn bad_input_never_produces_nan() {
        let mut s = SpringState::new(SpringConfig::new(0.0, 0.0, 0.0), 0.0, 1.0, 0.0);
        s.advance(f64::NAN);
        s.advance(0.016);
        assert!(s.position().is_finite());
        let mut v = VelocityTracker::new();
        v.push(f64::NAN, 1.0, 1.0);
        assert_eq!(v.velocity(0.0), (0.0, 0.0));
        assert!(rubber_band(f64::INFINITY, 100.0, 0.5) == 0.0);
    }
}
