use std::{fmt::Debug, marker::PhantomData, sync::Arc, hash::Hash};

use pi_curves::curve::{
    frame::{FrameDataValue, KeyFrameCurveValue, KeyFrameDataType, KeyFrameDataTypeAllocator},
    frame_curve::FrameCurve,
    FramePerSecond,
};
use pi_hash::XHashMap;
use pi_slotmap::{DefaultKey, SecondaryMap};

use crate::{
    amount::AnimationAmountCalc,
    animation::{AnimationID, AnimationInfo},
    animation_group::{AnimationGroupID, AnimationGroupRuntimeInfo},
    animation_group_manager::AnimationGroupManager,
    animation_listener::{AnimationListener, EAnimationEvent},
    animation_result_pool::{TypeAnimationResultPool, AnimeResult},
    curve_frame_event::CurveFrameEvent,
    error::EAnimationError,
    frame_curve_manager::{
        FrameCurveInfoID, FrameCurveInfoManager, FrameCurvePool, TFrameCurveInfoManager,
        TFrameCurvePool,
    },
    loop_mode::ELoopMode,
    runtime_info::{RuntimeInfo, RuntimeInfoMap},
    target_animation::TargetAnimation,
    target_modifier::{
        IDAnimatableAttr, IDAnimatableTarget, IDAnimatableTargetAllocator, TAnimatableTargetId,
        TAnimatableTargetModifier,
    },
};

pub trait TTypeFrameCurve<F: FrameDataValue> {
    fn curve(&self) -> &FrameCurve<F>;
}

impl<F: FrameDataValue> TTypeFrameCurve<F> for FrameCurve<F> {
    fn curve(&self) -> &FrameCurve<F> {
        &self
    }
}

/// 类型动画上下文 - 每种数据类型的动画实现一个
pub struct TypeAnimationContext<K: Clone + Hash + PartialEq + Eq, F: FrameDataValue, D: TTypeFrameCurve<F>> {
    ty: KeyFrameDataType,
    /// 记录使用的曲线名称, 在数组中的序号即anime的ID
    curve_usage: XHashMap<K, usize>,
    curves: Vec<Option<D>>,
    id_pool: Vec<usize>,
    pd: PhantomData<F>,
}

impl<K: Clone + Hash + PartialEq + Eq, F: FrameDataValue, D: TTypeFrameCurve<F>> TypeAnimationContext<K, F, D> {
    pub fn new<T: Clone>(
        ty: usize,
        runtime_info_map: &mut RuntimeInfoMap<T>,
    ) -> Self {
        runtime_info_map.add_type(ty);
        Self {
            ty,
            curves: vec![],
            curve_usage: XHashMap::default(),
            id_pool: vec![],
            pd: PhantomData::default()
        }
    }
    pub fn curves(&self) -> &Vec<Option<D>> {
        &self.curves
    }
    /// 添加 动画曲线数据
    pub fn create_animation(
        &mut self,
        attr: IDAnimatableAttr,
        curve: D,
    ) -> AnimationInfo {
        let curve_info = FrameCurvePool::curve_info(curve.curve());
        
        if let Some(index) = self.id_pool.pop() {
            let result = AnimationInfo {
                attr,
                ty: self.ty,
                curve_info,
                curve_id: index,
            };

            self.curves[index] = Some(curve);
            
            result
        } else {
            let index = self.curves.len();
            let result = AnimationInfo {
                attr,
                ty: self.ty,
                curve_info,
                curve_id: index,
            };

            self.curves.push(Some(curve));

            result
        }

    }
    /// 使用曲线计算结果 计算属性值
    pub fn anime<T: Clone, R: TypeAnimationResultPool<F, T>>(
        &self,
        runtime_infos: &RuntimeInfoMap<T>,
        result_pool: &mut R,
    ) -> Result<(), Vec<EAnimationError>> {
        let mut errs = vec![];
        let runtime_infos = runtime_infos.list.get(self.ty).unwrap();
        // log::trace!("anime, runtime_infos len: {}", runtime_infos.len());
        // println!("anime, runtime_infos len: {}", runtime_infos.len());
        for info in runtime_infos {
            if let Some(Some(curve)) = self.curves.get(info.curve_id) {
                // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
                let value = curve.curve().interple(info.amount_in_second);
                let result = AnimeResult {
                    value,
                    attr: info.attr,
                    weight: info.group_weight,
                };
                match result_pool.record_result(info.target.clone(), info.attr, result) {
                    Ok(_) => {}
                    Err(e) => errs.push(e),
                }
            }
        }

        if errs.len() > 0 {
            // println!("Error Number {}", errs.len());
            Err(errs)
        } else {
            Ok(())
        }
    }

