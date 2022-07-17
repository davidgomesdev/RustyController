use std::sync::Arc;

use juniper::{EmptySubscription, FieldError, RootNode, Value};
use juniper::{FieldResult, GraphQLEnum};
use tokio::sync::watch::Sender;

use crate::ps_move_api::{build_hsv, LedEffect};

pub struct Context {
    pub tx: Arc<Sender<LedEffect>>,
}

impl juniper::Context for Context {}

#[derive(GraphQLEnum)]
enum HealthStatus {
    Ok,
    Error,
}

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
    #[graphql(description = "Turn led off")]
    fn off(ctx: &Context) -> FieldResult<i32> {
        return match ctx.tx.send(LedEffect::Off) {
            Ok(_) => Ok(0),
            Err(_) => Ok(1),
        };
    }

    #[graphql(description = "Keep led in the specified setting")]
    fn r#static(ctx: &Context, h: f64, s: f64, v: f64) -> FieldResult<i32> {
        if h < 0.0 || h > 360.0 {
            return Err(FieldError::new("Hue must be between 0.0 and 360.0!", Value::Null))
        }
        if s < 0.0 || s > 1.0 {
            return Err(FieldError::new("Saturation must be between 0.0 and 1.0!", Value::Null))
        }
        if v < 0.0 || v > 1.0 {
            return Err(FieldError::new("Value must be between 0.0 and 1.0!", Value::Null))
        }

        let effect = LedEffect::Static {
            hsv: build_hsv(h, s, v),
        };

        return match ctx.tx.send(effect) {
            Ok(_) => Ok(0),
            Err(_) => Ok(1),
        };
    }

    #[graphql(description = "Increase/decrease [initial_v] to [peak] by [step]")]
    fn breathing(
        ctx: &Context,
        h: f64,
        s: f64,
        initial_v: f64,
        step: f64,
        peak: f64,
    ) -> FieldResult<i32> {
        if step > peak {
            return Err(FieldError::new("Step can't be higher than peak!", Value::Null))
        }
        if initial_v > peak {
            return Err(FieldError::new("Initial value can't be higher than peak!", Value::Null))
        }

        if h < 0.0 || h > 360.0 {
            return Err(FieldError::new("Hue must be between 0.0 and 360.0!", Value::Null))
        }
        if s < 0.0 || s > 1.0 {
            return Err(FieldError::new("Saturation must be between 0.0 and 1.0!", Value::Null))
        }
        if initial_v < 0.0 || initial_v > 1.0 {
            return Err(FieldError::new("Initial value must be between 0.0 and 1.0!", Value::Null))
        }
        if peak < 0.0 || peak > 1.0 {
            return Err(FieldError::new("Peak must be between 0.0 and 1.0!", Value::Null))
        }

        let effect = LedEffect::Breathing {
            initial_hsv: build_hsv(h, s, initial_v),
            step: step as f32,
            peak: peak as f32,
            inhaling: initial_v > peak,
        };

        return match ctx.tx.send(effect) {
            Ok(_) => Ok(0),
            Err(_) => Ok(1),
        };
    }

    #[graphql(description = "Cycle hue by [step]")]
    fn rainbow(ctx: &Context, s: f64, v: f64, step: f64) -> FieldResult<i32> {
        if step > 360.0 {
            return Err(FieldError::new("Step can't be higher than max hue (360)!", Value::Null))
        }
        if s < 0.0 || s > 1.0 {
            return Err(FieldError::new("Saturation must be between 0.0 and 1.0!", Value::Null))
        }
        if v < 0.0 || v > 1.0 {
            return Err(FieldError::new("Value must be between 0.0 and 1.0!", Value::Null))
        }

        let effect = LedEffect::Rainbow {
            saturation: s as f32,
            value: v as f32,
            step: step as f32,
        };

        return match ctx.tx.send(effect) {
            Ok(_) => Ok(0),
            Err(_) => Ok(1),
        };
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
