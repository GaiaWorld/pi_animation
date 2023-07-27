#![feature(test)]
extern crate test;

use std::{ops::Add, sync::Arc};

use pi_animation::{target_modifier::{TAnimatableTargetModifier, IDAnimatableAttr, TAnimatableTargetId}, error::EAnimationError, type_animation_context::{TypeAnimationContext, AnimationContextAmount}, runtime_info::RuntimeInfoMap, animation_result_pool::{TypeAnimationResultPoolDefault, TypeAnimationResultPool}, animation_group_manager::AnimationGroupManagerDefault};
use pi_curves::curve::{frame::{FrameValueScale, FrameDataValue, KeyFrameCurveValue, KeyFrameDataTypeAllocator}, frame_curve::FrameCurve};
use pi_slotmap::{DefaultKey, SlotMap};

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
////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////
pub struct Target0 {
    anim_target: DefaultKey,
    v0: Value0,
    v0a: Value0,
    v1: Value1,
    v2: f32,
}
impl Target0 {
    pub fn default(anim_target: DefaultKey) -> Self {
        Self { v0: Value0(0.), v0a: Value0(0.), v1: Value1(0, 0), v2: 0., anim_target }
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
impl TAnimatableTargetId<DefaultKey> for Target0 {
    fn anime_target_id(&self) -> DefaultKey {
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
impl TAnimatableTargetModifier<f32> for Target0 {
    fn anime_modify(&mut self, attr: IDAnimatableAttr, value: f32) -> Result<(), EAnimationError> {
        self.v2 = value;
        println!("Target Modify: Attr: {:?}, Value: {:?}", attr, value);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////
pub struct TypeAnimationContextMgr {
    pub value0_ctx: TypeAnimationContext<Value0, AssetCurve<Value0>>,
    pub value0_result_pool: TypeAnimationResultPoolDefault<Value0>,
    pub value1_ctx: TypeAnimationContext<Value1, AssetCurve<Value1>>,
    pub value1_result_pool: TypeAnimationResultPoolDefault<Value1>,
    pub f32_ctx: TypeAnimationContext<f32, AssetCurve<f32>>,
    pub f32_result_pool: TypeAnimationResultPoolDefault<f32>,
    pub runtime_infos: RuntimeInfoMap<DefaultKey>,
    // pub curve_infos: FrameCurveInfoManager,
    // pub target_allocator: IDAnimatableTargetAllocatorDefault,
	pub target_allocator: SlotMap<DefaultKey, ()>,
    pub ty_allocator: KeyFrameDataTypeAllocator,
}

impl TypeAnimationContextMgr {
    pub fn default() -> Self {
        let mut runtime_infos = RuntimeInfoMap::default();
        // let mut curve_infos = FrameCurveInfoManager::default();
        let mut target_allocator = SlotMap::default();
        let mut ty_allocator = KeyFrameDataTypeAllocator::default();


        let value0_ctx = TypeAnimationContext::new(ty_allocator.alloc().unwrap(), &mut runtime_infos);
        let value0_result_pool = TypeAnimationResultPoolDefault::default();
        let value1_ctx = TypeAnimationContext::new(ty_allocator.alloc().unwrap(), &mut runtime_infos);
        let value1_result_pool = TypeAnimationResultPoolDefault::default();
        let f32_ctx = TypeAnimationContext::new(ty_allocator.alloc().unwrap(), &mut runtime_infos);
        let f32_result_pool = TypeAnimationResultPoolDefault::default();
        Self {
            value0_ctx, value0_result_pool,
            value1_ctx, value1_result_pool,
            f32_ctx, f32_result_pool,
            runtime_infos,
            // curve_infos,
            target_allocator, ty_allocator,  }
    }

    /// 运行动画
    pub fn anime(
        &mut self,
        animation_context_amount: &mut AnimationContextAmount<DefaultKey, AnimationGroupManagerDefault<DefaultKey>>,
        delta_ms: u64,
    ) -> Result<(), Vec<EAnimationError>> {
        self.reset();

        animation_context_amount.anime_curve_calc(delta_ms, &mut self.runtime_infos);

        let mut r0 = self.value0_ctx.anime(&self.runtime_infos, &mut self.value0_result_pool);
        let r1 = self.value1_ctx.anime(&self.runtime_infos, &mut self.value1_result_pool);
        let r2 = self.f32_ctx.anime(&self.runtime_infos, &mut self.f32_result_pool);

        // cb(&self.value0_ctx.result);

        // r0.extend(r1.iter().);
        r0
    }
    
    /// 运行动画
    pub fn anime_uncheck(
        &mut self,
        animation_context_amount: &mut AnimationContextAmount<DefaultKey, AnimationGroupManagerDefault<DefaultKey>>,
        delta_ms: u64,
    ) {
        self.reset();

        animation_context_amount.anime_curve_calc(delta_ms, &mut self.runtime_infos);

        self.value0_ctx.anime_uncheck(&mut self.runtime_infos, &mut self.value0_result_pool);
        self.value1_ctx.anime_uncheck(&mut self.runtime_infos, &mut self.value1_result_pool);
        self.f32_ctx.anime_uncheck(&mut self.runtime_infos, &mut self.f32_result_pool);
    }

    /// 动画中间数据清理
    pub fn reset(&mut self) {
        self.runtime_infos.reset();
        self.value0_result_pool.reset();
        self.value1_result_pool.reset();
        self.f32_result_pool.reset();
    }

    /// 分配目标ID
    pub fn allocat_target_id(
        &mut self,
    ) -> DefaultKey {
        let id = self.target_allocator.insert(());
        self.value0_result_pool.record_target(id);
        self.value1_result_pool.record_target(id);
        self.f32_result_pool.record_target(id);

        id
    }
}

#[derive(Debug, Clone)]
pub struct AssetCurve<F: FrameDataValue>(pub Arc<FrameCurve<F>>);
impl<F: FrameDataValue> AsRef<FrameCurve<F>> for AssetCurve<F> {
    fn as_ref(&self) -> &FrameCurve<F> {
        &self.0
    }
}

#[cfg(test)]
mod test01 {
    use std::{sync::Arc, mem::replace};

    use pi_animation::{type_animation_context::{AnimationContextAmount}, target_modifier::{IDAnimatableTargetAllocator, TAnimatableTargetModifier, IDAnimatableAttr, IDAnimatableTarget, TAnimatableTargetId}, loop_mode::ELoopMode, animation_listener::{AnimationListener, EAnimationEventResult}, curve_frame_event::CurveFrameEvent, amount::AnimationAmountCalc, animation_group_manager::AnimationGroupManagerDefault, base::EFillMode};
    use pi_curves::{curve::{frame_curve::FrameCurve, FrameIndex, frame::KeyFrameCurveValue}, easing::EEasingMode, steps::EStepMode};
    use pi_slotmap::{SlotMap, DefaultKey};
    use test::{Bencher};

    use crate::{TypeAnimationContextMgr, Value0, Target0, Target0AnimatableAttrSet, AssetCurve};

    #[test]
    fn test_animatable_float1() {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

        // 创建动画管理器
        let mut type_animation_ctx_mgr = TypeAnimationContextMgr::default();
        
        let mut animation_context_amount = AnimationContextAmount::<DefaultKey, AnimationGroupManagerDefault<DefaultKey>>::default(AnimationGroupManagerDefault::default());
        animation_context_amount.debug(true);

        // 创建一个动画要作用的目标对象
        let mut target = Target0::default(type_animation_ctx_mgr.allocat_target_id());

        // 创建动画曲线
        let frame_count = 30 as FrameIndex;
        let key_curve1 = 0;
        let mut curve1 = FrameCurve::curve_easing(0.0f32, 100.0f32, frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
        let curve1 = crate::AssetCurve::<f32>(Arc::new(curve1));
        let animation0 = type_animation_ctx_mgr.f32_ctx.create_animation(Target0AnimatableAttrSet::V2 as IDAnimatableAttr, curve1);

        // 创建动画组
        let group0 = animation_context_amount.create_animation_group();
        // 向动画组添加 动画
        animation_context_amount.add_target_animation_notype(animation0, group0, target.anime_target_id());
        // 启动动画组
        animation_context_amount.start_with_progress(group0, 1.0, ELoopMode::Not, 0.0, 1., frame_count, AnimationAmountCalc::default(), 100., EFillMode::NONE);

        // 动画运行
        type_animation_ctx_mgr.anime(&mut animation_context_amount, 70);
        // 查询动画结果
        let results = type_animation_ctx_mgr.f32_result_pool.query_result(target.anime_target_id());
        println!("{:?}", results);
        results.iter().for_each(|value| {
            target.anime_modify(value.attr, value.value);
        });

        type_animation_ctx_mgr.anime(&mut animation_context_amount, 51);
        // 查询动画结果
        let results = type_animation_ctx_mgr.f32_result_pool.query_result(target.anime_target_id());
        println!("{:?}", results);
        results.iter().for_each(|value| {
            target.anime_modify(value.attr, value.value);
        });
        
        for i in 0..22 {
            type_animation_ctx_mgr.anime(&mut animation_context_amount, 50);
            // 查询动画结果
            let results = type_animation_ctx_mgr.f32_result_pool.query_result(target.anime_target_id());
            println!("{:?}", results);
            results.iter().for_each(|value| {
                target.anime_modify(value.attr, value.value);
            });
        }
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
        let mut animation_context_amount = AnimationContextAmount::<DefaultKey, AnimationGroupManagerDefault<DefaultKey>>::default(AnimationGroupManagerDefault::default());
        animation_context_amount.debug(true);

        // 创建一个动画要作用的目标对象
        let mut target = Target0::default(type_animation_ctx_mgr.allocat_target_id());

        // 创建动画曲线
        let frame_count = 60 as FrameIndex;
        let key_curve1 = 0;
        let mut curve1 = FrameCurve::curve_easing(0.0f32, 100.0f32, frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
        let curve1 = crate::AssetCurve::<f32>(Arc::new(curve1));
        // 创建属性动画
        let animation0 = type_animation_ctx_mgr.f32_ctx.create_animation(Target0AnimatableAttrSet::V2 as IDAnimatableAttr, curve1);

        // 创建动画组
        let group0 = animation_context_amount.create_animation_group();
        // 向动画组添加 动画
        animation_context_amount.add_target_animation_notype(animation0, group0, target.anime_target_id());
        // 启动动画组
        animation_context_amount.start_complete(group0, 1.0, ELoopMode::Not, 30, AnimationAmountCalc::default(), 0., EFillMode::BACKWARDS);


        // 查询动画事件
        // 创建帧事件
        let mut curve_frame_event = CurveFrameEvent::<TestFrameEventData>::new(60.0);
        // curve_frame_event.add(10, TestFrameEventData::Test0);
        // curve_frame_event.add(50, TestFrameEventData::Test1);
        // 创建动画监听器 - 监听动画组 group0
        let mut listener = AnimationListener::<TestFrameEventData> { 
            group: group0,
            on_start: vec![Box::new(|| {
                println!("Group Event Start.");
                Ok(EAnimationEventResult::RemoveListen)
            })],
            on_end: vec![Box::new(|| {
                println!("Group Event End.");
                Ok(EAnimationEventResult::RemoveListen)
            })],
            on_loop: vec![Box::new(|looped_count| {
                println!("Group Event Loop {}.", looped_count);
                Ok(EAnimationEventResult::None)
            })],
            on_frame_event: vec![Box::new(|events| {
                events.iter().for_each(|v| {
                    println!("Group Event Frame Event {:?}.", v);
                });
                Ok(EAnimationEventResult::None)
            })],
        };
        
        for i in 0..100 {
            // 动画运行
            // type_animation_ctx_mgr.anime(50);
            // animation_context_amount.animation_event(&mut listener, Some(&curve_frame_event));
            // // 查询动画结果
            // let results = type_animation_ctx_mgr.f32_result_pool.query_result(target.anime_target_id());
            // results.iter().for_each(|value| {
            //     target.anime_modify(value.attr, value.value);
            // });

            type_animation_ctx_mgr.runtime_infos.reset();
            animation_context_amount.anime_curve_calc(30, &mut type_animation_ctx_mgr.runtime_infos);
            for (group_id, ty, count) in animation_context_amount.group_events.iter() {
                println!("AG: {:?}, {:?}, {:?}", group_id, ty, count);
            }
        }

    }
    
    #[test]
    fn test_step_amount() {

        // let mut map = SlotMap::default();
        // map.

        // 创建动画管理器
        let mut type_animation_ctx_mgr = TypeAnimationContextMgr::default();
        let mut animation_context_amount = AnimationContextAmount::<DefaultKey, AnimationGroupManagerDefault<DefaultKey>>::default(AnimationGroupManagerDefault::default());
        animation_context_amount.debug(true);

        // 创建一个动画要作用的目标对象
        let mut target = Target0::default(type_animation_ctx_mgr.allocat_target_id());

        // 创建动画曲线
        let frame_count = 60 as FrameIndex;
        let key_curve1 = 0;
        let mut curve1 = FrameCurve::curve_easing(0.0f32, 100.0f32, frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
        let curve1 = crate::AssetCurve::<f32>(Arc::new(curve1));
        // 创建属性动画
        let animation0 = type_animation_ctx_mgr.f32_ctx.create_animation(Target0AnimatableAttrSet::V2 as IDAnimatableAttr, curve1);

        // 创建动画组
        let group0 = animation_context_amount.create_animation_group();
        // 向动画组添加 动画
        animation_context_amount.add_target_animation_notype(animation0, group0, target.anime_target_id());
        // 启动动画组
        animation_context_amount.start_complete(group0, 1.0, ELoopMode::Not, 30, AnimationAmountCalc::from_steps(5, EStepMode::JumpNone), 0., EFillMode::BACKWARDS);

        // 查询动画事件
        // 创建帧事件
        let mut curve_frame_event = CurveFrameEvent::<TestFrameEventData>::new(60.);
        curve_frame_event.add(10, TestFrameEventData::Test0);
        curve_frame_event.add(50, TestFrameEventData::Test1);
        // 创建动画监听器 - 监听动画组 group0
        let mut listener = AnimationListener::<TestFrameEventData> { 
            group: group0,
            on_start: vec![Box::new(|| {
                println!("Group Event Start.");
                Ok(EAnimationEventResult::RemoveListen)
            })],
            on_end: vec![Box::new(|| {
                println!("Group Event End.");
                Ok(EAnimationEventResult::RemoveListen)
            })],
            on_loop: vec![Box::new(|looped_count| {
                println!("Group Event Loop {}.", looped_count);
                Ok(EAnimationEventResult::None)
            })],
            on_frame_event: vec![Box::new(|events| {
                events.iter().for_each(|v| {
                    println!("Group Event Frame Event {:?}.", v);
                });
                Ok(EAnimationEventResult::None)
            })],
        };

        for i in 0..300 {
            // 动画运行
            type_animation_ctx_mgr.anime(&mut animation_context_amount, 50);
            animation_context_amount.animation_event(&mut listener, Some(&curve_frame_event));
            // 查询动画结果
            let results = type_animation_ctx_mgr.f32_result_pool.query_result(target.anime_target_id());
            results.iter().for_each(|value| {
                target.anime_modify(value.attr, value.value);
            });
        }

    }
    
    #[test]
    fn test_frames() {
        
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

        // let mut map = SlotMap::default();
        // map.

        // 创建动画管理器
        let mut type_animation_ctx_mgr = TypeAnimationContextMgr::default();
        let mut animation_context_amount = AnimationContextAmount::<DefaultKey, AnimationGroupManagerDefault<DefaultKey>>::default(AnimationGroupManagerDefault::default());
        animation_context_amount.debug(true);


        // 创建动画曲线
        let frame_count = 60 as FrameIndex;
        let key_curve1 = 0;
        let mut curve1 = FrameCurve::curve_frame_values(10000);
        FrameCurve::curve_frame_values_frame(&mut curve1, 0, 0.0);
        FrameCurve::curve_frame_values_frame(&mut curve1, 10000, 1.0);
        println!("{:?}", curve1);
        let curve1 = crate::AssetCurve::<f32>(Arc::new(curve1));

        let mut targets = vec![]; [
            Target0::default(type_animation_ctx_mgr.allocat_target_id()),
            Target0::default(type_animation_ctx_mgr.allocat_target_id()),
            Target0::default(type_animation_ctx_mgr.allocat_target_id()),
        ];
        // let mut listeners = vec![];
        {
            for i in 0..1 {
                type_animation_ctx_mgr.runtime_infos.reset();
                animation_context_amount.anime_curve_calc(200, &mut type_animation_ctx_mgr.runtime_infos);
                let mut list = replace(&mut animation_context_amount.group_events, vec![]);
                for (group_id, ty, count) in list.iter() {
                    println!("AG 0: {:?}, {:?}, {:?}", group_id, ty, count);
                    match ty {
                        pi_animation::animation_listener::EAnimationEvent::None => {},
                        pi_animation::animation_listener::EAnimationEvent::Start => {},
                        pi_animation::animation_listener::EAnimationEvent::End => {
                            animation_context_amount.del_animation_group(*group_id);
                        },
                        pi_animation::animation_listener::EAnimationEvent::Loop => {},
                        pi_animation::animation_listener::EAnimationEvent::FrameEvent => {},
                    }
                }

                for _ in 0..3 {
                    targets.push(
                        Target0::default(type_animation_ctx_mgr.allocat_target_id())
                    );
                    // 创建一个动画要作用的目标对象
                    let target = targets.get(i).unwrap();
                    // 创建动画组
                    let group0 = animation_context_amount.create_animation_group();
                    // 创建属性动画
                    let animation0 = type_animation_ctx_mgr.f32_ctx.create_animation(Target0AnimatableAttrSet::V2 as IDAnimatableAttr, curve1.clone());
                    // 向动画组添加 动画
                    animation_context_amount.add_target_animation_notype(animation0, group0, target.anime_target_id());
                    // 启动动画组
                    animation_context_amount.start_complete(group0, -0.5 - (i % 5) as f32 * 0.1, ELoopMode::Not, 60, AnimationAmountCalc::default(), 0., EFillMode::BACKWARDS);
                } 
                
                // // 创建动画监听器 - 监听动画组 group0
                // let mut listener = AnimationListener::<TestFrameEventData> { 
                //     group: group0,
                //     on_start: vec![Box::new(|| {
                //         println!("Group Event Start.");
                //         Ok(EAnimationEventResult::RemoveListen)
                //     })],
                //     on_end: vec![Box::new(|| {
                //         println!("Group Event End.");
                //         Ok(EAnimationEventResult::RemoveListen)
                //     })],
                //     on_loop: vec![Box::new(|looped_count| {
                //         println!("Group Event Loop {}.", looped_count);
                //         Ok(EAnimationEventResult::None)
                //     })],
                //     on_frame_event: vec![Box::new(|events| {
                //         events.iter().for_each(|v| {
                //             println!("Group Event Frame Event {:?}.", v);
                //         });
                //         Ok(EAnimationEventResult::None)
                //     })],
                // };

                // listeners.push(listener);
            }
        }

        // 查询动画事件
        // 创建帧事件
        let mut curve_frame_event = CurveFrameEvent::<TestFrameEventData>::new(60.);
        curve_frame_event.add(10, TestFrameEventData::Test0);
        curve_frame_event.add(50, TestFrameEventData::Test1);

        // animation_context_amount.log_groups();

        for i in 0..200 {
            // // 动画运行
            // type_animation_ctx_mgr.anime(50);
            
            // for i in 0..3 {
            //     let target = targets.get_mut(i).unwrap();
            //     let listener = listeners.get_mut(i).unwrap();
            //     animation_context_amount.animation_event(listener, Some(&curve_frame_event));
            //     // 查询动画结果
            //     let results = type_animation_ctx_mgr.f32_result_pool.query_result(target.anime_target_id());
            //     results.iter().for_each(|value| {
            //         target.anime_modify(value.attr, value.value);
            //     });
            // }
            
            type_animation_ctx_mgr.runtime_infos.reset();
            animation_context_amount.anime_curve_calc(40, &mut type_animation_ctx_mgr.runtime_infos);
            let mut list = replace(&mut animation_context_amount.group_events, vec![]);
            for (group_id, ty, count) in list.iter() {
                println!("AG 1: {:?}, {:?}, {:?}", group_id, ty, count);
                match ty {
                    pi_animation::animation_listener::EAnimationEvent::None => {},
                    pi_animation::animation_listener::EAnimationEvent::Start => {},
                    pi_animation::animation_listener::EAnimationEvent::End => {
                        animation_context_amount.del_animation_group(*group_id);
                    },
                    pi_animation::animation_listener::EAnimationEvent::Loop => {},
                    pi_animation::animation_listener::EAnimationEvent::FrameEvent => {},
                }
            }
        }

    }

    #[bench]
    fn test_peformance(b: &mut Bencher) {
        let curve_range = 100_000;
        let animation_range = 100_000;
        let group_range = 100;
        let group_animation_range = 1000;
        let mut type_animation_ctx_mgr = TypeAnimationContextMgr::default();
        let mut animation_context_amount = AnimationContextAmount::<DefaultKey, AnimationGroupManagerDefault<DefaultKey>>::default(AnimationGroupManagerDefault::default());
        animation_context_amount.debug(true);

        let frame_count = 60 as FrameIndex;
        // MinMaxCurve
        let key_curve0 = 0;
        let mut curve0 = FrameCurve::curve_minmax_curve(Value0(0.0f32), Value0(1.0f32), 60);
        FrameCurve::curve_minmax_curve_frame(&mut curve0, 0, 0.0f32, 2.0f32, 2.0f32);
        FrameCurve::curve_minmax_curve_frame(&mut curve0, (frame_count/2) as FrameIndex, 0.5f32, 0.0f32, 0.0f32);
        FrameCurve::curve_minmax_curve_frame(&mut curve0, frame_count as FrameIndex, 1.0f32, 2.0f32, 2.0f32);
        let curve0 = crate::AssetCurve::<Value0>(Arc::new(curve0));


        let key_curve1 = 1;
        let mut curve1 = FrameCurve::curve_easing(Value0(0.0), Value0(1.0), frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
        let curve1 = crate::AssetCurve::<Value0>(Arc::new(curve1));


        let mut targets = vec![];
        // 添加 10_000 目标对象
        for i in 0..group_animation_range {
            let mut target = Target0::default(type_animation_ctx_mgr.allocat_target_id());
            targets.push(target);
        }

        // 添加 10 动画组 每组 10000 动画
        for i in 0..group_range {
            let group0 = animation_context_amount.create_animation_group();
            for j in 0..group_animation_range {
                let animation = if j % 2 == 0 {
                    // 创建属性动画
                    type_animation_ctx_mgr.value0_ctx.create_animation(Target0AnimatableAttrSet::V0 as IDAnimatableAttr, curve0.clone())
                } else {
                    // 创建属性动画
                    type_animation_ctx_mgr.value0_ctx.create_animation(Target0AnimatableAttrSet::V0a as IDAnimatableAttr, curve1.clone())
                };
                animation_context_amount.add_target_animation_notype(animation, group0, targets.get(j).unwrap().anime_target_id());
            }
            animation_context_amount.start_complete(group0, 1.0, ELoopMode::Opposite(None), 60, AnimationAmountCalc::default(), 0., EFillMode::BACKWARDS);
        }
        animation_context_amount.debug(true);

        type_animation_ctx_mgr.anime(&mut animation_context_amount, 1);

        // 测试 动画性能 计 10w 个动画计算 & 10_000 个对象的数据修改
        b.iter(move || {

            type_animation_ctx_mgr.anime(&mut animation_context_amount, 1);
            // type_animation_ctx_mgr.anime_uncheck(1);
            
            // let mut r0 = type_animation_ctx_mgr.value0_ctx.anime(&type_animation_ctx_mgr.runtime_infos, &mut type_animation_ctx_mgr.value0_result_pool);
            // let r1 = type_animation_ctx_mgr.value1_ctx.anime(&type_animation_ctx_mgr.runtime_infos, &mut type_animation_ctx_mgr.value1_result_pool);
            // let r2 = type_animation_ctx_mgr.f32_ctx.anime(&type_animation_ctx_mgr.runtime_infos, &mut type_animation_ctx_mgr.f32_result_pool);


            let mut ii = 0;
            for i in 0..group_animation_range {
                let target = targets.get_mut(i).unwrap();
                let results = type_animation_ctx_mgr.value0_result_pool.query_result(target.anime_target_id());

                results.iter().for_each(|value| {
                    target.anime_modify(value.attr, value.value);
                    ii += 1;
                });
            }
        });
    }
    
    #[bench]
    fn test_peformance_2(b: &mut Bencher) {
        let curve_range = 100_000;
        let animation_range = 100_000;
        let group_range = 100;
        let group_animation_range = 1000;
        let mut type_animation_ctx_mgr = TypeAnimationContextMgr::default();
        let mut animation_context_amount = AnimationContextAmount::<DefaultKey, AnimationGroupManagerDefault<DefaultKey>>::default(AnimationGroupManagerDefault::default());
        animation_context_amount.debug(true);

        let frame_count = 60 as FrameIndex;
        // MinMaxCurve
        let key_curve0 = 0;
        let mut curve0 = FrameCurve::curve_minmax_curve(Value0(0.0f32), Value0(1.0f32), 60);
        FrameCurve::curve_minmax_curve_frame(&mut curve0, 0, 0.0f32, 2.0f32, 2.0f32);
        FrameCurve::curve_minmax_curve_frame(&mut curve0, (frame_count/2) as FrameIndex, 0.5f32, 0.0f32, 0.0f32);
        FrameCurve::curve_minmax_curve_frame(&mut curve0, frame_count as FrameIndex, 1.0f32, 2.0f32, 2.0f32);
        let curve0 = crate::AssetCurve::<Value0>(Arc::new(curve0));


        let key_curve1 = 1;
        let mut curve1 = FrameCurve::curve_easing(Value0(0.0), Value0(1.0), frame_count as FrameIndex, frame_count, pi_curves::easing::EEasingMode::None);
        let curve1 = crate::AssetCurve::<Value0>(Arc::new(curve1));


        let mut targets = vec![];
        // 添加 10_000 目标对象
        for i in 0..group_animation_range {
            let mut target = Target0::default(type_animation_ctx_mgr.allocat_target_id());
            targets.push(target);
        }

        // 添加 10 动画组 每组 10000 动画
        for i in 0..group_range {
            let group0 = animation_context_amount.create_animation_group();
            for j in 0..group_animation_range {
                let animation = if j % 2 == 0 {
                    // 创建属性动画
                    type_animation_ctx_mgr.value0_ctx.create_animation(Target0AnimatableAttrSet::V0 as IDAnimatableAttr, curve0.clone())
                } else {
                    // 创建属性动画
                    type_animation_ctx_mgr.value0_ctx.create_animation(Target0AnimatableAttrSet::V0a as IDAnimatableAttr, curve1.clone())
                };
                animation_context_amount.add_target_animation_notype(animation, group0, targets.get(j).unwrap().anime_target_id());
            }
            animation_context_amount.start_complete(group0, 1.0, ELoopMode::Opposite(None), 60, AnimationAmountCalc::default(), 0., EFillMode::BACKWARDS);
        }
        animation_context_amount.debug(true);

        type_animation_ctx_mgr.anime(&mut animation_context_amount, 1);

        // 测试 动画性能 计 10w 个动画计算 & 10_000 个对象的数据修改
        b.iter(move || {
            type_animation_ctx_mgr.reset();
            animation_context_amount.anime_curve_calc(1, &mut type_animation_ctx_mgr.runtime_infos);
            type_animation_ctx_mgr.value0_ctx.anime_uncheck(&mut type_animation_ctx_mgr.runtime_infos, &mut type_animation_ctx_mgr.value0_result_pool);

        });
    }
}