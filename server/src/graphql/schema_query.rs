use juniper::FieldResult;

use crate::graphql::schema::Context;
use crate::graphql::schema_response::{Controller, HealthStatus};
use crate::ps_move::models::LedEffect as ApiLedEffect;

use super::schema_response::LedEffect;

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
                battery_level: ctl.battery.clone(),
                connection_type: ctl.connection_type,
                current_effect: match ctl.led_effect {
                    ApiLedEffect::Off => { LedEffect::Off }
                    ApiLedEffect::Static { .. } => { LedEffect::Static }
                    ApiLedEffect::Breathing { .. } => { LedEffect::Breathing }
                    ApiLedEffect::Rainbow { .. } => { LedEffect::Rainbow }
                },
            }
        })
            .collect())
    }
}
