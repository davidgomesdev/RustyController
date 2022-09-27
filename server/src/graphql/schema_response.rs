use juniper::{GraphQLEnum, GraphQLObject};

use crate::ps_move::models::{BatteryLevel, ConnectionType};

#[derive(GraphQLEnum)]
pub enum HealthStatus {
    Ok,
    Error,
}

#[derive(GraphQLEnum)]
pub enum MutationResponse {
    Success,
    ServerError,
}

#[derive(GraphQLObject)]
pub struct Controller {
    pub address: String,
    pub battery_level: BatteryLevel,
    pub connection_type: ConnectionType,
    pub(super) current_effect: LedEffect,
}

#[derive(GraphQLEnum)]
pub(super) enum LedEffect {
    Off,
    Static,
    Breathing,
    Rainbow,
}
