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
    pub(super) current_led_effect: LedEffectType,
    pub(super) current_rumble_effect: RumbleEffectType,
}

#[derive(GraphQLEnum)]
pub(super) enum LedEffectType {
    Off,
    Static,
    Breathing,
    Rainbow,
    Blink,
}

#[derive(GraphQLEnum)]
pub(super) enum RumbleEffectType {
    Off,
    Static,
    Breathing,
    Blink
}
