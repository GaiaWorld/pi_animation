use pi_curves::curve::frame::{KeyFrameCurveValue, KeyFrameDataType};

use crate::{
    error::EAnimationError,
    frame_curve_manager::FrameCurveInfoID,
    target_modifier::{IDAnimatableAttr, IDAnimatableTarget},
};

/// 一个动画的运行时数据
#[derive(Debug, Clone, Copy)]
pub struct RuntimeInfo<T: Clone> {
    // pub group_info: AnimationGroupRuntimeInfo,
    /// 所属动画组的权重
    pub group_weight: f32,
    /// 动画进度
    pub amount_in_second: KeyFrameCurveValue,
    /// 作用的 目标对象 的ID
    pub target: T,
    /// 作用的 目标对象 的目标属性 的ID
    pub attr: IDAnimatableAttr,
    /// 在曲线对应的数据类型 曲线信息管理器中 该动画使用的曲线 的 ID
    pub curve_id: FrameCurveInfoID,
    // pub anime: TargetAnimation,
}

/// 运行时信息表 - 唯一
/// 每个 Vec<RuntimeInfo> 分别对应一个 动画数据类型
pub struct RuntimeInfoMap<T: Clone> {
    pub list: Vec<Vec<RuntimeInfo<T>>>,
}

impl<T: Clone> RuntimeInfoMap<T> {
    pub fn default() -> Self {
        Self { list: vec![] }
    }
    /// 仅在分配 KeyAnimeDataType 后立即调用
    pub fn add_type(&mut self, ty: KeyFrameDataType) {
        if ty >= self.list.len() {
            for _ in self.list.len()..ty + 1 {
                self.list.push(vec![]);
            }
        }
    }
    /// 动画运行时记录
    pub fn insert(
        &mut self,
        ty: KeyFrameDataType,
        info: RuntimeInfo<T>,
    ) -> Result<(), EAnimationError> {
        match self.list.get_mut(ty) {
            Some(list) => {
                list.push(info);
                Ok(())
            }
            None => Err(EAnimationError::RuntimeInfoMapNotFindType),
        }
    }
    pub fn reset(&mut self) {
        self.list.iter_mut().for_each(|x| x.clear());
    }
}
