use core::fmt;
use std::string::String;
use std::vec::Vec;

use juniper::{GraphQLEnum, GraphQLObject};

use crate::ps_move::effects::{LedEffect, RumbleEffect};

#[derive(Clone)]
pub enum EffectTarget {
    All,
    Only { bt_addresses: Vec<String> },
}

#[derive(Clone, Copy)]
pub enum EffectChangeType {
    Led { effect: LedEffect },
    Rumble { effect: RumbleEffect },
}

impl fmt::Display for EffectChangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EffectChangeType::Led { effect } => { write!(f, "Led::{}", &effect.details) }
            EffectChangeType::Rumble { effect } => { write!(f, "Rumble::{}", effect) }
        }
    }
}

#[derive(Clone)]
pub struct EffectChange {
    pub target: EffectTarget,
    pub effect: EffectChangeType,
}

#[derive(GraphQLEnum, Copy, Clone)]
pub enum Button {
    Cross,
    Square,
    Circle,
    Triangle,
    Move,
}

#[derive(Copy, Clone)]
pub enum ControllerChange {
    ButtonPressed(Button)
}
