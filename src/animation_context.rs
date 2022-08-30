use std::{ops::Index, fmt::{Debug, Display}, vec::Splice};

use pi_curves::{curve::{frame::{FrameDataValue, KeyFrameDataType, KeyFrameDataTypeAllocator, KeyFrameCurveValue, KeyFrame}, frame_curve::FrameCurve, FramePerSecond, ErrorCurve}, easing::EEasingMode};

use crate::{target_modifier::{IDAnimatableAttr, TAnimatableTargetModifier, IDAnimatableTarget, IDAnimatableTargetAllocator, TAnimatableTargetId}, animation_group::{AnimationGroup, AnimationGroupID, AnimationGroupRuntimeInfo, AnimationGroupManager}, error::EAnimationError, animation::{Animation, AnimationManager, AnimationID}, frame_curve_manager::{FrameCurveInfo, FrameCurvePool, FrameCurveInfoID, TypeFrameCurveInfoManager, FrameCurveInfoManager, TFrameCurveInfoManager, TFrameCurvePool}, runtime_info::{RuntimeInfo, RuntimeInfoMap}, target_animation::TargetAnimation, loop_mode::ELoopMode, AnimatableFloat1, animation_listener::AnimationListener, curve_frame_event::CurveFrameEvent, amount::AnimationAmountCalc};


#[derive(Debug, Clone, Copy)]
pub struct AnimeResult<T: FrameDataValue> {
    pub value: T,
    pub attr: IDAnimatableAttr,
    pub weight: f32,
}

/// 类型动画上下文 - 每种数据类型的动画实现一个
pub struct TypeAnimationContext<T: FrameDataValue> {
    ty: KeyFrameDataType,
    curves: FrameCurvePool<T>,
    result: Vec<Vec<AnimeResult<T>>>,
}

impl<T: FrameDataValue> TypeAnimationContext<T> {
    pub fn new(
        ty_allocator: &mut KeyFrameDataTypeAllocator,
        runtime_info_map: &mut RuntimeInfoMap,
        curve_infos: &mut FrameCurveInfoManager,
    ) -> Self {
        let ty = ty_allocator.alloc().unwrap();
        runtime_info_map.add_type(ty);
        curve_infos.add_type(ty);
        Self {
            ty,
            curves: FrameCurvePool::default(),
            result: vec![],
        }
    }
    /// 记录目标
    pub fn record_target(
        &mut self,
        id: IDAnimatableTarget,
    ) {
        let len = self.result.len();
        let id = id as usize + 1;
        for _ in len..id {
            self.result.push(vec![]);
        }
    }
    /// 添加 动画曲线数据
    pub fn add_frame_curve(
        &mut self,
        curve_infos: &mut FrameCurveInfoManager,
        curve: FrameCurve<T>,
    ) -> FrameCurveInfoID {
        let id = curve_infos.insert(self.ty, FrameCurvePool::curve_info(&curve));
        self.curves.insert(id, curve);

        id
    }
    /// 使用曲线计算结果 计算属性值
    pub fn anime(
        &mut self,
        runtime_infos: &RuntimeInfoMap,
    ) -> Result<(), Vec<EAnimationError>> {
        let mut errs = vec![];
        let runtime_infos = runtime_infos.list.get(self.ty).unwrap();
        for info in runtime_infos {
            let curve = self.curves.get(info.curve_id);
            match curve {
                Ok(curve) => {
                    // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
                    let value = curve.interple(info.amount_in_second);
                    let result = AnimeResult {
                        value,
                        attr: info.attr,
                        weight: info.group_weight
                    };
                    match self.result.get_mut(info.target) {
                        Some(results) => {
                            results.push(result);
                        },
                        None => {
                            errs.push(EAnimationError::TargetIDNotRecordForTypeAnimationContext);
                        },
                    }
                },
                Err(e) => errs.push(e),
            }
        }
    
        if errs.len() > 0 {
            Err(errs)
        } else {
            Ok(())
        }
    }
    
