use pi_curves::curve::frame::FrameDataValue;

use crate::{target_modifier::{IDAnimatableTarget, IDAnimatableAttr}, animation_context::AnimeResult, error::EAnimationError};

/// 对应动画数据类型的动画结果池
pub trait TypeAnimationResultPool<T: FrameDataValue> {
    fn record_target(
        &mut self,
        id_target: IDAnimatableTarget,
    );
    fn record_result(
        &mut self,
        id_target: IDAnimatableTarget,
        id_attr: IDAnimatableAttr,
        result: AnimeResult<T>,
    ) -> Result<(), EAnimationError>;
}

/// 实现一个二维数组保存的动画数据类型的动画结果池
pub struct TypeAnimationResultPoolDefault<T: FrameDataValue> {
    result: Vec<Vec<AnimeResult<T>>>,
}

impl<T: FrameDataValue> TypeAnimationResultPoolDefault<T>  {
    pub fn reset(
        &mut self,
    ) {
        self.result.iter_mut().for_each(|x| x.clear());
    }
    pub fn query_result(
        &mut self,
        target: IDAnimatableTarget,
    ) -> Vec<AnimeResult<T>> {
        self.result.get_mut(target).unwrap().splice(.., []).collect()
    }
}

impl<T: FrameDataValue> Default for TypeAnimationResultPoolDefault<T> {
    fn default() -> Self {
        Self { result: vec![] }
    }
}

impl<T: FrameDataValue> TypeAnimationResultPool<T> for TypeAnimationResultPoolDefault<T> {
    fn record_target(
        &mut self,
        id_target: IDAnimatableTarget,
    ) {
        let len = self.result.len();
        let id = id_target as usize + 1;
        if len <= id {
            for _ in len..id {
                self.result.push(vec![]);
            }
        }
    }
    fn record_result(
        &mut self,
        id_target: IDAnimatableTarget,
        _: IDAnimatableAttr,
        result: AnimeResult<T>,
    ) -> Result<(), EAnimationError> {
        match self.result.get_mut(id_target) {
            Some(results) => {
                results.push(result);
                Ok(())
            },
            None => {
                Err(EAnimationError::TargetIDNotRecordForTypeAnimationContext)
            },
        }
    }
}