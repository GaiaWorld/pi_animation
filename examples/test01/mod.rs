pub mod value0;
pub mod value1;


#[cfg(test)]
mod test {
    use std::ops::Add;

    use pi_animation::{target_modifier::{TAnimatableTargetModifier, IDAnimatableAttr}, error::EAnimationError};
    use pi_curves::curve::frame::{FrameValueScale, FrameDataValue, KeyFrameDataType, KeyFrameCurveValue};


    pub struct TestData {
        v0: Value0,
        v1: Value1
    }

    impl TAnimatableTargetModifier<Value0> for TestData {
        fn anime_modify(&mut self, attr: IDAnimatableAttr, value: Value0) -> Result<(), EAnimationError> {
            self.v0 = value;
            Ok(())
        }
    }

    impl TAnimatableTargetModifier<Value1> for TestData {
        fn anime_modify(&mut self, attr: IDAnimatableAttr, value: Value1) -> Result<(), EAnimationError> {
            self.v1 = value;
            Ok(())
        }
    }
}