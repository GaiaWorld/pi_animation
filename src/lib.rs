use std::ops::Add;

use pi_curves::curve::frame::{KeyFrameCurveValue, FrameDataValue, FrameValueScale};


pub mod target_modifier;
pub mod error;
pub mod animation;
pub mod animation_context;
pub mod animation_group;
pub mod loop_mode;
pub mod frame_curve_manager;
pub mod runtime_info;
pub mod target_animation;
pub mod animation_listener;
pub mod curve_frame_event;
pub mod amount;

/// 可动画的 f32 数据
#[derive(Debug, Clone, Copy)]
pub struct AnimatableFloat1(pub f32);
impl Add for AnimatableFloat1 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl FrameValueScale for AnimatableFloat1 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        Self(
            self.0 * rhs as f32
        )
    }
}
impl FrameDataValue for AnimatableFloat1 {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        Self (
            self.0 * (1.0 - amount) + rhs.0 * amount
        )
    }
}

/// 可动画的 (f32, f32) 数据
#[derive(Debug, Clone, Copy)]
pub struct AnimatableFloat2(pub f32, pub f32);
impl Add for AnimatableFloat2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl FrameValueScale for AnimatableFloat2 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        Self(
            self.0 * rhs as f32,
            self.1 * rhs as f32,
        )
    }
}
impl FrameDataValue for AnimatableFloat2 {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        Self(
            self.0 * (1.0 - amount) + rhs.0 * amount,
            self.1 * (1.0 - amount) + rhs.1 * amount
        )
        
    }
}

/// 可动画的 (f32, f32, f32) 数据
#[derive(Debug, Clone, Copy)]
pub struct AnimatableFloat3(pub f32, pub f32, pub f32);
impl Add for AnimatableFloat3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl FrameValueScale for AnimatableFloat3 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        Self(
            self.0 * rhs as f32,
            self.1 * rhs as f32,
            self.2 * rhs as f32,
        )
    }
}
impl FrameDataValue for AnimatableFloat3 {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        Self(
            self.0 * (1.0 - amount) + rhs.0 * amount,
            self.1 * (1.0 - amount) + rhs.1 * amount,
            self.2 * (1.0 - amount) + rhs.2 * amount,
        )
    }
}

/// 可动画的 (f32, f32, f32, f32) 数据
#[derive(Debug, Clone, Copy)]
pub struct AnimatableFloat4(pub f32, pub f32, pub f32, pub f32);
impl Add for AnimatableFloat4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}
impl FrameValueScale for AnimatableFloat4 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        Self(
            self.0 * rhs as f32,
            self.1 * rhs as f32,
            self.2 * rhs as f32,
            self.3 * rhs as f32,
        )
    }
}
impl FrameDataValue for AnimatableFloat4 {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        Self(
            self.0 * (1.0 - amount) + rhs.0 * amount,
            self.1 * (1.0 - amount) + rhs.1 * amount,
            self.2 * (1.0 - amount) + rhs.2 * amount,
            self.3 * (1.0 - amount) + rhs.3 * amount,
        )
    }
}