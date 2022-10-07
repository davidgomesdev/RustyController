use std::time::Duration;

use juniper::{FieldError, FieldResult, Value};
use tokio::time::Instant;

use crate::{EffectChange, EffectChangeType, EffectTarget, LedEffect};
use crate::graphql::schema::Context;
use crate::graphql::schema_input::*;
use crate::graphql::schema_response::MutationResponse;
use crate::ps_move::api::build_hsv;
use crate::ps_move::models::RumbleEffect;

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Turn the led off.")]
    fn set_led_off(ctx: &Context, input: Option<OffEffectInput>) -> FieldResult<MutationResponse> {
        let controllers = input.map_or(None, |input| Some(input.controllers));
        process_led_effect_mutation(ctx, LedEffect::Off, controllers)
    }

    #[graphql(description = "Set a constant color.")]
    fn set_led_static(ctx: &Context, input: StaticLedEffectInput) -> FieldResult<MutationResponse> {
        if input.hue < 0.0 || input.hue > 360.0 {
            return Err(FieldError::new(
                "Hue must be between 0.0 and 360.0!",
                Value::Null,
            ));
        }
        if input.saturation < 0.0 || input.saturation > 1.0 {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.value < 0.0 || input.value > 1.0 {
            return Err(FieldError::new(
                "Value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let effect = LedEffect::Static {
            hsv: build_hsv(input.hue, input.saturation, input.value),
        };

        process_led_effect_mutation(ctx, effect, input.controllers)
    }

    #[graphql(
    description = "Increase brightness of a color over time, reaching a peak, then reverting."
    )]
    fn set_led_breathing(
        ctx: &Context,
        input: BreathingLedEffectInput,
    ) -> FieldResult<MutationResponse> {
        if input.step < 0.0 || input.step > 1.0 {
            return Err(FieldError::new(
                "Step must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.initial_value > input.peak {
            return Err(FieldError::new(
                "Initial value can't be higher than peak!",
                Value::Null,
            ));
        }

        if input.hue < 0.0 || input.hue > 360.0 {
            return Err(FieldError::new(
                "Hue must be between 0.0 and 360.0!",
                Value::Null,
            ));
        }
        if input.saturation < 0.0 || input.saturation > 1.0 {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.initial_value < 0.0 || input.initial_value > 1.0 {
            return Err(FieldError::new(
                "Initial value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.peak < 0.0 || input.peak > 1.0 {
            return Err(FieldError::new(
                "Peak must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let effect = LedEffect::Breathing {
            initial_hsv: build_hsv(input.hue, input.saturation, input.initial_value),
            step: input.step as f32,
            peak: input.peak as f32,
            inhaling: true,
        };

        process_led_effect_mutation(ctx, effect, input.controllers)
    }

    #[graphql(description = "Cycle through colors.")]
    fn set_led_rainbow(
        ctx: &Context,
        input: RainbowLedEffectInput,
    ) -> FieldResult<MutationResponse> {
        if input.step < 0.0 || input.step > 1.0 {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.saturation < 0.0 || input.saturation > 1.0 {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.value < 0.0 || input.value > 1.0 {
            return Err(FieldError::new(
                "Value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let effect = LedEffect::Rainbow {
            saturation: input.saturation as f32,
            value: input.value as f32,
            step: input.step as f32,
        };

        process_led_effect_mutation(ctx, effect, input.controllers)
    }

    #[graphql(description = "Alternate between color and off.")]
    fn set_led_blink(
        ctx: &Context,
        input: BlinkLedEffectInput,
    ) -> FieldResult<MutationResponse> {
        if input.hue < 0.0 || input.hue > 360.0 {
            return Err(FieldError::new(
                "Hue must be between 0.0 and 360.0!",
                Value::Null,
            ));
        }
        if input.saturation < 0.0 || input.saturation > 1.0 {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.value < 0.0 || input.value > 1.0 {
            return Err(FieldError::new(
                "Value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.interval < 0 {
            return Err(FieldError::new(
                "Interval must be positive!",
                Value::Null,
            ));
        }

        let effect = LedEffect::Blink {
            hsv: build_hsv(input.hue, input.saturation, input.value),
            interval: Duration::from_millis(input.interval as u64),
            start: Instant::now(),
        };

        process_led_effect_mutation(ctx, effect, input.controllers)
    }

    #[graphql(description = "Turn rumble off.")]
    fn set_rumble_off(ctx: &Context, input: Option<OffEffectInput>) -> FieldResult<MutationResponse> {
        let controllers = input.map_or(None, |input| Some(input.controllers));
        process_rumble_effect_mutation(ctx, RumbleEffect::Off, controllers)
    }

    #[graphql(description = "Set a constant rumble.")]
    fn set_rumble_static(
        ctx: &Context,
        input: StaticRumbleEffectInput,
    ) -> FieldResult<MutationResponse> {
        if input.strength < 0.0 || input.strength > 1.0 {
            return Err(FieldError::new(
                "Strength must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let effect = RumbleEffect::Static {
            strength: input.strength as f32,
        };

        process_rumble_effect_mutation(ctx, effect, input.controllers)
    }

    #[graphql(description = "Increase rumble strength over time, reaching a peak, then reverting.")]
    fn set_rumble_breathing(
        ctx: &Context,
        input: BreathingRumbleEffectInput,
    ) -> FieldResult<MutationResponse> {
        if input.step < 0.0 || input.step > 1.0 {
            return Err(FieldError::new(
                "Step must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.initial_strength > input.peak {
            return Err(FieldError::new(
                "Initial strength can't be higher than peak!",
                Value::Null,
            ));
        }

        if input.initial_strength < 0.0 || input.initial_strength > 1.0 {
            return Err(FieldError::new(
                "Initial strength must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }
        if input.peak < 0.0 || input.peak > 1.0 {
            return Err(FieldError::new(
                "Peak must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let effect = RumbleEffect::Breathing {
            initial_strength: input.initial_strength as f32,
            step: input.step as f32,
            peak: input.peak as f32,
            inhaling: true,
        };

        process_rumble_effect_mutation(ctx, effect, input.controllers)
    }
}

fn process_led_effect_mutation(
    ctx: &Context,
    effect: LedEffect,
    target: Option<Vec<String>>,
) -> FieldResult<MutationResponse> {
    process_effect_mutation(ctx, EffectChangeType::Led { effect }, target)
}

fn process_rumble_effect_mutation(
    ctx: &Context,
    effect: RumbleEffect,
    target: Option<Vec<String>>,
) -> FieldResult<MutationResponse> {
    process_effect_mutation(ctx, EffectChangeType::Rumble { effect }, target)
}

fn process_effect_mutation(
    ctx: &Context,
    effect: EffectChangeType,
    target: Option<Vec<String>>,
) -> FieldResult<MutationResponse> {
    let target = match target {
        None => EffectTarget::All,
        Some(bt_addresses) => {
            if bt_addresses.is_empty() {
                return Err(FieldError::new(
                    "You must specify controllers!",
                    Value::Null,
                ));
            } else {
                EffectTarget::Only { bt_addresses }
            }
        }
    };

    return match ctx.tx.send(EffectChange { effect, target }) {
        Ok(_) => Ok(MutationResponse::Success),
        Err(_) => Ok(MutationResponse::ServerError),
    };
}
