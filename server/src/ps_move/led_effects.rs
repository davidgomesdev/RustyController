use std::fmt;
use std::fmt::Formatter;

use lazy_static::lazy_static;
use palette::{Hsv, ShiftHue};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use strum_macros::Display;
use tokio::time::{Duration, Instant};

use crate::tasks::effects_update;

lazy_static! {
    static ref LED_OFF: Hsv = Hsv::from_components((0.0, 0.0, 0.0));
}

const MAX_HUE_VALUE: f32 = 360.0;

#[derive(Clone, Copy)]
pub struct LedEffect {
    pub kind: LedEffectKind,
    pub start: Instant,
    pub duration: Option<Duration>,
}

impl LedEffect {
    pub fn new_expiring(kind: LedEffectKind, duration: Duration) -> LedEffect {
        LedEffect {
            kind,
            start: Instant::now(),
            duration: Some(duration),
        }
    }

    pub fn new(kind: LedEffectKind) -> LedEffect {
        LedEffect {
            kind,
            start: Instant::now(),
            duration: None,
        }
    }

    pub fn off() -> LedEffect {
        LedEffect {
            kind: LedEffectKind::Off,
            start: Instant::now(),
            duration: None,
        }
    }

    /// Creates an expiring `LedEffect` if `duration_millis` is present,
    /// otherwise a non-expiring one
    pub fn from(kind: LedEffectKind, duration_millis: Option<i32>) -> LedEffect {
        duration_millis.map_or(LedEffect::new(kind), |millis| {
            if millis < 0 {
                panic!("Negative milliseconds as duration not allowed!")
            }

            LedEffect::new_expiring(kind, Duration::from_millis(millis as u64))
        })
    }

    pub fn is_off(&self) -> bool {
        self.kind == LedEffectKind::Off
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Led::{}", &self.kind)
    }
}

#[derive(Clone, Copy, Display, Debug, PartialEq)]
pub enum LedEffectKind {
    Off,
    Static {
        hsv: Hsv,
    },
    Breathing {
        initial_hsv: Hsv,
        time_to_peak: i32,
        peak: f32,
        inhaling: bool,
        last_update: Instant,
    },
    Rainbow {
        saturation: f32,
        value: f32,
        time_to_complete: f32,
    },
    Blink {
        hsv: Hsv,
        interval: Duration,
        last_blink: Instant,
    },
    Candle {
        hue: f32,
        saturation: f32,
        value_sample: Uniform<f32>,
        interval: i32,
        last_change: Instant,
    },
}

impl LedEffectKind {
    /// Creates an instance with `LedEffect::Breathing` having `step`
    /// according to `time_to_peak` and tick rate
    pub fn new_timed_breathing(
        initial_hsv: Hsv,
        time_to_peak: Duration,
        peak: f32,
    ) -> LedEffectKind {
        let time_to_peak = time_to_peak.as_millis() as i32;

        LedEffectKind::Breathing {
            initial_hsv,
            time_to_peak,
            peak,
            inhaling: initial_hsv.value < peak,
            last_update: Instant::now(),
        }
    }

    /// Creates an instance with `LedEffect::Rainbow` having `step`
    /// according to `time_to_peak` and tick rate
    pub fn new_timed_rainbow(
        saturation: f32,
        value: f32,
        time_to_peak: Duration,
    ) -> LedEffectKind {
        let time_to_peak = time_to_peak.as_millis() as f32;
        let step =
            effects_update::INTERVAL_DURATION.as_millis() as f32 * MAX_HUE_VALUE / time_to_peak;

        LedEffectKind::Rainbow {
            saturation,
            value,
            time_to_complete: step,
        }
    }

    pub fn new_candle(
        hue: f32,
        saturation: f32,
        min_value: f32,
        max_value: f32,
        interval: Option<i32>
    ) -> LedEffectKind {
        let value_sample = Uniform::new_inclusive(
            min_value,
            max_value,
        );

        LedEffectKind::Candle {
            hue,
            saturation,
            value_sample,
            interval: interval.unwrap_or(1),
            last_change: Instant::now(),
        }
    }

    pub fn get_initial_hsv(&self) -> Hsv {
        match *self {
            LedEffectKind::Off => Hsv::from_components((0.0, 0.0, 0.0)),
            LedEffectKind::Static { hsv }
            | LedEffectKind::Blink {
                hsv,
                interval: _,
                last_blink: _,
            } => hsv,
            LedEffectKind::Breathing {
                initial_hsv, peak, ..
            } => {
                if peak < initial_hsv.value {
                    tracing::error!("Peak must be higher than initial value")
                }

                initial_hsv
            }
            LedEffectKind::Rainbow {
                saturation,
                value,
                time_to_complete: step,
            } => {
                if step > 360.0 {
                    tracing::error!("Step can't be higher than 360 (max hue)")
                }

                Hsv::from_components((0.0, saturation, value))
            }
            LedEffectKind::Candle {
                hue,
                saturation,
                value_sample,
                ..
            } => Hsv::from_components((hue, saturation, value_sample.sample(&mut thread_rng()))),
        }
    }

    pub fn get_updated_hsv(&mut self, current_hsv: Hsv) -> Hsv {
        match *self {
            LedEffectKind::Off => *LED_OFF,
            LedEffectKind::Static { hsv } => hsv,
            LedEffectKind::Breathing {
                initial_hsv,
                time_to_peak,
                peak,
                ref mut inhaling,
                ref mut last_update,
            } => Self::get_updated_breathing_hsv(
                initial_hsv,
                last_update,
                time_to_peak as f32,
                peak,
                inhaling,
            ),
            LedEffectKind::Rainbow {
                time_to_complete,
                ..
            } => {
                // no need to use [saturation] and [value],
                // since it was already set in the beginning similar to breathing,
                // the step is relative to the max possible value
                current_hsv.shift_hue(time_to_complete)
            }
            LedEffectKind::Blink {
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
            LedEffectKind::Candle {
                hue,
                saturation,
                value_sample,
                interval,
                ref mut last_change
            } => {
                if last_change.elapsed().as_millis() as i32 > interval {
                    *last_change = Instant::now();

                    let new_value = value_sample
                        .sample(&mut thread_rng());

                    Hsv::from_components((hue, saturation, new_value))
                } else {
                    current_hsv
                }
            }
        }
    }

    fn get_updated_breathing_hsv(
        initial_hsv: Hsv,
        last_update: &mut Instant,
        time_to_peak: f32,
        peak: f32,
        inhaling: &mut bool,
    ) -> Hsv {
        let initial_value = initial_hsv.value;
        let time_elapsed = (*last_update).elapsed().as_millis() as f32;
        let factor = (time_elapsed / time_to_peak).powf(2.0);

        let mut new_value = if *inhaling {
            initial_value + (peak - initial_value) * factor
        } else {
            initial_value - (initial_value - peak) * (1.0 - factor)
        };

        if *inhaling && new_value >= peak {
            *last_update = Instant::now();
            new_value = peak;
            *inhaling = false;
        } else if !*inhaling && new_value <= initial_value {
            *last_update = Instant::now();
            new_value = initial_value;
            *inhaling = true;
        }

        Hsv::from_components((initial_hsv.hue, initial_hsv.saturation, new_value))
    }
}
