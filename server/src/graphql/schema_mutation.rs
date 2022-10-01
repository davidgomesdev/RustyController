use juniper::{FieldError, FieldResult, Value};

use crate::{EffectTarget, LedEffect, LedEffectChange};
use crate::graphql::schema::Context;
use crate::graphql::schema_input::*;
use crate::graphql::schema_response::MutationResponse;
use crate::ps_move::api::build_hsv;

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Turn the led off.")]
    fn set_led_off(ctx: &Context, input: OffEffectInput) -> FieldResult<MutationResponse> {
        process_effect_mutation(ctx, LedEffect::Off, input.controllers)
    }

    #[graphql(description = "Set a constant color.")]
    fn set_led_static(ctx: &Context, input: StaticEffectInput) -> FieldResult<MutationResponse> {
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

        process_effect_mutation(ctx, effect, input.controllers)
    }

    #[graphql(description = "Increase/decrease brightness of a color over time.")]
    fn set_led_breathing(ctx: &Context, input: BreathingEffectInput) -> FieldResult<MutationResponse> {
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

        process_effect_mutation(ctx, effect, input.controllers)
    }

    #[graphql(description = "Cycle through colors.")]
    fn set_led_rainbow(ctx: &Context, input: RainbowEffectInput) -> FieldResult<MutationResponse> {
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

        process_effect_mutation(ctx, effect, input.controllers)
    }
}

fn process_effect_mutation(
    ctx: &Context,
    effect: LedEffect,
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

    return match ctx.tx.send(LedEffectChange { effect, target }) {
        Ok(_) => Ok(MutationResponse::Success),
        Err(_) => Ok(MutationResponse::ServerError),
    };
}
