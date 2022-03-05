use std::sync::Arc;

use juniper::{EmptySubscription, RootNode};
use juniper::{FieldResult, GraphQLEnum, GraphQLInputObject};
use palette::encoding::Srgb;
use palette::Hsv;
use tokio::sync::watch::Sender;

use crate::ps_move_api::LedEffect;

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
    fn health(ctx: &Context) -> FieldResult<HealthStatus> {
        Ok(HealthStatus::Ok)
    }
}

#[derive(GraphQLEnum)]
enum EffectType {
    Static,
    Rainbow,
}

#[derive(GraphQLInputObject)]
struct LedSetting {
    effectType: EffectType,
    hue: f64,
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Set a LED effect and hue")]
    fn set_led(ctx: &Context, _new_led: LedSetting) -> FieldResult<i32> {
        return match ctx.tx.send(LedEffect::Static {
            hsv: Hsv::<Srgb, f32>::from_components((120.0, 0.4, 0.34)),
        }) {
            Ok(_) => Ok(0),
            Err(_) => Ok(1),
        };
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