    /// 使用曲线计算结果 计算属性值
    pub fn anime_uncheck<T: Clone, R: TypeAnimationResultPool<F, T>>(
        &self,
        runtime_infos: &RuntimeInfoMap<T>,
        result_pool: &mut R,
    ) {
        let runtime_infos = runtime_infos.list.get(self.ty).unwrap();
        for info in runtime_infos {
            let curve = self.curves.get(info.curve_id).unwrap().as_ref().unwrap();
            // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
            let value = curve.curve().interple(info.amount_in_second);
            let result = AnimeResult {
                value,
                attr: info.attr,
                weight: info.group_weight,
            };
            result_pool.record_result(info.target.clone(), info.attr, result);
        }
    }

    pub fn ty(&self) -> KeyFrameDataType {
        self.ty
    }

    /// 移除动画对应的曲线信息
    /// * animations 为 AnimationContextAmount.del_animation_group 的返回值
    pub fn remove(
        &mut self,
        animations: & Vec<AnimationInfo>,
    ) {
        animations.iter().for_each(|anime| {
            if anime.ty == self.ty {
                self.curves[anime.curve_id] = None;
                self.id_pool.push(anime.curve_id);
            }
        });
    }

}

/// 动画进度计算上下文
/// * 运行所有活动动画组
/// * 管理 Target动画数据、动画组数据
/// * 提供 动画组操作接口
/// * 自身也是可动画的目标
///   * 可动画的属性为
///     * time_scale
pub struct AnimationContextAmount<T: Clone, M: AnimationGroupManager<T>> {
    pub group_mgr: M,
    // pub group_infos: Vec<AnimationGroupRuntimeInfo>,
    pub group_infos: SecondaryMap<DefaultKey, AnimationGroupRuntimeInfo>,
    pub time_scale: f32,
    pub group_events: Vec<(DefaultKey, EAnimationEvent, u32)>,
    mark: PhantomData<T>,
}

