use std::ops::Add;

use pi_curves::curve::frame::{FrameValueScale, KeyFrameCurveValue, KeyFrameDataType, FrameDataValue};

#[derive(Debug, Clone, Copy)]
pub struct Value1(u32, u32);
impl Add for Value1 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl FrameValueScale for Value1 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        Self(
            (self.0 as KeyFrameCurveValue * rhs) as u32,
            (self.1 as KeyFrameCurveValue * rhs) as u32
        )
    }
}