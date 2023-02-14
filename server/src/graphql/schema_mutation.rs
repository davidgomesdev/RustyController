use std::time::Duration;

use juniper::{FieldError, FieldResult, Value};
use log::{debug, info};
use palette::Hsv;
use tokio::time::Instant;

use crate::{EffectChange, EffectChangeType, EffectTarget, LedEffectDetails};
use crate::graphql::schema::Context;
use crate::graphql::schema_input::*;
use crate::graphql::schema_response::MutationResponse;
use crate::ps_move::api::build_hsv;
use crate::ps_move::effects::{LedEffect, RumbleEffect, RumbleEffectDetails};

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Turn the led off.")]
    fn set_led_off(ctx: &Context, input: Option<OffEffectInput>) -> FieldResult<MutationResponse> {
        info!("Received led off effect");
        debug!("Effect input: {input:?}");

        let controllers = input.map(|input| input.controllers);
        process_led_effect_mutation(ctx, LedEffect::off(), controllers)
    }

    #[graphql(description = "Set a constant color.")]
    fn set_led_static(ctx: &Context, input: StaticLedEffectInput) -> FieldResult<MutationResponse> {
        info!(
            "Received led static effect ({})",
            input
                .label
                .clone()
                .map_or(String::from("unlabeled"), |label| format!("'{label}'"))
        );
        debug!("Effect input: {input:?}");

        if input.label.map_or(false, |label| label.is_empty()) {
            return Err(FieldError::new("Label can't be empty!", Value::Null));
        }

        let ColorInput {
            hue,
            saturation,
            value,
        } = input.color;

        if !(0..=360).contains(&hue) {
            return Err(FieldError::new(
                "Hue must be between 0 and 360!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&saturation) {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(FieldError::new(
                "Value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        let effect = LedEffectDetails::Static {
            hsv: build_hsv(hue as f64, saturation, value),
        };

        process_led_effect_mutation(
            ctx,
            LedEffect::from(effect, input.duration),
            input.controllers,
        )
    }

    #[graphql(
        description = "Increase brightness of a color over time, reaching a peak, then reverting."
    )]
    fn set_led_breathing(
        ctx: &Context,
        input: BreathingLedEffectInput,
    ) -> FieldResult<MutationResponse> {
        info!(
            "Received led breathing effect ({})",
            input
                .label
                .clone()
                .map_or(String::from("unlabeled"), |label| format!("'{label}'"))
        );
        debug!("Effect input: {input:?}");

        if input.label.map_or(false, |label| label.is_empty()) {
            return Err(FieldError::new("Label can't be empty!", Value::Null));
        }

        if input.time_to_peak < 0 {
            return Err(FieldError::new("Step must be positive!", Value::Null));
        }

        if input.initial_value > input.peak {
            return Err(FieldError::new(
                "Initial value can't be higher than peak!",
                Value::Null,
            ));
        }

        if !(0..=360).contains(&input.hue) {
            return Err(FieldError::new(
                "Hue must be between 0.0 and 360.0!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.saturation) {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.initial_value) {
            return Err(FieldError::new(
                "Initial value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        if !(0.0..=1.0).contains(&input.peak) {
            return Err(FieldError::new(
                "Peak must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let effect = LedEffectDetails::new_timed_breathing(
            build_hsv(input.hue as f64, input.saturation, input.initial_value),
            Duration::from_millis(input.time_to_peak as u64),
            input.peak as f32,
        );

        process_led_effect_mutation(
            ctx,
            LedEffect::from(effect, input.duration),
            input.controllers,
        )
    }

    #[graphql(description = "Cycle through colors.")]
    fn set_led_rainbow(
        ctx: &Context,
        input: RainbowLedEffectInput,
    ) -> FieldResult<MutationResponse> {
        info!(
            "Received led rainbow effect ({})",
            input
                .label
                .clone()
                .map_or(String::from("unlabeled"), |label| format!("'{label}'"))
        );
        debug!("Effect input: {input:?}");

        if input.label.map_or(false, |label| label.is_empty()) {
            return Err(FieldError::new("Label can't be empty!", Value::Null));
        }

        if input.time_to_complete < 0.0 {
            return Err(FieldError::new("Step must be positive!", Value::Null));
        }

        if !(0.0..=1.0).contains(&input.saturation) {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.value) {
            return Err(FieldError::new(
                "Value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        let effect = LedEffectDetails::new_timed_rainbow(
            input.saturation as f32,
            input.value as f32,
            Duration::from_secs_f64(input.time_to_complete),
        );

        process_led_effect_mutation(
            ctx,
            LedEffect::from(effect, input.duration),
            input.controllers,
        )
    }

    #[graphql(description = "Alternate between color and off.")]
    fn set_led_blink(ctx: &Context, input: BlinkLedEffectInput) -> FieldResult<MutationResponse> {
        info!(
            "Received led blink effect ({})",
            input
                .label
                .clone()
                .map_or(String::from("unlabeled"), |label| format!("'{label}'"))
        );
        debug!("Effect input: {input:?}");

        if input.label.map_or(false, |label| label.is_empty()) {
            return Err(FieldError::new("Label can't be empty!", Value::Null));
        }

        let ColorInput {
            hue,
            saturation,
            value,
        } = input.color;

        if !(0..=360).contains(&hue) {
            return Err(FieldError::new(
                "Hue must be between 0 and 360!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&saturation) {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if value <= 0.0 || value > 1.0 {
            return Err(FieldError::new(
                "Value must be above 0.0 and equal or below 1.0!",
                Value::Null,
            ));
        }

        if input.interval < 0 {
            return Err(FieldError::new("Interval must be positive!", Value::Null));
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        let effect = LedEffectDetails::Blink {
            hsv: build_hsv(hue as f64, saturation, value),
            interval: Duration::from_millis(input.interval as u64),
            last_blink: Instant::now(),
        };

        process_led_effect_mutation(
            ctx,
            LedEffect::from(effect, input.duration),
            input.controllers,
        )
    }

    #[graphql(description = "Shift between colors.")]
    fn set_led_shift(ctx: &Context, input: ShiftLedEffectInput) -> FieldResult<MutationResponse> {
        info!(
            "Received led shift effect ({})",
            input
                .label
                .clone()
                .map_or(String::from("unlabeled"), |label| format!("'{label}'"))
        );
        debug!("Effect input: {input:?}");

        if input.label.map_or(false, |label| label.is_empty()) {
            return Err(FieldError::new("Name can't be empty!", Value::Null));
        }

        if input.colors.len() < 2 {
            return Err(FieldError::new("You must provide at least 2 colors. (Did you mean breathing or static?)", Value::Null));
        }

        if input.colors.len() > 7 {
            return Err(FieldError::new("Only 7 colors are allowed.", Value::Null));
        }

        for color in &input.colors {
            let ColorInput {
                hue,
                saturation,
                value,
            } = *color;

            if !(0..=360).contains(&hue) {
                return Err(FieldError::new(
                    "Hue must be between 0 and 360!",
                    Value::Null,
                ));
            }

            if !(0.0..=1.0).contains(&saturation) {
                return Err(FieldError::new(
                    "Saturation must be between 0.0 and 1.0!",
                    Value::Null,
                ));
            }

            if value <= 0.0 || value > 1.0 {
                return Err(FieldError::new(
                    "Value must be above 0.0 and equal or below 1.0!",
                    Value::Null,
                ));
            }
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        let hsv_list = input
            .colors
            .iter()
            .map(|color| {
                Hsv::new(
                    color.hue as f32,
                    color.saturation as f32,
                    color.value as f32,
                )
            })
            .collect();

        let effect = LedEffectDetails::new_shift(hsv_list);

        process_led_effect_mutation(
            ctx,
            LedEffect::from(effect, input.duration),
            input.controllers,
        )
    }

    #[graphql(description = "Turn rumble off.")]
    fn set_rumble_off(
        ctx: &Context,
        input: Option<OffEffectInput>,
    ) -> FieldResult<MutationResponse> {
        debug!("Received rumble off effect (with {input:?})");

        let controllers = input.map(|input| input.controllers);
        process_rumble_effect_mutation(ctx, RumbleEffect::off(), controllers)
    }

    #[graphql(description = "Set a constant rumble.")]
    fn set_rumble_static(
        ctx: &Context,
        input: StaticRumbleEffectInput,
    ) -> FieldResult<MutationResponse> {
        debug!("Received rumble static effect (with {input:?})");

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        if !(0.0..=1.0).contains(&input.strength) {
            return Err(FieldError::new(
                "Strength must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let details = RumbleEffectDetails::Static {
            strength: input.strength as f32,
        };

        process_rumble_effect_mutation(
            ctx,
            RumbleEffect::from(details, input.duration),
            input.controllers,
        )
    }

    #[graphql(description = "Increase rumble strength over time, reaching a peak, then reverting.")]
    fn set_rumble_breathing(
        ctx: &Context,
        input: BreathingRumbleEffectInput,
    ) -> FieldResult<MutationResponse> {
        debug!("Received rumble breathing effect (with {input:?})");

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        if !(0.0..=1.0).contains(&input.step) {
            return Err(FieldError::new(
                "Step must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.initial_strength > input.peak {
            return Err(FieldError::new(
                "Initial strength can't be higher than peak!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.initial_strength) {
            return Err(FieldError::new(
                "Initial strength must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.peak) {
            return Err(FieldError::new(
                "Peak must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let details = RumbleEffectDetails::Breathing {
            initial_strength: input.initial_strength as f32,
            step: input.step as f32,
            peak: input.peak as f32,
            inhaling: true,
        };

        process_rumble_effect_mutation(
            ctx,
            RumbleEffect::from(details, input.duration),
            input.controllers,
        )
    }

    #[graphql(description = "Alternate between rumble on and off.")]
    fn set_rumble_blink(
        ctx: &Context,
        input: BlinkRumbleEffectInput,
    ) -> FieldResult<MutationResponse> {
        info!("Received rumble blink effect");
        debug!("Effect input: {input:?}");

        if !(0.0..=1.0).contains(&input.strength) {
            return Err(FieldError::new(
                "Strength must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.interval < 0 {
            return Err(FieldError::new("Interval must be positive!", Value::Null));
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        let details = RumbleEffectDetails::Blink {
            strength: input.strength as f32,
            interval: Duration::from_millis(input.interval as u64),
            last_blink: Instant::now(),
        };

        process_rumble_effect_mutation(
            ctx,
            RumbleEffect::from(details, input.duration),
            input.controllers,
        )
    }
}

fn process_led_effect_mutation(
    ctx: &Context,
    effect: LedEffect,
    target: Option<Vec<String>>,
) -> FieldResult<MutationResponse> {
    process_effect_mutation(ctx, EffectChangeType::Led { effect }, target)
}

fn process_rumble_effect_mutation(
    ctx: &Context,
    effect: RumbleEffect,
    target: Option<Vec<String>>,
) -> FieldResult<MutationResponse> {
    process_effect_mutation(ctx, EffectChangeType::Rumble { effect }, target)
}

fn process_effect_mutation(
    ctx: &Context,
    effect: EffectChangeType,
    target: Option<Vec<String>>,
) -> FieldResult<MutationResponse> {
    let target = match target {
        None => EffectTarget::All,
        Some(bt_addresses) => {
            if bt_addresses.is_empty() {
                return Err(FieldError::new(
                    "You must specify controllers!",
                    Value::Null,
                ));
            } else {
                EffectTarget::Only { bt_addresses }
            }
        }
    };

    match ctx.effect_tx.send(EffectChange { effect, target }) {
        Ok(_) => Ok(MutationResponse::Success),
        Err(_) => Ok(MutationResponse::ServerError),
    }
}
