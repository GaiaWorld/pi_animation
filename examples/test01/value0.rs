use std::ops::Add;

use pi_curves::curve::frame::{FrameValueScale, KeyFrameCurveValue, KeyFrameDataType, FrameDataValue};


#[derive(Debug, Clone, Copy)]
pub struct Value0(f32);
impl Add for Value0 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl FrameValueScale for Value0 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        Self(self.0 * rhs as f32)
    }
}
