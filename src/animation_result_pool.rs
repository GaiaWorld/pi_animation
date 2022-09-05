use pi_curves::curve::frame::FrameDataValue;
use pi_slotmap::{DefaultKey, SlotMap, SecondaryMap};

use crate::{target_modifier::{IDAnimatableTarget, IDAnimatableAttr}, animation_context::AnimeResult, error::EAnimationError};

/// 对应动画数据类型的动画结果池
pub trait TypeAnimationResultPool<F: FrameDataValue, T> {
    fn record_target(
        &mut self,
        id_target: T,
    );
    fn record_result(
        &mut self,
        id_target: T,
        id_attr: IDAnimatableAttr,
        result: AnimeResult<F>,
    ) -> Result<(), EAnimationError>;
}

/// 实现一个二维数组保存的动画数据类型的动画结果池
pub struct TypeAnimationResultPoolDefault<T: FrameDataValue> {
    result: SecondaryMap<DefaultKey, Vec<AnimeResult<T>>>,
}

impl<T: FrameDataValue> TypeAnimationResultPoolDefault<T>  {
    pub fn reset(
        &mut self,
    ) {
		// self.result.clear()
        self.result.iter_mut().for_each(|(k, x)| x.clear());
    }
    pub fn query_result(
        &mut self,
        target: DefaultKey,
    ) -> Vec<AnimeResult<T>> {
		// self.result.remove(target)
        self.result.get_mut(target).unwrap().splice(.., []).collect()
    }
}

impl<F: FrameDataValue> Default for TypeAnimationResultPoolDefault<F> {
    fn default() -> Self {
        Self { result: SecondaryMap::default() }
    }
}

impl<F: FrameDataValue> TypeAnimationResultPool<F, DefaultKey> for TypeAnimationResultPoolDefault<F> {
    fn record_target(
        &mut self,
        id_target: DefaultKey,
    ) {
		if let None = self.result.get(id_target) {
			self.result.insert(id_target, Vec::default());
		}
        // let len = self.result.len();
        // let id = id_target as usize + 1;
        // if len <= id {
        //     for _ in len..id {
        //         self.result.push(vec![]);
        //     }
        // }
    }
    fn record_result(
        &mut self,
        id_target: DefaultKey,
        _: IDAnimatableAttr,
        result: AnimeResult<F>,
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