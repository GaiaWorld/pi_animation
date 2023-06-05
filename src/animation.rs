use pi_curves::curve::{frame::{KeyFrameDataType, KeyFrameCurveValue}, FramePerSecond, FrameIndex};

use crate::{target_modifier::{IDAnimatableAttr}, frame_curve_manager::{FrameCurveInfoID, FrameCurveInfo}};

pub type AnimationID = usize;

/// 属性动画 数据结构
/// * 关联 属性ID 和 动画曲线
#[derive(Debug)]
pub struct AnimationInfo {
    /// 属性ID
    pub attr: IDAnimatableAttr,
    /// 数据类型 ID
    pub ty: KeyFrameDataType,
    /// 曲线 的描述信息
    pub curve_info: FrameCurveInfo,
    /// 曲线 的描述信息 的ID
    pub curve_id: FrameCurveInfoID,
}

impl AnimationInfo {
    pub fn get_max_frame_for_running_speed(&self, running_frame_per_second: FramePerSecond) -> KeyFrameCurveValue {
        // println!("{:?}", self.curve_info);
        self.curve_info.max_frame() as KeyFrameCurveValue / self.curve_info.design_frame_per_second() as KeyFrameCurveValue * running_frame_per_second as KeyFrameCurveValue
    }
    pub fn max_frame(
        &self,
    ) -> FrameIndex {
        self.curve_info.max_frame()
    }
    pub fn min_frame(
        &self,
    ) -> FrameIndex {
        self.curve_info.min_frame()
    }
    pub fn design_frame_per_second(
        &self,
    ) -> FrameIndex {
        self.curve_info.design_frame_per_second()
    }
    pub fn ty(&self) -> KeyFrameDataType {
        self.ty
    }
    pub fn attr(&self) -> IDAnimatableAttr {
        self.attr
    }
    pub fn curve_id(&self) -> FrameCurveInfoID {
        self.curve_id
    }
}
