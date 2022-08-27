use pi_curves::curve::frame::{KeyFrameDataType, KeyFrameCurveValue};

use crate::{animation::AnimationInfo, target_modifier::{IDAnimatableTarget, IDAnimatableAttr}, error::EAnimationError, target_animation::TargetAnimation, frame_curve_manager::FrameCurveInfoID};

/// 一个动画的运行时数据
#[derive(Debug, Clone, Copy)]
pub struct RuntimeInfo {
    // pub group_info: AnimationGroupRuntimeInfo,
    /// 所属动画组的权重
    pub group_weight: f32,
    /// 动画进度
    pub amount_in_second: KeyFrameCurveValue,
    /// 作用的 目标对象 的ID
    pub target: IDAnimatableTarget,
    /// 作用的 目标对象 的目标属性 的ID
    pub attr: IDAnimatableAttr,
    /// 在曲线对应的数据类型 曲线信息管理器中 该动画使用的曲线 的 ID
    pub curve_id: FrameCurveInfoID,
    // pub anime: TargetAnimation,
}

/// 运行时信息表 - 唯一
/// 每个 Vec<RuntimeInfo> 分别对应一个 动画数据类型
pub struct RuntimeInfoMap {
    pub list: Vec<Vec<RuntimeInfo>>,
}

impl RuntimeInfoMap {
    pub fn default() -> Self {
        Self { list: vec![] }
    }
    /// 仅在分配 KeyAnimeDataType 后立即调用
    pub fn add_type(
        &mut self,
        ty: KeyFrameDataType,
    ) {
        self.list.push(vec![]);
    }
    /// 动画运行时记录
    pub fn insert(
        &mut self,
        ty: KeyFrameDataType,
        info: RuntimeInfo,
    ) -> Result<(), EAnimationError> {
        match self.list.get_mut(ty) {
            Some(list) => {
                list.push(info);
                Ok(())
            },
            None => Err(EAnimationError::RuntimeInfoMapNotFindType),
        }
    }
    pub fn reset(
        &mut self,
    ) {
        self.list.iter_mut().for_each(|x| x.clear());
    }
}