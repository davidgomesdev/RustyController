use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject, Debug)]
pub(super) struct RevertEffectInput {
    #[graphql(
    description = "Applies the effect only on these controller addresses. (must not be empty)"
    )]
    pub controllers: Vec<String>,
}

#[derive(GraphQLInputObject, Debug)]
pub(super) struct OffEffectInput {
    #[graphql(
    description = "Applies the effect only on these controller addresses. (must not be empty)"
    )]
    pub controllers: Vec<String>,
}

#[derive(GraphQLInputObject, Debug)]
pub(super) struct StaticLedEffectInput {
    #[graphql(
    description = "If specified, must not be empty, and applies the effect only on these controller addresses."
    )]
    pub controllers: Option<Vec<String>>,
    #[graphql(description = "Duration of effect, in milliseconds, if specified.")]
    pub duration: Option<i32>,
    #[graphql(description = "Name of the effect.")]
    pub name: Option<String>,
    #[graphql(description = "Hue/color (min 0.0, max 360.0)")]
    pub hue: i32,
    #[graphql(description = "Saturation (min 0.0, max 1.0)")]
    pub saturation: f64,
    #[graphql(description = "Value/brightness (min 0.0, max 1.0)")]
    pub value: f64,
}

#[derive(GraphQLInputObject, Debug)]
pub(super) struct BreathingLedEffectInput {
    #[graphql(
    description = "If specified, must not be empty, and applies the effect only on these controller addresses."
    )]
    pub controllers: Option<Vec<String>>,
    #[graphql(description = "Duration of effect, in milliseconds, if specified.")]
    pub duration: Option<i32>,
    #[graphql(description = "Name of the effect.")]
    pub name: Option<String>,
    #[graphql(description = "Hue/color (min 0.0, max 360.0)")]
    pub hue: i32,
    #[graphql(description = "Saturation (min 0.0, max 1.0)")]
    pub saturation: f64,
    #[graphql(description = "Initial value/brightness. (min 0.0, max `peak`)")]
    pub initial_value: f64,
    #[graphql(
    description = "Time that value/brightness takes to reach `peak`, in milliseconds."
    )]
    pub time_to_peak: i32,
    #[graphql(
    description = "Defines the max value that the controller breathes to. (min 0.0, max 1.0)"
    )]
    pub peak: f64,
}

#[derive(GraphQLInputObject, Debug)]
pub(super) struct RainbowLedEffectInput {
    #[graphql(
    description = "If specified, must not be empty, and applies the effect only on these controller addresses."
    )]
    pub controllers: Option<Vec<String>>,
    #[graphql(description = "Duration of effect, in milliseconds, if specified.")]
    pub duration: Option<i32>,
    #[graphql(description = "Name of the effect.")]
    pub name: Option<String>,
    #[graphql(description = "Saturation (min 0.0, max 1.0)")]
    pub saturation: f64,
    #[graphql(description = "Value/brightness (min 0.0, max 1.0)")]
    pub value: f64,
    #[graphql(description = "Time that the controller takes to reach go through the full color spectrum, in seconds")]
    pub time_to_complete: f64,
}

#[derive(GraphQLInputObject, Debug)]
pub(super) struct BlinkLedEffectInput {
    #[graphql(
    description = "If specified, must not be empty, and applies the effect only on these controller addresses."
    )]
    pub controllers: Option<Vec<String>>,
    #[graphql(description = "Duration of effect, in milliseconds, if specified.")]
    pub duration: Option<i32>,
    #[graphql(description = "Name of the effect.")]
    pub name: Option<String>,
    #[graphql(description = "Hue/color (min 0.0, max 360.0)")]
    pub hue: i32,
    #[graphql(description = "Saturation (min 0.0, max 1.0)")]
    pub saturation: f64,
    #[graphql(description = "Value/brightness (min 0.0, max 1.0)")]
    pub value: f64,
    #[graphql(description = "Interval between blinks, in ms.")]
    pub interval: i32,
}

#[derive(GraphQLInputObject, Debug)]
pub(super) struct CandleLedEffectInput {
    #[graphql(
    description = "If specified, must not be empty, and applies the effect only on these controller addresses."
    )]
    pub controllers: Option<Vec<String>>,
    #[graphql(description = "Duration of effect, in milliseconds, if specified.")]
    pub duration: Option<i32>,
    #[graphql(description = "Name of the effect.")]
    pub name: Option<String>,
    #[graphql(description = "Hue/color (min 0.0, max 360.0)")]
    pub hue: i32,
    #[graphql(description = "Saturation (min 0.0, max 1.0)")]
    pub saturation: f64,
    #[graphql(description = "Min value (min 0.0, max 1.0)")]
    pub min_value: f64,
    #[graphql(description = "Max value (min 0.0, max 1.0)")]
    pub max_value: f64,
    #[graphql(description = "Percentage of variability of the range. (min 0.0, max 1.0)")]
    pub variability: f64,
    #[graphql(description = "Interval to change, in milliseconds. (1ms is the default)")]
    pub interval: Option<i32>,
}

#[derive(GraphQLInputObject, Debug)]
pub(super) struct StaticRumbleEffectInput {
    #[graphql(
    description = "If specified, must not be empty, and applies the effect only on these controller addresses."
    )]
    pub controllers: Option<Vec<String>>,
    #[graphql(description = "Duration of effect, in milliseconds, if specified.")]
    pub duration: Option<i32>,
    #[graphql(description = "Rumble strength (min 0.0, max 1.0)")]
    pub strength: f64,
}

#[derive(GraphQLInputObject, Debug)]
pub(super) struct BreathingRumbleEffectInput {
    #[graphql(
    description = "If specified, must not be empty, and applies the effect only on these controller addresses."
    )]
    pub controllers: Option<Vec<String>>,
    #[graphql(description = "Duration of effect, in milliseconds, if specified.")]
    pub duration: Option<i32>,
    #[graphql(description = "Initial rumble strength. (min 0.0, max `peak`)")]
    pub initial_strength: f64,
    #[graphql(
    description = "Percentage that the rumble strength changes per update, relative to `peak`. (min 0.0, max 1.0)"
    )]
    pub step: f64,
    #[graphql(
    description = "Defines the max rumble that the controller gets to. (min 0.0, max 1.0)"
    )]
    pub peak: f64,
}

#[derive(GraphQLInputObject, Debug)]
pub(super) struct BlinkRumbleEffectInput {
    #[graphql(
    description = "If specified, must not be empty, and applies the effect only on these controller addresses."
    )]
    pub controllers: Option<Vec<String>>,
    #[graphql(description = "Duration of effect, in milliseconds, if specified.")]
    pub duration: Option<i32>,
    #[graphql(description = "Rumble strength. (min 0.0, max 1.0)")]
    pub strength: f64,
    #[graphql(description = "Interval between blinks, in ms.")]
    pub interval: i32,
}
