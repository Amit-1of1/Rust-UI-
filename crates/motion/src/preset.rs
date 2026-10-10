use dioxus_motion::prelude::{AnimationConfig, Spring};

/// Named spring feels. Tuned by eye against Motion's defaults; adjust here, in one place.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpringPreset {
    /// Soft and slow, with almost no overshoot. Good for large surfaces.
    Gentle,
    /// Quick and settled. The default for overlays and menus.
    #[default]
    Snappy,
    /// Visible overshoot. Use sparingly, for playful feedback.
    Bouncy,
}

impl SpringPreset {
    /// The spring physics for this preset.
    pub fn spring(self) -> Spring {
        match self {
            Self::Gentle => Spring {
                stiffness: 120.0,
                damping: 20.0,
                mass: 1.0,
            },
            Self::Snappy => Spring {
                stiffness: 300.0,
                damping: 28.0,
                mass: 1.0,
            },
            Self::Bouncy => Spring {
                stiffness: 400.0,
                damping: 14.0,
                mass: 1.0,
            },
        }
    }
}

/// A spring preset that knows how to stand down for reduced motion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transition {
    preset: SpringPreset,
}

impl Transition {
    pub fn new(preset: SpringPreset) -> Self {
        Self { preset }
    }

    /// The animation config to use: the spring, or a near-instant tween if the user
    /// asked the OS to reduce motion.
    pub fn resolve(self, reduced_motion: bool) -> AnimationConfig {
        if reduced_motion {
            AnimationConfig::tween_ms(1)
        } else {
            AnimationConfig::spring(self.preset.spring())
        }
    }
}