    /// 使用曲线计算结果 计算属性值
    pub fn anime_uncheck(
        &mut self,
        runtime_infos: &RuntimeInfoMap,
    ) {
        let runtime_infos = runtime_infos.list.get(self.ty).unwrap();
        for info in runtime_infos {
            let curve = self.curves.get(info.curve_id).unwrap();
            // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
            let value = curve.interple(info.amount_in_second);
            let result = AnimeResult {
                value,
                attr: info.attr,
                weight: info.group_weight
            };
            self.result.get_mut(info.target).unwrap().push(result);
        }
    }
    pub fn reset(
        &mut self,
    ) {
        self.result.iter_mut().for_each(|x| x.clear());
    }
    pub fn query_result(
        &mut self,
        target: IDAnimatableTarget,
    ) -> Vec<AnimeResult<T>> {
        self.result.get_mut(target).unwrap().splice(.., []).collect()
    }
    pub fn ty(&self) -> KeyFrameDataType {
        self.ty
    }
}

/// 动画进度计算上下文
/// * 运行所有活动动画组
/// * 管理 Target动画数据、动画组数据
/// * 提供 动画组操作接口
/// * 自身也是可动画的目标
///   * 可动画的属性为
///     * time_scale
pub struct AnimationContextAmount {
    animatable_target_id: IDAnimatableTarget,
    pub animation_mgr: AnimationManager,
    pub group_mgr: AnimationGroupManager,
    pub group_infos: Vec<AnimationGroupRuntimeInfo>,
    pub time_scale: AnimatableFloat1,
}

impl AnimationContextAmount {
    pub fn default(
        animatable_target_id: IDAnimatableTarget
    ) -> Self {
        Self {
            animatable_target_id,
            animation_mgr: AnimationManager::default(),
            group_mgr: AnimationGroupManager::default(),
            group_infos: vec![],
            time_scale: AnimatableFloat1(1.0),
        }
    }
    /// 添加 属性动画数据
    pub fn add_animation(
        &mut self,
        curve_infos: &mut FrameCurveInfoManager,
        curve_id: FrameCurveInfoID,
        attr: IDAnimatableAttr,
        ty: KeyFrameDataType,
    ) -> Result<AnimationID, EAnimationError> {
        match curve_infos.get(ty, curve_id) {
            Ok(curve_info) => {
                Ok(self.animation_mgr.create(attr, ty, curve_info, curve_id))
            },
            Err(e) => Err(e),
        }
    }
    /// 创建动画组
    pub fn create_animation_group(
        &mut self,
        target_allocator: &mut IDAnimatableTargetAllocator,
    ) -> AnimationGroupID {
        let id = self.group_mgr.create(target_allocator);
        if id >= self.group_infos.len() {
            self.group_infos.push(
                AnimationGroupRuntimeInfo { last_amount_in_second: 0., amount_in_second: 0., looped_count: 0, is_playing: false, loop_event: false, start_event: false, end_event: false }
            );
        };

        id
    }
    /// 删除动画组
    pub fn del_animation_group(
        &mut self,
        target_allocator: &mut IDAnimatableTargetAllocator,
        id: AnimationGroupID,
    ) {
        match self.group_infos.get_mut(id) {
            Some(group_info) => {
                group_info.is_playing = false;
                group_info.amount_in_second = 0.;
                group_info.last_amount_in_second = 0.;
                group_info.looped_count = 0;
                group_info.start_event = false;
                group_info.end_event = false;
                group_info.loop_event = false;
                self.group_mgr.del(target_allocator, id);
            },
            None => {},
        }
    }
    /// 为动画组添加 Target动画
    pub fn add_target_animation<R: TAnimatableTargetId>(
        &mut self,
        animation: AnimationID,
        group_id: AnimationGroupID,
        target: &R,
    ) -> Result<(), EAnimationError> {
        match self.group_mgr.get_mut(group_id) {
            Some(group) => {
                match self.animation_mgr.get(animation) {
                    Ok(animation) => {
                        group.add_target_animation(
                            TargetAnimation {
                                target: target.anime_target_id(),
                                animation,
                            }
                        )
                    },
                    Err(e) => {
                        Err(e)
                    },
                }
            },
            None => Err(EAnimationError::AnimationGroupNotFound)
        }
    }
    /// 启动动画组
    pub fn start(
        &mut self,
        id: AnimationGroupID,
        is_loop: bool,
        speed: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        from: KeyFrameCurveValue,
        to: KeyFrameCurveValue,
        frame_per_second: FramePerSecond,
        amount_calc: AnimationAmountCalc,
    ) -> Result<(), EAnimationError> {
        match  self.group_infos.get_mut(id) {
            Some(group_info) => match group_info.is_playing {
                true => Err(EAnimationError::AnimationGroupHasStarted),
                false => {
                    group_info.is_playing = true;
                    self.group_mgr.get_mut(id).unwrap().start(is_loop, speed, loop_mode, from, to, frame_per_second, group_info, amount_calc);
                    Ok(())
                },
            },
            None => Err(EAnimationError::AnimationGroupNotFound),
        }
    }