impl<T: Clone, M: AnimationGroupManager<T>> AnimationContextAmount<T, M> {
    pub fn default(group_mgr: M) -> Self {
        Self {
            group_mgr,
            group_infos: SecondaryMap::default(),
            time_scale: 1.0,
            group_events: vec![],
            mark: PhantomData,
        }
    }
    /// 设置是否为Debug模式, 当Banch测试性能时设置true
    pub fn debug(
        &mut self,
        flag: bool
    ) {
        for (i, _) in self.group_infos.iter_mut() {
            let group = self.group_mgr.get_mut(i).unwrap();
            group.debug = flag;
        }
    }
    /// 创建动画组
    pub fn create_animation_group(&mut self) -> AnimationGroupID {
        let id = self.group_mgr.create();
        self.group_infos.insert(
            id,
            AnimationGroupRuntimeInfo {
                last_amount_in_second: 0.,
                amount_in_second: 0.,
                looped_count: 0,
                is_playing: false,
                loop_event: false,
                start_event: false,
                end_event: false,
            },
        );
        // if id >= self.group_infos.len() {
        //     self.group_infos.push(
        //         AnimationGroupRuntimeInfo { last_amount_in_second: 0., amount_in_second: 0., looped_count: 0, is_playing: false, loop_event: false, start_event: false, end_event: false }
        //     );
        // };

        id
    }
    /// 删除动画组
    pub fn del_animation_group(&mut self, id: AnimationGroupID) -> Vec<AnimationInfo> {
        match self.group_infos.get_mut(id) {
            Some(group_info) => {
                group_info.is_playing = false;
                group_info.amount_in_second = 0.;
                group_info.last_amount_in_second = 0.;
                group_info.looped_count = 0;
                group_info.start_event = false;
                group_info.end_event = false;
                group_info.loop_event = false;
                self.group_mgr.del(id)
            }
            None => {
                vec![]
            }
        }
    }
    /// 为动画组添加 Target动画
    pub fn add_target_animation(
        &mut self,
        animation: AnimationInfo,
        group_id: AnimationGroupID,
        target: T,
    ) -> Result<(), EAnimationError> {
        match self.group_mgr.get_mut(group_id) {
            Some(group) => {
                group.add_target_animation(TargetAnimation { target, animation })
            },
            None => Err(EAnimationError::AnimationGroupNotFound),
        }
    }
    /// 启动动画组 - 完整播放,不关心动画到底设计了多少帧
    /// * `seconds` 播放时长 - 秒
    /// * `loop_mode` 循环模式
    /// * `amount_calc` 播放进度变化控制
    pub fn start_complete(
        &mut self,
        id: AnimationGroupID,
        seconds: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        frame_per_second: FramePerSecond,
        amount_calc: AnimationAmountCalc,
    ) -> Result<(), EAnimationError> {
        match self.group_infos.get_mut(id) {
            Some(group_info) => match group_info.is_playing {
                true => Err(EAnimationError::AnimationGroupHasStarted),
                false => {
                    group_info.is_playing = true;
                    self.group_mgr.get_mut(id).unwrap().start_complete(
                        seconds,
                        loop_mode,
                        frame_per_second,
                        amount_calc,
                        group_info,
                    );
                    Ok(())
                }
            },
            None => Err(EAnimationError::AnimationGroupNotFound),
        }
    }
    /// 启动动画组
    /// * `speed` 动画速度 - 正常速度为 1
    /// * `loop_mode` 循环模式
    /// * `from` 指定动画组的起始帧百分比位置 - 0~1
    /// * `to` 指定动画组的结束帧百分比位置 - 0~1
    /// * `frame_per_second` 指定动画组每秒运行多少帧 - 影响动画流畅度和计算性能
    /// * `amount_calc` 播放进度变化控制
    pub fn start_with_progress(
        &mut self,
        id: AnimationGroupID,
        speed: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        from: KeyFrameCurveValue,
        to: KeyFrameCurveValue,
        frame_per_second: FramePerSecond,
        amount_calc: AnimationAmountCalc,
    ) -> Result<(), EAnimationError> {
        match self.group_infos.get_mut(id) {
            Some(group_info) => match group_info.is_playing {
                true => Err(EAnimationError::AnimationGroupHasStarted),
                false => {
                    group_info.is_playing = true;
                    self.group_mgr.get_mut(id).unwrap().start_with_progress(
                        speed,
                        loop_mode,
                        from,
                        to,
                        frame_per_second,
                        group_info,
                        amount_calc,
                    );
                    Ok(())
                }
            },
            None => Err(EAnimationError::AnimationGroupNotFound),
        }
    }
    /// 启动动画组
    /// * `speed` 动画速度 - 正常速度为 1
    /// * `loop_mode` 循环模式
    /// * `from` 指定动画组的起始帧位置
    /// * `to` 指定动画组的结束帧位置
    /// * `frame_per_second` 指定动画组每秒运行多少帧 - 影响动画流畅度和计算性能
    /// * `amount_calc` 播放进度变化控制
    pub fn start(
        &mut self,
        id: AnimationGroupID,
        speed: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        from: KeyFrameCurveValue,
        to: KeyFrameCurveValue,
        frame_per_second: FramePerSecond,
        amount_calc: AnimationAmountCalc,
    ) -> Result<(), EAnimationError> {
        match self.group_infos.get_mut(id) {
            Some(group_info) => match group_info.is_playing {
                true => Err(EAnimationError::AnimationGroupHasStarted),
                false => {
                    group_info.is_playing = true;
                    self.group_mgr.get_mut(id).unwrap().start(
                        speed,
                        loop_mode,
                        from,
                        to,
                        frame_per_second,
                        group_info,
                        amount_calc,
                    );
                    Ok(())
                }
            },
            None => Err(EAnimationError::AnimationGroupNotFound),
        }
    }

