use strum_macros::Display;
use std::time::Duration;
use tokio::time::Instant;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy)]
pub struct RumbleEffect {
    pub kind: RumbleEffectKind,
    pub start: Instant,
    pub duration: Option<Duration>,
}

impl RumbleEffect {
    pub fn new_expiring(kind: RumbleEffectKind, duration: Duration) -> RumbleEffect {
        RumbleEffect {
            kind,
            start: Instant::now(),
            duration: Some(duration),
        }
    }

    pub fn new(kind: RumbleEffectKind) -> RumbleEffect {
        RumbleEffect {
            kind,
            start: Instant::now(),
            duration: None,
        }
    }

    pub fn off() -> RumbleEffect {
        RumbleEffect {
            kind: RumbleEffectKind::Off,
            start: Instant::now(),
            duration: None,
        }
    }

    /// Creates an expiring `RumbleEffect` if `duration_millis` is present,
    /// otherwise a non-expiring one
    pub fn from(kind: RumbleEffectKind, duration_millis: Option<i32>) -> RumbleEffect {
        duration_millis.map_or(RumbleEffect::new(kind), |millis| {
            if millis < 0 {
                panic!("Negative milliseconds as duration not allowed!")
            }

            RumbleEffect::new_expiring(kind, Duration::from_millis(millis as u64))
        })
    }
}

impl fmt::Display for RumbleEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Rumble::{}", &self.kind)
    }
}

#[derive(Clone, Copy, Display, Debug, PartialEq)]
pub enum RumbleEffectKind {
    Off,
    Static {
        strength: f32,
    },
    Breathing {
        initial_strength: f32,
        step: f32,
        peak: f32,
        inhaling: bool,
    },
    Blink {
        strength: f32,
        interval: Duration,
        last_blink: Instant,
    },
}

impl RumbleEffectKind {
    pub fn get_updated_rumble(&mut self, mut current_rumble: f32) -> f32 {
        match *self {
            RumbleEffectKind::Off => 0.0,
            RumbleEffectKind::Static { strength: value } => value,
            RumbleEffectKind::Breathing {
                initial_strength: initial,
                step,
                peak,
                ref mut inhaling,
            } => {
                if *inhaling {
                    current_rumble += step * peak
                } else {
                    current_rumble -= step * peak
                }

                if current_rumble >= peak {
                    current_rumble = peak;
                    *inhaling = false
                } else if current_rumble <= initial {
                    current_rumble = initial;
                    *inhaling = true
                }

                current_rumble
            }
            RumbleEffectKind::Blink {
                strength,
                interval,
                last_blink: ref mut start,
            } => {
                if start.elapsed() > interval / 2 {
                    *start = Instant::now();

                    if current_rumble == 0.0 {
                        strength
                    } else {
                        0.0
                    }
                } else {
                    current_rumble
                }
            }
        }
    }
}
