#![feature(test)]
extern crate test;

use std::ops::Add;

use pi_animation::{target_modifier::{TAnimatableTargetModifier, IDAnimatableAttr, IDAnimatableTarget, TAnimatableTargetId, IDAnimatableTargetAllocator}, error::EAnimationError, animation_context::{TypeAnimationContext, AnimationContextAmount}, runtime_info::RuntimeInfoMap, frame_curve_manager::FrameCurveInfoManager, AnimatableFloat1};
use pi_curves::curve::frame::{FrameValueScale, FrameDataValue, KeyFrameDataType, KeyFrameCurveValue, KeyFrameDataTypeAllocator};

////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy)]
pub struct Value0(f32);
impl Add for Value0 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl FrameValueScale for Value0 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        Self(self.0 * rhs as f32)
    }
}
impl FrameDataValue for Value0 {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        Self(
            self.0 * (1.0 - amount) + rhs.0 * amount
        )
    }
}

////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy)]
pub struct Value1(u32, u32);
impl Add for Value1 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl FrameValueScale for Value1 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        Self(
            (self.0 as KeyFrameCurveValue * rhs) as u32,
            (self.1 as KeyFrameCurveValue * rhs) as u32
        )
    }
}
impl FrameDataValue for Value1 {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        Self(
            (self.0 as KeyFrameCurveValue * (1.0 - amount) + rhs.0 as KeyFrameCurveValue * amount) as u32,
            (self.0 as KeyFrameCurveValue * (1.0 - amount) + rhs.0 as KeyFrameCurveValue * amount) as u32,
        )
    }
}
////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////
pub struct Target0 {
    anim_target: IDAnimatableTarget,
    v0: Value0,
    v0a: Value0,
    v1: Value1,
    v2: AnimatableFloat1,
}
impl Target0 {
    pub fn default(anim_target: IDAnimatableTarget) -> Self {
        Self { v0: Value0(0.), v0a: Value0(0.), v1: Value1(0, 0), v2: AnimatableFloat1(0.), anim_target }
    }
}
/// 定义 Target0 对象的 可动画属性枚举
pub enum Target0AnimatableAttrSet {
    V0 = 0,
    V0a,
    V1,
    V2,
}
/// 为 Target0 实现  TAnimatableTargetId
impl TAnimatableTargetId for Target0 {
    fn anime_target_id(&self) -> IDAnimatableTarget {
        self.anim_target
    }
}
/// 为 Target0 实现  TAnimatableTargetModifier
impl TAnimatableTargetModifier<Value0> for Target0 {
    fn anime_modify(&mut self, attr: IDAnimatableAttr, value: Value0) -> Result<(), EAnimationError> {
        self.v0 = value;
        Ok(())
    }
}
impl TAnimatableTargetModifier<Value1> for Target0 {
    fn anime_modify(&mut self, attr: IDAnimatableAttr, value: Value1) -> Result<(), EAnimationError> {
        self.v1 = value;
        Ok(())
    }
}
impl TAnimatableTargetModifier<AnimatableFloat1> for Target0 {
    fn anime_modify(&mut self, attr: IDAnimatableAttr, value: AnimatableFloat1) -> Result<(), EAnimationError> {
        self.v2 = value;
        println!("Target Modify: Attr: {:?}, Value: {:?}", attr, value);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////
pub struct TypeAnimationContextMgr {
    pub value0_ctx: TypeAnimationContext<Value0>,
    pub value1_ctx: TypeAnimationContext<Value1>,
    pub f32_ctx: TypeAnimationContext<AnimatableFloat1>,
    pub runtime_infos: RuntimeInfoMap,
    pub curve_infos: FrameCurveInfoManager,
    pub target_allocator: IDAnimatableTargetAllocator,
    pub ty_allocator: KeyFrameDataTypeAllocator,
    pub animation_context_amount: AnimationContextAmount,
}

impl TypeAnimationContextMgr {
    pub fn default() -> Self {
        let mut runtime_infos = RuntimeInfoMap::default();
        let mut curve_infos = FrameCurveInfoManager::default();
        let mut target_allocator = IDAnimatableTargetAllocator::default();
        let mut animation_context_amount = AnimationContextAmount::default(target_allocator.allocat().unwrap());
        let mut ty_allocator = KeyFrameDataTypeAllocator::default();

        let value0_ctx = TypeAnimationContext::new(&mut ty_allocator, &mut runtime_infos, &mut curve_infos);
        let value1_ctx = TypeAnimationContext::new(&mut ty_allocator, &mut runtime_infos, &mut curve_infos);
        let f32_ctx = TypeAnimationContext::new(&mut ty_allocator, &mut runtime_infos, &mut curve_infos);
        Self { value0_ctx, value1_ctx, f32_ctx, runtime_infos, curve_infos, target_allocator, animation_context_amount, ty_allocator,  }
    }

    /// 运行动画
    pub fn anime(
        &mut self,
        delta_ms: u64,
    ) -> Result<(), Vec<EAnimationError>> {
        self.reset();

        self.animation_context_amount.anime_curve_calc(delta_ms, &mut self.runtime_infos);

        let mut r0 = self.value0_ctx.anime(&self.runtime_infos);
        let r1 = self.value1_ctx.anime(&self.runtime_infos);
        let r2 = self.f32_ctx.anime(&self.runtime_infos);

        // cb(&self.value0_ctx.result);

        // r0.extend(r1.iter().);
        r0
    }
    
    /// 运行动画
    pub fn anime_uncheck(
        &mut self,
        delta_ms: u64,
    ) {
        self.reset();

        self.animation_context_amount.anime_curve_calc(delta_ms, &mut self.runtime_infos);

        self.value0_ctx.anime_uncheck(&self.runtime_infos);
        self.value1_ctx.anime_uncheck(&self.runtime_infos);
        self.f32_ctx.anime_uncheck(&self.runtime_infos);
    }

    /// 动画中间数据清理
    pub fn reset(&mut self) {
        self.runtime_infos.reset();
        self.value0_ctx.reset();
        self.value1_ctx.reset();
        self.f32_ctx.reset();
    }

    /// 分配目标ID
    pub fn allocat_target_id(
        &mut self,
    ) -> IDAnimatableTarget {
        let id = self.target_allocator.allocat().unwrap();
        self.value0_ctx.record_target(id);
        self.value1_ctx.record_target(id);
        self.f32_ctx.record_target(id);

        id
    }
    /// 回收目标ID
    pub fn collect_target_id(
        &mut self,
        id: IDAnimatableTarget,
    ) {
        self.target_allocator.collect(id)
    }
}

#[cfg(test)]
mod test01 {
    use pi_animation::{animation_context::{AnimationContextAmount}, target_modifier::{IDAnimatableTargetAllocator, TAnimatableTargetModifier, IDAnimatableAttr, IDAnimatableTarget, TAnimatableTargetId}, loop_mode::ELoopMode, AnimatableFloat1, animation_listener::{AnimationListener, EAnimationEventResult}, curve_frame_event::CurveFrameEvent, amount::AnimationAmountCalc};
    use pi_curves::{curve::{frame_curve::FrameCurve, FrameIndex, frame::KeyFrameCurveValue}, easing::EEasingMode, steps::EStepMode};
    use pi_slotmap::SlotMap;
    use test::{Bencher};

    use crate::{TypeAnimationContextMgr, Value0, Target0, Target0AnimatableAttrSet};

    #[test]
    fn test_animatable_float1() {
        // 创建动画管理器
        let mut type_animation_ctx_mgr = TypeAnimationContextMgr::default();

        // 创建一个动画要作用的目标对象
        let mut target = Target0::default(type_animation_ctx_mgr.allocat_target_id());

        // 创建动画曲线
        let frame_count = 60 as FrameIndex;
        let mut curve1 = FrameCurve::curve_easing(AnimatableFloat1(0.0f32), AnimatableFloat1(100.0f32), frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
        let curve1_id = type_animation_ctx_mgr.f32_ctx.add_frame_curve(&mut type_animation_ctx_mgr.curve_infos, curve1);

        // 创建属性动画
        let animation0 = type_animation_ctx_mgr.animation_context_amount.add_animation(&mut type_animation_ctx_mgr.curve_infos, curve1_id, Target0AnimatableAttrSet::V2 as IDAnimatableAttr, type_animation_ctx_mgr.f32_ctx.ty()).unwrap();

        // 创建动画组
        let group0 = type_animation_ctx_mgr.animation_context_amount.create_animation_group(&mut type_animation_ctx_mgr.target_allocator);
        // 向动画组添加 动画
        type_animation_ctx_mgr.animation_context_amount.add_target_animation(animation0, group0, &target);
        // 启动动画组
        type_animation_ctx_mgr.animation_context_amount.start(group0, true, 1.0, ELoopMode::Not, 0.0, frame_count as KeyFrameCurveValue, 30, AnimationAmountCalc::default());

        // 动画运行
        type_animation_ctx_mgr.anime(100);

        // 查询动画结果
        let results = type_animation_ctx_mgr.f32_ctx.query_result(target.anime_target_id());
        println!("{:?}", results);
        results.iter().for_each(|value| {
            target.anime_modify(value.attr, value.value);
        });
    }

    #[derive(Debug, Clone, Copy)]
    pub enum TestFrameEventData {
        Test0,
        Test1,
    }

    #[test]
    fn test_event() {
        
        // let mut map = SlotMap::default();
        // map.

        // 创建动画管理器
        let mut type_animation_ctx_mgr = TypeAnimationContextMgr::default();

        // 创建一个动画要作用的目标对象
        let mut target = Target0::default(type_animation_ctx_mgr.allocat_target_id());

        // 创建动画曲线
        let frame_count = 60 as FrameIndex;
        let mut curve1 = FrameCurve::curve_easing(AnimatableFloat1(0.0f32), AnimatableFloat1(100.0f32), frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
        let curve1_id = type_animation_ctx_mgr.f32_ctx.add_frame_curve(&mut type_animation_ctx_mgr.curve_infos, curve1);

        // 创建属性动画
        let animation0 = type_animation_ctx_mgr.animation_context_amount.add_animation(&mut type_animation_ctx_mgr.curve_infos, curve1_id, Target0AnimatableAttrSet::V2 as IDAnimatableAttr, type_animation_ctx_mgr.f32_ctx.ty()).unwrap();

        // 创建动画组
        let group0 = type_animation_ctx_mgr.animation_context_amount.create_animation_group(&mut type_animation_ctx_mgr.target_allocator);
        // 向动画组添加 动画
        type_animation_ctx_mgr.animation_context_amount.add_target_animation(animation0, group0, &target);
        // 启动动画组
        type_animation_ctx_mgr.animation_context_amount.start(group0, true, 1.0, ELoopMode::Not, 0.0, frame_count as KeyFrameCurveValue, 30, AnimationAmountCalc::default());


        // 查询动画事件
        // 创建帧事件
        let mut curve_frame_event = CurveFrameEvent::<TestFrameEventData>::new(60);
        curve_frame_event.add(10, TestFrameEventData::Test0);
        curve_frame_event.add(50, TestFrameEventData::Test1);
        // 创建动画监听器 - 监听动画组 group0
        let mut listener = AnimationListener::<TestFrameEventData> { 
            group: group0,
            on_start: Some(Box::new(|| {
                println!("Group Event Start.");
                Ok(EAnimationEventResult::RemoveListen)
            })),
            on_end: Some(Box::new(|| {
                println!("Group Event End.");
                Ok(EAnimationEventResult::RemoveListen)
            })),
            on_loop: Some(Box::new(|looped_count| {
                println!("Group Event Loop {}.", looped_count);
                Ok(EAnimationEventResult::None)
            })),
            on_frame_event: Some(Box::new(|events| {
                events.iter().for_each(|v| {
                    println!("Group Event Frame Event {:?}.", v);
                });
                Ok(EAnimationEventResult::None)
            })),
        };
        
        for i in 0..30 {
            // 动画运行
            type_animation_ctx_mgr.anime(50);
            type_animation_ctx_mgr.animation_context_amount.animation_event(&mut listener, Some(&curve_frame_event));
            // 查询动画结果
            let results = type_animation_ctx_mgr.f32_ctx.query_result(target.anime_target_id());
            results.iter().for_each(|value| {
                target.anime_modify(value.attr, value.value);
            });
        }

    }
    
    #[test]
    fn test_step_amount() {
        
        // let mut map = SlotMap::default();
        // map.

        // 创建动画管理器
        let mut type_animation_ctx_mgr = TypeAnimationContextMgr::default();

        // 创建一个动画要作用的目标对象
        let mut target = Target0::default(type_animation_ctx_mgr.allocat_target_id());

        // 创建动画曲线
        let frame_count = 60 as FrameIndex;
        let mut curve1 = FrameCurve::curve_easing(AnimatableFloat1(0.0f32), AnimatableFloat1(100.0f32), frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
        let curve1_id = type_animation_ctx_mgr.f32_ctx.add_frame_curve(&mut type_animation_ctx_mgr.curve_infos, curve1);

        // 创建属性动画
        let animation0 = type_animation_ctx_mgr.animation_context_amount.add_animation(&mut type_animation_ctx_mgr.curve_infos, curve1_id, Target0AnimatableAttrSet::V2 as IDAnimatableAttr, type_animation_ctx_mgr.f32_ctx.ty()).unwrap();

        // 创建动画组
        let group0 = type_animation_ctx_mgr.animation_context_amount.create_animation_group(&mut type_animation_ctx_mgr.target_allocator);
        // 向动画组添加 动画
        type_animation_ctx_mgr.animation_context_amount.add_target_animation(animation0, group0, &target);
        // 启动动画组
        type_animation_ctx_mgr.animation_context_amount.start(group0, true, 1.0, ELoopMode::Not, 0.0, frame_count as KeyFrameCurveValue, 30, AnimationAmountCalc::from_steps(5, EStepMode::JumpNone));

        // 查询动画事件
        // 创建帧事件
        let mut curve_frame_event = CurveFrameEvent::<TestFrameEventData>::new(60);
        curve_frame_event.add(10, TestFrameEventData::Test0);
        curve_frame_event.add(50, TestFrameEventData::Test1);
        // 创建动画监听器 - 监听动画组 group0
        let mut listener = AnimationListener::<TestFrameEventData> { 
            group: group0,
            on_start: Some(Box::new(|| {
                println!("Group Event Start.");
                Ok(EAnimationEventResult::RemoveListen)
            })),
            on_end: Some(Box::new(|| {
                println!("Group Event End.");
                Ok(EAnimationEventResult::RemoveListen)
            })),
            on_loop: Some(Box::new(|looped_count| {
                println!("Group Event Loop {}.", looped_count);
                Ok(EAnimationEventResult::None)
            })),
            on_frame_event: Some(Box::new(|events| {
                events.iter().for_each(|v| {
                    println!("Group Event Frame Event {:?}.", v);
                });
                Ok(EAnimationEventResult::None)
            })),
        };
        
        for i in 0..30 {
            // 动画运行
            type_animation_ctx_mgr.anime(50);
            type_animation_ctx_mgr.animation_context_amount.animation_event(&mut listener, Some(&curve_frame_event));
            // 查询动画结果
            let results = type_animation_ctx_mgr.f32_ctx.query_result(target.anime_target_id());
            results.iter().for_each(|value| {
                target.anime_modify(value.attr, value.value);
            });
        }

    }
    
    #[bench]
    fn test_peformance(b: &mut Bencher) {
        let curve_range = 100_000;
        let animation_range = 100_000;
        let group_range = 10;
        let group_animation_range = 10000;
        let mut type_animation_ctx_mgr = TypeAnimationContextMgr::default();

        let frame_count = 60 as FrameIndex;
        // MinMaxCurve
        let mut curve0 = FrameCurve::curve_minmax_curve(Value0(0.0f32), Value0(1.0f32), 60);
        FrameCurve::curve_minmax_curve_frame(&mut curve0, 0, 0.0f32, 2.0f32, 2.0f32);
        FrameCurve::curve_minmax_curve_frame(&mut curve0, (frame_count/2) as FrameIndex, 0.5f32, 0.0f32, 0.0f32);
        FrameCurve::curve_minmax_curve_frame(&mut curve0, frame_count as FrameIndex, 1.0f32, 2.0f32, 2.0f32);

        let curve0_id = type_animation_ctx_mgr.value0_ctx.add_frame_curve(&mut type_animation_ctx_mgr.curve_infos, curve0);

        let mut curve1 = FrameCurve::curve_easing(Value0(0.0), Value0(1.0), frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
        let curve1_id = type_animation_ctx_mgr.value0_ctx.add_frame_curve(&mut type_animation_ctx_mgr.curve_infos, curve1);

        // 添加 100_000 曲线
        for _ in 0..curve_range {
            let mut c = FrameCurve::curve_easing(Value0(0.0), Value0(1.0), frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
            let v = type_animation_ctx_mgr.value0_ctx.add_frame_curve(&mut type_animation_ctx_mgr.curve_infos, c);
        }

        let animation0 = type_animation_ctx_mgr.animation_context_amount.add_animation(&mut type_animation_ctx_mgr.curve_infos, curve0_id, Target0AnimatableAttrSet::V0 as IDAnimatableAttr, type_animation_ctx_mgr.value0_ctx.ty()).unwrap();
        let animation1 = type_animation_ctx_mgr.animation_context_amount.add_animation(&mut type_animation_ctx_mgr.curve_infos, curve1_id, Target0AnimatableAttrSet::V0a as IDAnimatableAttr, type_animation_ctx_mgr.value0_ctx.ty()).unwrap();

        // 添加 100_000 动画
        for i in 0..animation_range {
            let a = type_animation_ctx_mgr.animation_context_amount.add_animation(&mut type_animation_ctx_mgr.curve_infos, curve0_id, Target0AnimatableAttrSet::V0a as IDAnimatableAttr, type_animation_ctx_mgr.value0_ctx.ty()).unwrap();
        }
        
        let mut targets = vec![];
        // 添加 10_000 目标对象
        for i in 0..group_animation_range {
            let mut target = Target0::default(type_animation_ctx_mgr.allocat_target_id());
            targets.push(target);
        }

        // 添加 10 动画组 每组 10000 动画
        for i in 0..group_range {
            let group0 = type_animation_ctx_mgr.animation_context_amount.create_animation_group(&mut type_animation_ctx_mgr.target_allocator);
            for j in 0..group_animation_range {
                type_animation_ctx_mgr.animation_context_amount.add_target_animation(i, group0, targets.get(j).unwrap());
            }
            type_animation_ctx_mgr.animation_context_amount.start(group0, true, 1.0, ELoopMode::Not, 0.0, frame_count as KeyFrameCurveValue, 30, AnimationAmountCalc::default());
        }
 
        // 测试 动画性能 计 10w 个动画计算 & 10_000 个对象的数据修改
        b.iter(move || {
        
            type_animation_ctx_mgr.anime(1);
            // type_animation_ctx_mgr.anime_uncheck(1);

            for i in 0..group_animation_range {
                let target = targets.get_mut(i).unwrap();
                let results = type_animation_ctx_mgr.value0_ctx.query_result(i);

                results.iter().for_each(|value| {
                    target.anime_modify(value.attr, value.value);
                });
            }
        });
    }
}