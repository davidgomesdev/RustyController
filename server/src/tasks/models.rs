use core::fmt;
use std::string::String;
use std::vec::Vec;

use juniper::{GraphQLEnum, GraphQLObject};
use strum_macros::Display;

use crate::ps_move::effects::{LedEffect, RumbleEffect};
use crate::ps_move::models::ButtonState;

#[derive(Clone)]
pub enum EffectTarget {
    All,
    Only { bt_addresses: Vec<String> },
}

#[derive(Clone, Copy)]
pub enum EffectChangeType {
    RevertLed,
    Led { effect: LedEffect },
    Rumble { effect: RumbleEffect },
}

impl fmt::Display for EffectChangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EffectChangeType::RevertLed => { write!(f, "RevertLed") }
            EffectChangeType::Led { effect } => { write!(f, "Led::{}", &effect.details) }
            EffectChangeType::Rumble { effect } => { write!(f, "Rumble::{effect}") }
        }
    }
}

#[derive(Clone)]
pub struct EffectChange {
    pub target: EffectTarget,
    pub effect: EffectChangeType,
}

#[derive(GraphQLEnum, Eq, PartialEq, Hash, Copy, Clone, Debug, Display)]
pub enum Button {
    Cross,
    Square,
    Circle,
    Triangle,
    Move,
    Start,
    Select,
    Trigger,
}

#[derive(GraphQLObject, PartialEq, Copy, Clone, Debug)]
pub struct ButtonChange {
    button: Button,
    state: ButtonState,
}

#[derive(Copy, Clone, Display, Debug)]
pub enum ControllerChange {
    ButtonChange(ButtonChange),
}

impl ControllerChange {
    pub fn from_button(btn: &Button, state: &ButtonState) -> ControllerChange {
        ControllerChange::ButtonChange(ButtonChange {
            button: *btn,
            state: *state,
        })
    }
}
