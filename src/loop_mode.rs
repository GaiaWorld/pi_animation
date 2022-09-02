use pi_curves::curve::{frame::KeyFrameCurveValue};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ELoopMode {
    /// 不循环
    Not             = 1,
    /// 正向循环
    Positive        = 2,
    /// 反向循环
    Opposite        = 3,
    /// 正向反复循环
    PositivePly     = 4,
    /// 反向反复循环
    OppositePly     = 5
}

impl Default for ELoopMode {
    fn default() -> Self {
        ELoopMode::Not
    }
}

pub fn get_amount_calc(mode: ELoopMode) -> fn(KeyFrameCurveValue, KeyFrameCurveValue) -> (KeyFrameCurveValue, u16) {
    match mode {
        ELoopMode::Not => amount_not,
        ELoopMode::Positive => amount_positive,
        ELoopMode::Opposite => amount_opposite,
        ELoopMode::PositivePly => amount_positive_ply,
        ELoopMode::OppositePly => amount_opposite_ply,
    }
}

fn amount_not(once_time: KeyFrameCurveValue, delay_ms: KeyFrameCurveValue) -> (KeyFrameCurveValue, u16) {
    let loop_count = (delay_ms / once_time).floor();
    let delay_ms = KeyFrameCurveValue::max(0., KeyFrameCurveValue::min(once_time, delay_ms));
    let amount = (delay_ms as f32 / once_time as f32) as KeyFrameCurveValue;

    (amount, loop_count as u16)
}

fn amount_positive(once_time: KeyFrameCurveValue, delay_ms: KeyFrameCurveValue) -> (KeyFrameCurveValue, u16) {
    let loop_count = (delay_ms / once_time).floor();

    let amount = ((delay_ms as f32 - loop_count * once_time) / once_time as f32) as KeyFrameCurveValue;

    (amount, loop_count as u16)
}

fn amount_opposite(once_time: KeyFrameCurveValue, delay_ms: KeyFrameCurveValue) -> (KeyFrameCurveValue, u16) {
    let loop_count = (delay_ms / once_time).floor();

    let amount = (1.0 - (delay_ms as f32 - loop_count * once_time) / once_time as f32) as KeyFrameCurveValue;

    (amount, loop_count as u16)
}

fn amount_positive_ply(once_time: KeyFrameCurveValue, delay_ms: KeyFrameCurveValue) -> (KeyFrameCurveValue, u16) {
    let loop_count = (delay_ms / once_time).floor();

    let amount = if loop_count <= 1. {
        ((delay_ms as f32 - loop_count * once_time) / once_time as f32) as KeyFrameCurveValue
    } else {
        (1.0 - (delay_ms as f32 - loop_count * once_time) / once_time as f32) as KeyFrameCurveValue
    };

    (amount, (loop_count / 2.) as u16)
}

fn amount_opposite_ply(once_time: KeyFrameCurveValue, delay_ms: KeyFrameCurveValue) -> (KeyFrameCurveValue, u16) {
    let loop_count = (delay_ms / once_time).floor();

    let amount = if loop_count < 1. {
        ((delay_ms as f32 - loop_count * once_time) / once_time as f32) as KeyFrameCurveValue
    } else {
        (1.0 - (delay_ms as f32 - loop_count * once_time) / once_time as f32) as KeyFrameCurveValue
    };

    (amount, (loop_count / 2.) as u16)
}