use juniper::FieldResult;

use crate::graphql::schema::Context;
use crate::graphql::schema_response::{Controller, HealthStatus};

mod api {
    pub use crate::ps_move::effects::*;
}

mod graphql {
    pub use crate::graphql::schema_response::*;
}


pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Check the service health")]
    fn health(_ctx: &Context) -> FieldResult<HealthStatus> {
        Ok(HealthStatus::Ok)
    }

    #[graphql(description = "Lists all connected controllers")]
    async fn controllers(_ctx: &Context) -> FieldResult<Vec<Controller>> {
        let controllers = _ctx.controllers.lock().await;

        Ok(controllers.iter().map(|ctl| {
            Controller {
                address: ctl.bt_address.clone(),
                battery_level: ctl.battery,
                connection_type: ctl.connection_type,
                current_led_effect: match ctl.led_effect.kind {
                    api::LedEffectKind::Off => { graphql::LedEffectType::Off }
                    api::LedEffectKind::Static { .. } => { graphql::LedEffectType::Static }
                    api::LedEffectKind::Breathing { .. } => { graphql::LedEffectType::Breathing }
                    api::LedEffectKind::Rainbow { .. } => { graphql::LedEffectType::Rainbow }
                    api::LedEffectKind::Blink { .. } => { graphql::LedEffectType::Blink }
                    api::LedEffectKind::Candle { .. } => { graphql::LedEffectType::Candle }
                },
                current_rumble_effect: match ctl.rumble_effect.kind {
                    api::RumbleEffectKind::Off => { graphql::RumbleEffectType::Off }
                    api::RumbleEffectKind::Static { .. } => { graphql::RumbleEffectType::Static }
                    api::RumbleEffectKind::Breathing { .. } => { graphql::RumbleEffectType::Breathing }
                    api::RumbleEffectKind::Blink { .. } => { graphql::RumbleEffectType::Blink }
                },
            }
        })
            .collect())
    }
}