    /// 暂停动画组
    pub fn pause(
        &mut self,
        id: AnimationGroupID,
    ) -> Result<(), EAnimationError> {
        match  self.group_infos.get_mut(id) {
            Some(group_info) => match group_info.is_playing {
                true =>  {
                    group_info.is_playing = false;
                    group_info.amount_in_second = 0.;
                    group_info.last_amount_in_second = 0.;
                    group_info.looped_count = 0;
                    group_info.start_event = false;
                    group_info.end_event = false;
                    group_info.loop_event = false;
                    Ok(())
                },
                false => Err(EAnimationError::AnimationGroupNotPlaying),
            },
            None => Err(EAnimationError::AnimationGroupNotFound),
        }
    }

    /// 停止动画组
    pub fn stop(
        &mut self,
        id: AnimationGroupID,
    ) -> Result<(), EAnimationError> {
        match  self.group_infos.get_mut(id) {
            Some(group_info) => match group_info.is_playing {
                true =>  {
                    group_info.is_playing = false;
                    group_info.amount_in_second = 0.;
                    group_info.last_amount_in_second = 0.;
                    group_info.looped_count = 0;
                    group_info.start_event = false;
                    group_info.end_event = false;
                    group_info.loop_event = false;
                    match self.group_mgr.get_mut(id) {
                        Some(group) => {
                            group.stop();
                            Ok(())
                        },
                        None => Err(EAnimationError::AnimationGroupNotFound)
                    }
                },
                false => Err(EAnimationError::AnimationGroupNotPlaying),
            },
            None => Err(EAnimationError::AnimationGroupNotFound),
        }
    }

    /// 动画的曲线计算
    pub fn anime_curve_calc(
        &mut self,
        delta_ms: u64,
        runtime_infos: &mut RuntimeInfoMap,
    ) {
        let delta_ms = delta_ms as KeyFrameCurveValue * self.time_scale.0 as KeyFrameCurveValue;
        let group_mgr = &mut self.group_mgr;
        self.group_infos.iter_mut().enumerate().for_each(
            |(i, group_info)| {

                group_info.start_event = false;
                group_info.end_event = false;
                group_info.loop_event = false;
                group_info.last_amount_in_second = group_info.amount_in_second;

                if group_info.is_playing == true {
                    let group = group_mgr.get_mut(i).unwrap();
                    group.anime(runtime_infos, &self.animation_mgr, delta_ms, group_info);
                }
            }
        );
    }

    pub fn animation_event<E: Clone>(
        &self,
        listener: &mut AnimationListener<E>,
        curve_frame_event: Option<&CurveFrameEvent<E>>
    ) {
        match self.group_infos.get(listener.group) {
            Some(group_info) => {
                if group_info.start_event {
                    listener.on_start();
                }
                if group_info.end_event {
                    listener.on_end();
                }
                if group_info.loop_event {
                    listener.on_loop(group_info.looped_count);
                }
                match curve_frame_event {
                    Some(frame_event) => {
                        match frame_event.query(group_info.last_amount_in_second, group_info.amount_in_second) {
                            Some(eventdatas) => {
                                listener.on_frame(eventdatas);
                            },
                            None => {},
                        }
                    },
                    None => {},
                }
            },
            None => {
                
            },
        }
    }
}

/// AnimationContextAmount 的可动画属性的枚举
pub enum AnimationContextAmountAnimatableAttrSet {
    TimeScale = 0,
}

/// 为 AnimationContextAmount 实现 TAnimatableTargetId
impl TAnimatableTargetId for AnimationContextAmount {
    fn anime_target_id(&self) -> IDAnimatableTarget {
        self.animatable_target_id
    }
}
/// 为 AnimationContextAmount 实现 TAnimatableTargetModifier
impl TAnimatableTargetModifier<AnimatableFloat1> for AnimationContextAmount {
    fn anime_modify(&mut self, attr: IDAnimatableAttr, value: AnimatableFloat1) -> Result<(), EAnimationError> {
        if attr == AnimationContextAmountAnimatableAttrSet::TimeScale as IDAnimatableAttr {
            self.time_scale = value;
        }
        Ok(())
    }
}