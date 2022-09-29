use juniper::{FieldError, FieldResult, Value};

use crate::graphql::schema::Context;
use crate::graphql::schema_input::*;
use crate::graphql::schema_response::MutationResponse;
use crate::LedEffect;
use crate::LedEffectChange::All;
use crate::ps_move::api::build_hsv;

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Turn the led off.")]
    fn off(ctx: &Context) -> FieldResult<MutationResponse> {
        return match ctx.tx.send(All { effect: LedEffect::Off }) {
            Ok(_) => Ok(MutationResponse::Success),
            Err(_) => Ok(MutationResponse::ServerError),
        };
    }

    #[graphql(description = "Set a constant color.")]
    fn r#static(ctx: &Context, input: StaticEffectInput) -> FieldResult<MutationResponse> {
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

        return match ctx.tx.send(All { effect }) {
            Ok(_) => Ok(MutationResponse::Success),
            Err(_) => Ok(MutationResponse::ServerError),
        };
    }

    #[graphql(description = "Increase/decrease brightness of a color over time.")]
    fn breathing(
        ctx: &Context,
        input: BreathingEffectInput,
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

        return match ctx.tx.send(All { effect }) {
            Ok(_) => Ok(MutationResponse::Success),
            Err(_) => Ok(MutationResponse::ServerError),
        };
    }

    #[graphql(description = "Cycle through colors.")]
    fn rainbow(ctx: &Context, input: RainbowEffectInput) -> FieldResult<MutationResponse> {
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

        return match ctx.tx.send(All { effect }) {
            Ok(_) => Ok(MutationResponse::Success),
            Err(_) => Ok(MutationResponse::ServerError),
        };
    }
}