use pi_curves::curve::{frame::{KeyFrameDataType, KeyFrameCurveValue}, FramePerSecond, FrameIndex};

use crate::{target_modifier::{IDAnimatableAttr}, error::EAnimationError, frame_curve_manager::{FrameCurveInfoID, FrameCurveInfo}};

pub type AnimationID = usize;

pub trait AnimationManager {
    /// 创建一个属性动画
    fn create(&mut self, attr: IDAnimatableAttr, ty: KeyFrameDataType, curve_info: FrameCurveInfo, curve_id: FrameCurveInfoID,) -> AnimationID;
    /// 删除一个属性动画
    fn del(&mut self, id: AnimationID, ) -> Result<(), EAnimationError>;
    /// 获取指定属性动画
    fn get(&self, id: AnimationID,) -> Result<AnimationInfo, EAnimationError>;
}

/// 属性动画 数据结构
/// * 关联 属性ID 和 动画曲线
#[derive(Debug, Clone, Copy)]
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

/// 属性动画 数据的管理器
pub struct AnimationManagerDefault {
    id_pool: Vec<AnimationID>,
    counter: AnimationID,
    animation_infos: Vec<AnimationInfo>,
}

impl Default for AnimationManagerDefault {
    fn default() -> Self {
        Self {
            id_pool: vec![],
            counter: 0,
            animation_infos: vec![],
        }
    }
}

impl AnimationManager for AnimationManagerDefault {
    /// 创建一个属性动画
    fn create(
        &mut self,
        attr: IDAnimatableAttr,
        ty: KeyFrameDataType,
        curve_info: FrameCurveInfo,
        curve_id: FrameCurveInfoID
    ) -> AnimationID {
        let id = match self.id_pool.pop() {
            Some(id) => {
                let info = self.animation_infos.get_mut(id).unwrap();
                info.attr = attr;
                info.ty = ty;
                info.curve_info = curve_info;
                info.curve_id = curve_id;

                id as AnimationID
            },
            None => {
                let id = self.counter;
                self.counter += 1;

                self.animation_infos.push(AnimationInfo { attr, ty, curve_info, curve_id });
                id
            },
        };

        id
    }
    /// 删除一个属性动画
    fn del(
        &mut self,
        id: AnimationID,
    ) -> Result<(), EAnimationError> {
        if id < self.counter {
            // 回收 ID
            if !self.id_pool.contains(&id) {
                self.id_pool.push(id);
            }
            Ok(())
        } else {
            Err(EAnimationError::FrameCurveNotFound)
        }
    }
    /// 获取指定属性动画
    fn get(
        &self,
        id: AnimationID,
    ) -> Result<AnimationInfo, EAnimationError> {
        match self.animation_infos.get(id) {
            Some(v) => Ok(*v),
            None => Err(EAnimationError::FrameCurveNotFound),
        }
    }
}