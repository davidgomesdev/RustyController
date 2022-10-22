use std::fmt;

use lazy_static::lazy_static;
use log::error;
use palette::{Hsv, Hue};
use strum_macros::Display;
use tokio::time::{Duration, Instant};

use crate::tasks::effects_update;

lazy_static! {
    static ref LED_OFF: Hsv = Hsv::from_components((0.0, 0.0, 0.0));
}

#[derive(Clone, Copy)]
pub struct LedEffect {
    pub details: LedEffectDetails,
    pub start: Instant,
    pub duration: Option<Duration>,
}

impl LedEffect {
    pub fn new_expiring(details: LedEffectDetails, duration: Duration) -> LedEffect {
        LedEffect {
            details,
            start: Instant::now(),
            duration: Some(duration),
        }
    }

    pub fn new(details: LedEffectDetails) -> LedEffect {
        LedEffect {
            details,
            start: Instant::now(),
            duration: None,
        }
    }

    pub fn off() -> LedEffect {
        LedEffect {
            details: LedEffectDetails::Off,
            start: Instant::now(),
            duration: None,
        }
    }

    /// Creates an expiring `LedEffect` if `duration_millis` is present,
    /// otherwise a non-expiring one
    pub fn from(details: LedEffectDetails, duration_millis: Option<i32>) -> LedEffect {
        duration_millis.map_or(LedEffect::new(details), |millis| {
            if millis < 0 {
                panic!("Negative milliseconds as duration not allowed!")
            }

            LedEffect::new_expiring(details, Duration::from_millis(millis as u64))
        })
    }

    pub fn is_off(&self) -> bool {
        self.details == LedEffectDetails::Off
    }

    pub fn has_expired(&self) -> bool {
        if let Some(duration) = self.duration {
            self.start.elapsed() > duration
        } else {
            false
        }
    }
}

impl fmt::Display for LedEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Led::{}", &self.details)
    }
}

#[derive(Clone, Copy, Display, Debug, PartialEq)]
pub enum LedEffectDetails {
    Off,
    Static {
        hsv: Hsv,
    },
    Breathing {
        initial_hsv: Hsv,
        step: f32,
        peak: f32,
        inhaling: bool,
    },
    Rainbow {
        saturation: f32,
        value: f32,
        step: f32,
    },
    Blink {
        hsv: Hsv,
        interval: Duration,
        last_blink: Instant,
    },
}

#[derive(Clone, Copy, Display, Debug)]
pub enum RumbleEffect {
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
}

impl LedEffectDetails {
    pub fn new_timed_breathing(
        initial_hsv: Hsv,
        time_to_peak: Duration,
        peak: f32,
    ) -> LedEffectDetails {
        let time_to_peak = time_to_peak.as_millis() as f32;
        let step = effects_update::INTERVAL_DURATION.as_millis() as f32 / time_to_peak;

        LedEffectDetails::Breathing {
            initial_hsv,
            step,
            peak,
            inhaling: initial_hsv.value < peak,
        }
    }

    pub fn get_initial_hsv(&self) -> Hsv {
        match *self {
            LedEffectDetails::Off => Hsv::from_components((0.0, 0.0, 0.0)),
            LedEffectDetails::Static { hsv }
            | LedEffectDetails::Blink {
                hsv,
                interval: _,
                last_blink: _,
            } => hsv,
            LedEffectDetails::Breathing {
                initial_hsv,
                step,
                peak,
                ..
            } => {
                if step < 0.0 || step > 1.0 {
                    error!("Step must be between 0.0 and 1.0")
                }

                if peak < initial_hsv.value {
                    error!("Peak must be higher than initial value")
                }

                initial_hsv
            }
            LedEffectDetails::Rainbow {
                saturation,
                value,
                step,
            } => {
                if step > 360.0 {
                    error!("Step can't be higher than 360 (max hue)")
                }

                Hsv::from_components((0.0, saturation, value))
            }
        }
    }

    pub fn get_updated_hsv(&mut self, current_hsv: Hsv) -> Hsv {
        match *self {
            LedEffectDetails::Off => *LED_OFF,
            LedEffectDetails::Static { hsv } => hsv,
            LedEffectDetails::Breathing {
                initial_hsv,
                step,
                peak,
                ref mut inhaling,
            } => Self::get_updated_breathing_hsv(current_hsv, initial_hsv, step, peak, inhaling),
            LedEffectDetails::Rainbow {
                saturation: _,
                value: _,
                step,
            } => {
                // no need to use [saturation] and [value],
                // since it was already set in the beginning similar to breathing,
                // the step is relative to the max possible value
                current_hsv.shift_hue(step * 360.0)
            }
            LedEffectDetails::Blink {
                hsv,
                interval,
                last_blink: ref mut start,
            } => {
                if start.elapsed() > interval / 2 {
                    *start = Instant::now();

                    if current_hsv.value == 0.0 {
                        hsv
                    } else {
                        *LED_OFF
                    }
                } else {
                    current_hsv
                }
            }
        }
    }

    fn get_updated_breathing_hsv(
        current_hsv: Hsv,
        initial_hsv: Hsv,
        step: f32,
        peak: f32,
        inhaling: &mut bool,
    ) -> Hsv {
        let initial_value = initial_hsv.value;

        let mut new_hsv = current_hsv.clone();
        let mut new_value = new_hsv.value;

        if *inhaling {
            new_value += step * (peak - initial_value)
        } else {
            new_value -= step * (peak - initial_value)
        }

        if new_value >= peak {
            new_value = peak;
            *inhaling = false
        } else if new_value <= initial_value {
            new_value = initial_value;
            *inhaling = true
        }

        new_hsv.value = new_value;
        new_hsv
    }
}

impl RumbleEffect {
    pub fn get_updated_rumble(&mut self, mut current_rumble: f32) -> f32 {
        match *self {
            RumbleEffect::Off => 0.0,
            RumbleEffect::Static { strength: value } => value,
            RumbleEffect::Breathing {
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
        }
    }
}
