use crate::{animation::{AnimationInfo}};

/// Target动画 数据结构
/// * 关联动画目标 和 动画
#[derive(Debug)]
pub struct TargetAnimation<T> {
    pub target: T,
    pub animation: AnimationInfo,
}