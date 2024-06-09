use std::time::Duration;

use juniper::{FieldError, FieldResult, Value};
use tokio::time::Instant;

use crate::{EffectChange, EffectChangeType, EffectTarget, LedEffectKind};
use crate::graphql::schema::Context;
use crate::graphql::schema_input::*;
use crate::graphql::schema_response::MutationResponse;
use crate::ps_move::api::build_hsv;
use crate::ps_move::effects::{LedEffect, RumbleEffect, RumbleEffectKind};
use crate::tasks::models::EffectChangeType::RevertLed;

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Revert the led to the last effect. (if it's expired, it goes off)")]
    fn revert_led(
        ctx: &Context,
        input: Option<RevertEffectInput>,
    ) -> FieldResult<MutationResponse> {
        tracing::info!("Received led to last effect");
        tracing::debug!("Effect input: {input:?}");

        let controllers = input.map(|input| input.controllers);

        let target = match controller_to_effect_target(controllers) {
            Ok(value) => value,
            Err(value) => return value,
        };

        match ctx.effect_tx.send(EffectChange {
            effect: RevertLed,
            target,
        }) {
            Ok(_) => Ok(MutationResponse::Success),
            Err(_) => Ok(MutationResponse::ServerError),
        }
    }

    #[graphql(description = "Turn the led off.")]
    fn set_led_off(ctx: &Context, input: Option<OffEffectInput>) -> FieldResult<MutationResponse> {
        tracing::info!("Received led off effect");
        tracing::debug!("Effect input: {input:?}");

        let controllers = input.map(|input| input.controllers);
        process_led_effect_mutation(ctx, LedEffect::off(), controllers)
    }

    #[graphql(description = "Set a constant color.")]
    fn set_led_static(ctx: &Context, input: StaticLedEffectInput) -> FieldResult<MutationResponse> {
        tracing::info!(
            "Received led static effect ({})",
            input
                .name
                .clone()
                .map_or(String::from("unnamed"), |name| format!("'{name}'"))
        );
        tracing::debug!("Effect input: {input:?}");

        if input.name.map_or(false, |name| name.is_empty()) {
            return Err(FieldError::new("Name can't be empty!", Value::Null));
        }

        if input.hue < 0 || input.hue > 360 {
            return Err(FieldError::new(
                "Hue must be between 0 and 360!",
                Value::Null,
            ));
        }

        if input.saturation < 0.0 || input.saturation > 1.0 {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.value < 0.0 || input.value > 1.0 {
            return Err(FieldError::new(
                "Value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        let effect = LedEffectKind::Static {
            hsv: build_hsv(input.hue as f64, input.saturation, input.value),
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
        tracing::info!(
            "Received led breathing effect ({})",
            input
                .name
                .clone()
                .map_or(String::from("unnamed"), |name| format!("'{name}'"))
        );
        tracing::debug!("Effect input: {input:?}");

        if input.name.map_or(false, |name| name.is_empty()) {
            return Err(FieldError::new("Name can't be empty!", Value::Null));
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

        if input.hue < 0 || input.hue > 360 {
            return Err(FieldError::new(
                "Hue must be between 0.0 and 360.0!",
                Value::Null,
            ));
        }

        if input.saturation < 0.0 || input.saturation > 1.0 {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.initial_value < 0.0 || input.initial_value > 1.0 {
            return Err(FieldError::new(
                "Initial value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        if input.peak < 0.0 || input.peak > 1.0 {
            return Err(FieldError::new(
                "Peak must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let effect = LedEffectKind::new_timed_breathing(
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
        tracing::info!(
            "Received led rainbow effect ({})",
            input
                .name
                .clone()
                .map_or(String::from("unnamed"), |name| format!("'{name}'"))
        );
        tracing::debug!("Effect input: {input:?}");

        if input.name.map_or(false, |name| name.is_empty()) {
            return Err(FieldError::new("Name can't be empty!", Value::Null));
        }

        if input.time_to_complete < 0.0 {
            return Err(FieldError::new("Step must be positive!", Value::Null));
        }

        if input.saturation < 0.0 || input.saturation > 1.0 {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.value < 0.0 || input.value > 1.0 {
            return Err(FieldError::new(
                "Value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        let effect = LedEffectKind::new_timed_rainbow(
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
        tracing::info!(
            "Received led blink effect ({})",
            input
                .name
                .clone()
                .map_or(String::from("unnamed"), |name| format!("'{name}'"))
        );
        tracing::debug!("Effect input: {input:?}");

        if input.name.map_or(false, |name| name.is_empty()) {
            return Err(FieldError::new("Name can't be empty!", Value::Null));
        }

        if !(0..=360).contains(&input.hue) {
            return Err(FieldError::new(
                "Hue must be between 0 and 360!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.saturation) {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.value <= 0.0 || input.value > 1.0 {
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

        let effect = LedEffectKind::Blink {
            hsv: build_hsv(input.hue as f64, input.saturation, input.value),
            interval: Duration::from_millis(input.interval as u64),
            last_blink: Instant::now(),
        };

        process_led_effect_mutation(
            ctx,
            LedEffect::from(effect, input.duration),
            input.controllers,
        )
    }

    #[graphql(
        description = "Randomly set brightness between min value and max value, simulating a candle/flame."
    )]
    fn set_led_candle(ctx: &Context, input: CandleLedEffectInput) -> FieldResult<MutationResponse> {
        tracing::info!(
            "Received led candle effect ({})",
            input
                .name
                .clone()
                .map_or(String::from("unnamed"), |name| format!("'{name}'"))
        );
        tracing::debug!("Effect input: {input:?}");

        if input.name.map_or(false, |name| name.is_empty()) {
            return Err(FieldError::new("Name can't be empty!", Value::Null));
        }

        if !(0..=360).contains(&input.hue) {
            return Err(FieldError::new(
                "Hue must be between 0 and 360!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.saturation) {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.min_value) {
            return Err(FieldError::new(
                "Min value must between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.max_value) {
            return Err(FieldError::new(
                "Max value must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.variability) {
            return Err(FieldError::new(
                "Variability must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        let hue = input.hue as f32;
        let saturation = input.saturation as f32;
        let min_value = input.min_value as f32;
        let max_value = input.max_value as f32;
        let variability = input.variability as f32;

        let effect = LedEffectKind::new_candle(
            hue,
            saturation,
            min_value,
            max_value,
            variability,
            input.interval,
        );

        process_led_effect_mutation(
            ctx,
            LedEffect::from(effect, input.duration),
            input.controllers,
        )
    }

    #[graphql(
        description = "Bounces from one color to the other."
    )]
    fn set_led_bounce(ctx: &Context, input: BounceLedEffectInput) -> FieldResult<MutationResponse> {
        tracing::info!(
            "Received led bounce effect ({})",
            input
                .name
                .clone()
                .map_or(String::from("unnamed"), |name| format!("'{name}'"))
        );
        tracing::debug!("Effect input: {input:?}");

        if input.name.map_or(false, |name| name.is_empty()) {
            return Err(FieldError::new("Name can't be empty!", Value::Null));
        }

        if !input.hues.iter().all(|hue| (0..=360).contains(hue)) {
            return Err(FieldError::new(
                "Hue must be between 0 and 360!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.saturation) {
            return Err(FieldError::new(
                "Saturation must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if !(0.0..=1.0).contains(&input.value) {
            return Err(FieldError::new(
                "Min value must between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let effect = LedEffectKind::new_bounce(input.hues.iter().map(|hue| *hue as f32).collect(), input.saturation as f32, input.value as f32, input.step as f32);

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
        tracing::debug!("Received rumble off effect (with {input:?})");

        let controllers = input.map(|input| input.controllers);
        process_rumble_effect_mutation(ctx, RumbleEffect::off(), controllers)
    }

    #[graphql(description = "Set a constant rumble.")]
    fn set_rumble_static(
        ctx: &Context,
        input: StaticRumbleEffectInput,
    ) -> FieldResult<MutationResponse> {
        tracing::debug!("Received rumble static effect (with {input:?})");

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        if input.strength < 0.0 || input.strength > 1.0 {
            return Err(FieldError::new(
                "Strength must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let kind = RumbleEffectKind::Static {
            strength: input.strength as f32,
        };

        process_rumble_effect_mutation(
            ctx,
            RumbleEffect::from(kind, input.duration),
            input.controllers,
        )
    }

    #[graphql(description = "Increase rumble strength over time, reaching a peak, then reverting.")]
    fn set_rumble_breathing(
        ctx: &Context,
        input: BreathingRumbleEffectInput,
    ) -> FieldResult<MutationResponse> {
        tracing::debug!("Received rumble breathing effect (with {input:?})");

        if input.duration.filter(|duration| *duration < 0).is_some() {
            return Err(FieldError::new("Duration must be positive!", Value::Null));
        }

        if input.step < 0.0 || input.step > 1.0 {
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

        if input.initial_strength < 0.0 || input.initial_strength > 1.0 {
            return Err(FieldError::new(
                "Initial strength must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        if input.peak < 0.0 || input.peak > 1.0 {
            return Err(FieldError::new(
                "Peak must be between 0.0 and 1.0!",
                Value::Null,
            ));
        }

        let kind = RumbleEffectKind::Breathing {
            initial_strength: input.initial_strength as f32,
            step: input.step as f32,
            peak: input.peak as f32,
            inhaling: true,
        };

        process_rumble_effect_mutation(
            ctx,
            RumbleEffect::from(kind, input.duration),
            input.controllers,
        )
    }

    #[graphql(description = "Alternate between rumble on and off.")]
    fn set_rumble_blink(
        ctx: &Context,
        input: BlinkRumbleEffectInput,
    ) -> FieldResult<MutationResponse> {
        tracing::info!("Received rumble blink effect");
        tracing::debug!("Effect input: {input:?}");

        if input.strength < 0.0 || input.strength > 1.0 {
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

        let kind = RumbleEffectKind::Blink {
            strength: input.strength as f32,
            interval: Duration::from_millis(input.interval as u64),
            last_blink: Instant::now(),
        };

        process_rumble_effect_mutation(
            ctx,
            RumbleEffect::from(kind, input.duration),
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
    controllers: Option<Vec<String>>,
) -> FieldResult<MutationResponse> {
    let target = match controller_to_effect_target(controllers) {
        Ok(value) => value,
        Err(value) => return value,
    };

    match ctx.effect_tx.send(EffectChange { effect, target }) {
        Ok(_) => Ok(MutationResponse::Success),
        Err(_) => Ok(MutationResponse::ServerError),
    }
}

fn controller_to_effect_target(
    controllers: Option<Vec<String>>,
) -> Result<EffectTarget, FieldResult<MutationResponse>> {
    Ok(match controllers {
        None => EffectTarget::All,
        Some(bt_addresses) => {
            if bt_addresses.is_empty() {
                return Err(Err(FieldError::new(
                    "You must specify controllers!",
                    Value::Null,
                )));
            } else {
                EffectTarget::Only { bt_addresses }
            }
        }
    })
}
