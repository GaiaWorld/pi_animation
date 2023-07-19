
use pi_curves::{EAmountMode, AmountParam, steps::EStepMode, curve::{frame::KeyFrameCurveValue, FrameIndex}, easing::EEasingMode};

pub struct AnimationAmountCalc {
    mode: EAmountMode,
    param: AmountParam,
    call: fn(KeyFrameCurveValue, &AmountParam) -> KeyFrameCurveValue,
}

impl Default for AnimationAmountCalc {
    fn default() -> Self {
        Self {
            mode: EAmountMode::None,
            param: AmountParam::default(),
            call: EAmountMode::get_transform_amount_call(EAmountMode::None),
        }
    }
}

impl AnimationAmountCalc {
    pub fn mode(&self) -> EAmountMode {
        self.mode
    }
    pub fn from_steps(step: FrameIndex, mode: EStepMode) -> Self {
        if step <= 1 {
            AnimationAmountCalc::default()
        } else {
            let mode = EAmountMode::Steps(mode);
            Self {
                mode,
                param: AmountParam(step as KeyFrameCurveValue, 0., 0., 0.),
                call: EAmountMode::get_transform_amount_call(mode),
            }
        }
    }
    pub fn from_easing(mode: EEasingMode) -> Self {
        let mode = EAmountMode::Easing(mode);
        Self {
            mode,
            param: AmountParam::default(),
            call: EAmountMode::get_transform_amount_call(mode),
        }
    }
    pub fn from_cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        let mode = EAmountMode::CubicBezier;
        Self {
            mode,
            param: AmountParam(x1 as KeyFrameCurveValue, y1 as KeyFrameCurveValue, x2 as KeyFrameCurveValue, y2 as KeyFrameCurveValue),
            call: EAmountMode::get_transform_amount_call(mode),
        }
    }
    pub fn calc(&self, amount: KeyFrameCurveValue) -> KeyFrameCurveValue {
        let call = &self.call;
        call(amount, &self.param)
    }
}