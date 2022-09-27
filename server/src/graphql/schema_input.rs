use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub(super) struct StaticEffectInput {
    #[graphql(description = "Hue/color (min 0.0, max 360.0)")]
    pub hue: f64,
    #[graphql(description = "Saturation (min 0.0, max 1.0)")]
    pub saturation: f64,
    #[graphql(description = "Value/brightness (min 0.0, max 1.0)")]
    pub value: f64,
}

#[derive(GraphQLInputObject)]
pub(super) struct BreathingEffectInput {
    #[graphql(description = "Hue/color (min 0.0, max 360.0)")]
    pub hue: f64,
    #[graphql(description = "Saturation (min 0.0, max 1.0)")]
    pub saturation: f64,
    #[graphql(description = "Initial `value` that increases to `peak` by `step`. (min 0.0, max `peak`)")]
    pub initial_value: f64,
    #[graphql(description = "Percentage that `value` changes per update, relative to `peak`. (min 0.0, max 1.0)")]
    pub step: f64,
    #[graphql(description = "Defines the max value that the controller breathes to. (min 0.0, max 1.0)")]
    pub peak: f64,
}

#[derive(GraphQLInputObject)]
pub(super) struct RainbowEffectInput {
    #[graphql(description = "Saturation (min 0.0, max 1.0)")]
    pub saturation: f64,
    #[graphql(description = "Value/brightness (min 0.0, max 1.0)")]
    pub value: f64,
    #[graphql(description = "Percentage that `hue` increases per update. (min 0.0, max 1.0)")]
    pub step: f64,
}
