use pi_curves::curve::{frame::KeyFrameCurveValue};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ELoopMode {
    /// 不循环
    Not,
    /// 正向循环
    Positive(Option<u16>),
    /// 反向循环
    Opposite(Option<u16>),
    /// 正向反复循环
    PositivePly(Option<u16>),
    /// 反向反复循环
    OppositePly(Option<u16>),
}

impl Default for ELoopMode {
    fn default() -> Self {
        ELoopMode::Not
    }
}

pub fn get_amount_calc(mode: ELoopMode) -> fn(KeyFrameCurveValue, KeyFrameCurveValue) -> (KeyFrameCurveValue, u16) {
    match mode {
        ELoopMode::Not => amount_not,
        ELoopMode::Positive(_) => amount_positive,
        ELoopMode::Opposite(_) => amount_opposite,
        ELoopMode::PositivePly(_) => amount_positive_ply,
        ELoopMode::OppositePly(_) => amount_opposite_ply,
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
    let loop_count = (delay_ms / once_time).floor() as i32;
    let result_count = loop_count / 2;

    let amount = if loop_count != result_count * 2  {
        1.0 - (delay_ms as KeyFrameCurveValue - loop_count as KeyFrameCurveValue * once_time) / once_time as KeyFrameCurveValue
    } else {
        (delay_ms as KeyFrameCurveValue - loop_count as KeyFrameCurveValue * once_time) / once_time as KeyFrameCurveValue
    };

    (amount, result_count as u16)
}

fn amount_opposite_ply(once_time: KeyFrameCurveValue, delay_ms: KeyFrameCurveValue) -> (KeyFrameCurveValue, u16) {
    let loop_count = (delay_ms / once_time).floor() as i32;
    let result_count = loop_count / 2;

    let amount = if loop_count != result_count * 2 {
        (delay_ms - loop_count as KeyFrameCurveValue * once_time) / once_time
    } else {
        1.0 - (delay_ms - loop_count as KeyFrameCurveValue * once_time) / once_time
    };

    (amount, result_count as u16)
}