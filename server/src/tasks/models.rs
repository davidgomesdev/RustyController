use core::fmt;
use std::string::String;
use std::vec::Vec;

use crate::ps_move::models::{LedEffect, RumbleEffect};

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
            EffectChangeType::Led { effect } => { write!(f, "Led::{}", effect) }
            EffectChangeType::Rumble { effect } => { write!(f, "Rumble::{}", effect) }
        }
    }
}

#[derive(Clone)]
pub struct EffectChange {
    pub target: EffectTarget,
    pub effect: EffectChangeType,
}
