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
    pub(super) current_led_effect: LedEffect,
    pub(super) current_rumble_effect: RumbleEffect,
}

#[derive(GraphQLEnum)]
pub(super) enum LedEffect {
    Off,
    Static,
    Breathing,
    Rainbow,
}

#[derive(GraphQLEnum)]
pub(super) enum RumbleEffect {
    Off,
    Static,
    Breathing,
}
