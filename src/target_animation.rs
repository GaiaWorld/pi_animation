use crate::{target_modifier::IDAnimatableTarget, animation::{AnimationInfo}};

/// Target动画 数据结构
/// * 关联动画目标 和 动画
#[derive(Debug, Clone, Copy)]
pub struct TargetAnimation<T> {
    pub target: T,
    pub animation: AnimationInfo,
}