    /// 暂停动画组
    pub fn pause(&mut self, id: AnimationGroupID) -> Result<(), EAnimationError> {
        match self.group_infos.get_mut(id) {
            Some(group_info) => match group_info.is_playing {
                true => {
                    group_info.is_playing = false;
                    group_info.amount_in_second = 0.;
                    group_info.last_amount_in_second = 0.;
                    group_info.looped_count = 0;
                    group_info.start_event = false;
                    group_info.end_event = false;
                    group_info.loop_event = false;
                    Ok(())
                }
                false => Err(EAnimationError::AnimationGroupNotPlaying),
            },
            None => Err(EAnimationError::AnimationGroupNotFound),
        }
    }

    /// 停止动画组
    pub fn stop(&mut self, id: AnimationGroupID) -> Result<(), EAnimationError> {
        match self.group_infos.get_mut(id) {
            Some(group_info) => match group_info.is_playing {
                true => {
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
                        }
                        None => Err(EAnimationError::AnimationGroupNotFound),
                    }
                }
                false => Err(EAnimationError::AnimationGroupNotPlaying),
            },
            None => Err(EAnimationError::AnimationGroupNotFound),
        }
    }

    /// 动画的曲线计算
    pub fn anime_curve_calc(&mut self, delta_ms: u64, runtime_infos: &mut RuntimeInfoMap<T>) {
        self.group_events.clear();

        let delta_ms = delta_ms as KeyFrameCurveValue * self.time_scale as KeyFrameCurveValue;
        let group_mgr = &mut self.group_mgr;
        for (i, group_info) in self.group_infos.iter_mut() {
            group_info.start_event = false;
            group_info.end_event = false;
            group_info.loop_event = false;
            group_info.last_amount_in_second = group_info.amount_in_second;

            if group_info.is_playing == true {
                let group = group_mgr.get_mut(i).unwrap();
                group.anime(runtime_infos, delta_ms, group_info);
            }

            if group_info.start_event {
                self.group_events.push((i, EAnimationEvent::Start, 0));
            }
            if group_info.end_event {
                self.group_events.push((i, EAnimationEvent::End, 0));
            }
            if group_info.loop_event {
                self.group_events.push((i, EAnimationEvent::Loop, group_info.looped_count as u32));
            }
        }
        // self.group_infos.iter_mut().enumerate().for_each(
        //     |(i, group_info)| {

        //         group_info.start_event = false;
        //         group_info.end_event = false;
        //         group_info.loop_event = false;
        //         group_info.last_amount_in_second = group_info.amount_in_second;

        //         if group_info.is_playing == true {
        //             let group = group_mgr.get_mut(i).unwrap();
        //             group.anime(runtime_infos, delta_ms, group_info);
        //         }
        //     }
        // );
    }

    pub fn animation_event<E: Clone>(
        &self,
        listener: &mut AnimationListener<E>,
        curve_frame_event: Option<&CurveFrameEvent<E>>,
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
                        match frame_event.query(
                            group_info.last_amount_in_second,
                            group_info.amount_in_second,
                        ) {
                            Some(eventdatas) => {
                                listener.on_frame(eventdatas);
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}

/// AnimationContextAmount 的可动画属性的枚举
pub enum AnimationContextAmountAnimatableAttrSet {
    TimeScale = 0,
}

/// 为 AnimationContextAmount 实现 TAnimatableTargetId
// impl<A: AnimationManager, T, M: AnimationGroupManager<T>> TAnimatableTargetId<T> for AnimationContextAmount<A, T, M> {
//     fn anime_target_id(&self) -> T {
//         self.animatable_target_id
//     }
// }
/// 为 AnimationContextAmount 实现 TAnimatableTargetModifier
impl<T: Clone, M: AnimationGroupManager<T>> TAnimatableTargetModifier<f32>
    for AnimationContextAmount<T, M>
{
    fn anime_modify(&mut self, attr: IDAnimatableAttr, value: f32) -> Result<(), EAnimationError> {
        if attr == AnimationContextAmountAnimatableAttrSet::TimeScale as IDAnimatableAttr {
            self.time_scale = value;
        }
        Ok(())
    }
}
