use std::ops::Deref;

use pi_curves::curve::frame::KeyFrameCurveValue;


pub type TimeMS = KeyFrameCurveValue;

/// 动画启动和结束时的状态控制
#[derive(Debug, Clone, Copy)]
pub struct EFillMode(u8);
impl Deref for EFillMode {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl EFillMode {
    /// 无动画操作
    pub const NONE: Self = EFillMode(0);
    /// 动画完成后保持最后一帧状态
    pub const FORWARDS: Self = EFillMode(1);
    /// 有延时启动的情况, 动画启动前保持第一帧状态
    pub const BACKWARDS: Self = EFillMode(2);
    /// 同时应用 Forwards Backwards
    pub const BOTH: Self = EFillMode(3);
}