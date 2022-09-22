use std::sync::Arc;

use juniper::{EmptySubscription, FieldError, RootNode, Value};
use juniper::{FieldResult, GraphQLEnum, GraphQLInputObject};
use tokio::sync::watch::Sender;

use crate::services::ps_move_api::{build_hsv, LedEffect};

pub struct Context {
    pub tx: Arc<Sender<LedEffect>>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Check the service health")]
    fn health(_ctx: &Context) -> FieldResult<HealthStatus> {
        Ok(HealthStatus::Ok)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Turn the led off.")]
    fn off(ctx: &Context) -> FieldResult<MutationResponse> {
        return match ctx.tx.send(LedEffect::Off) {
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

        return match ctx.tx.send(effect) {
            Ok(_) => Ok(MutationResponse::Success),
            Err(_) => Ok(MutationResponse::ServerError),
        };
    }

    #[graphql(description = "Increase/decrease brightness of a color.")]
    fn breathing(
        ctx: &Context,
        input: BreathingEffectInput
    ) -> FieldResult<MutationResponse> {
        if input.step > input.peak {
            return Err(FieldError::new(
                "Step can't be higher than peak!",
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

        return match ctx.tx.send(effect) {
            Ok(_) => Ok(MutationResponse::Success),
            Err(_) => Ok(MutationResponse::ServerError),
        };
    }

    #[graphql(description = "Cycle through colors.")]
    fn rainbow(ctx: &Context, input: RainbowEffectInput) -> FieldResult<MutationResponse> {
        if input.step > 360.0 {
            return Err(FieldError::new(
                "Step can't be higher than max hue (360)!",
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

        return match ctx.tx.send(effect) {
            Ok(_) => Ok(MutationResponse::Success),
            Err(_) => Ok(MutationResponse::ServerError),
        };
    }
}

#[derive(GraphQLEnum)]
enum HealthStatus {
    Ok,
    Error,
}

#[derive(GraphQLEnum)]
enum MutationResponse {
    Success,
    ServerError,
}

#[derive(GraphQLInputObject)]
struct StaticEffectInput {
    hue: f64,
    saturation: f64,
    value: f64,
}

#[derive(GraphQLInputObject)]
struct BreathingEffectInput {
    hue: f64,
    saturation: f64,
    initial_value: f64,
    #[graphql(description = "Amount that the value changes per update.")]
    step: f64,
    #[graphql(description = "Defines the max value/brightness that the controller breathes to.")]
    peak: f64,
}

#[derive(GraphQLInputObject)]
struct RainbowEffectInput {
    saturation: f64,
    value: f64,
    #[graphql(description = "Amount that the hue/color changes per update.")]
    step: f64,
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
