use juniper::FieldResult;

use crate::graphql::schema::Context;
use crate::graphql::schema_response::{Controller, HealthStatus};

mod api {
    pub use crate::ps_move::models::*;
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
                battery_level: ctl.battery.clone(),
                connection_type: ctl.connection_type,
                current_led_effect: match ctl.led_effect {
                    api::LedEffect::Off => { graphql::LedEffect::Off }
                    api::LedEffect::Static { .. } => { graphql::LedEffect::Static }
                    api::LedEffect::Breathing { .. } => { graphql::LedEffect::Breathing }
                    api::LedEffect::Rainbow { .. } => { graphql::LedEffect::Rainbow }
                    api::LedEffect::Blink { .. } => { graphql::LedEffect::Blink }
                },
                current_rumble_effect: match ctl.rumble_effect {
                    api::RumbleEffect::Off => { graphql::RumbleEffect::Off }
                    api::RumbleEffect::Static { .. } => { graphql::RumbleEffect::Static }
                    api::RumbleEffect::Breathing { .. } => { graphql::RumbleEffect::Breathing }
                },
            }
        })
            .collect())
    }
}
