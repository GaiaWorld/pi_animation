use pi_curves::{curve::{frame::KeyFrameCurveValue, FramePerSecond, FrameIndex}};
use pi_slotmap::{DefaultKey, Key};

use log::trace;

use crate::{error::EAnimationError, loop_mode::{ELoopMode, get_amount_calc}, target_modifier::{IDAnimatableTarget, TAnimatableTargetId, TAnimatableTargetModifier, IDAnimatableAttr}, runtime_info::{RuntimeInfo, RuntimeInfoMap}, target_animation::TargetAnimation, amount::AnimationAmountCalc, end_mode::EEndMode};

pub type AnimationGroupID = DefaultKey;

#[derive(Debug, Clone, Copy)]
pub struct AnimationGroupRuntimeInfo {
    /// 在 秒 级比例下的进度
    pub last_amount_in_second: KeyFrameCurveValue,
    /// 在 秒 级比例下的进度
    pub amount_in_second: KeyFrameCurveValue,
    /// 循环次数
    pub looped_count: u32,
    /// 是否活动状态
    pub is_playing: bool,
    /// 是否触发 loop 事件
    pub loop_event: bool,
    /// 是否触发 start 事件
    pub start_event: bool,
    /// 是否触发 end 事件
    pub end_event: bool,
}

/// 动画组数据结构
/// * 计算动画进度
/// * 更新动画进度到内部的各个动画
/// * 响应动画事件
pub struct AnimationGroup<T: Clone> {
    // animatable_target_id: T,
    id: AnimationGroupID,
    animations: Vec<TargetAnimation<T>>,
    loop_count: Option<u32>,
    /// 动画组速度
    pub speed: KeyFrameCurveValue,
    pub end_mode: EEndMode,
    from: KeyFrameCurveValue,
    to: KeyFrameCurveValue,
    /// 动画组有效运行时间
    delay_time: KeyFrameCurveValue,
    /// 动画组循环记录
    looped_count: u32,
    /// 动画组循环模式
    loop_mode: ELoopMode,
    /// 设计每秒帧数据分辨率 - 速度为 1 的情况下
    frame_per_second: FramePerSecond,
    /// 累计未运行动画的有效间隔时间
    detal_ms_record: KeyFrameCurveValue,
    /// 动画组运行的帧间隔时长
    frame_ms: KeyFrameCurveValue,
    /// 动画组使用的动画集合中最大帧数
    max_frame: KeyFrameCurveValue,
    /// 动画组运行一次的时间 - ms
    once_time: KeyFrameCurveValue,
    is_playing: bool,
    /// 动画组的混合权重
    blend_weight: f32,
    /// 动画组的在秒单位下的进度
    amount_in_second: KeyFrameCurveValue,
    amount: fn(KeyFrameCurveValue, KeyFrameCurveValue) -> (KeyFrameCurveValue, u32),
    amount_calc: AnimationAmountCalc,
    /// 是否为测试模式
    pub debug: bool,
}

impl<T: Clone> AnimationGroup<T> {
    pub fn new() -> Self {
        Self {
            // animatable_target_id,
            id: AnimationGroupID::null(),
            animations: vec![],
            loop_count: Some(1),
            speed: 1.,
            from: 0.,
            to: 1.,
            delay_time: 0.,
            looped_count: 0,
            loop_mode: ELoopMode::Not,
            frame_per_second: 60,
            frame_ms: 16.6,
            detal_ms_record: 0.,
            max_frame: 0.,
            once_time: 1.,
            is_playing: false,
            blend_weight: 1.0,
            amount_in_second: 0.,
            end_mode: EEndMode::KeepEnd,
            amount: get_amount_calc(ELoopMode::Not),
            amount_calc: AnimationAmountCalc::default(),
            debug: false,
        }
    }

    /// 获取动画组整体的最大帧位置 - 可 用于 start 接口 from to 参数的参考
    pub fn max_frame(&self) -> KeyFrameCurveValue {
        self.max_frame
    }

	/// 设置id
	pub fn set_id(&mut self, id: AnimationGroupID) {
		self.id = id;
	}

    /// 动画组运行接口
    /// * `delta_ms` 帧推的间隔时间
    pub fn anime(
        &mut self,
        runtime_infos: &mut RuntimeInfoMap<T>,
        delta_ms: KeyFrameCurveValue,
        group_info: &mut AnimationGroupRuntimeInfo,
    ) {
        group_info.last_amount_in_second = self.amount_in_second;

        if self.is_playing {
            if self.delay_time.abs() < 0.00001 {
                group_info.start_event = true;
            }

            self.detal_ms_record += delta_ms;
            log::trace!(">>>>>>>>>>>>>>>> detal_ms_record {}, frame_ms {}", self.detal_ms_record, self.frame_ms);

            if group_info.start_event || self.detal_ms_record >= self.frame_ms || self.debug {
                let amount_call = &self.amount;
    
                let (mut amount, loop_count) = amount_call(self.once_time, self.delay_time);

                if self.looped_count != loop_count {
                    match self.loop_count {
                        Some(count) => {
                            if count <= loop_count {
                                group_info.end_event = true;
                                self.is_playing = false;

                                amount = match self.end_mode {
                                    EEndMode::KeepEnd => match self.loop_mode {
                                        ELoopMode::Not => 1.,
                                        ELoopMode::Positive(_) => 1.,
                                        ELoopMode::Opposite(_) => 0.,
                                        ELoopMode::PositivePly(_) => 0.,
                                        ELoopMode::OppositePly(_) => 1.,
                                    },
                                    EEndMode::BackToStart => match self.loop_mode {
                                        ELoopMode::Not => 0.,
                                        ELoopMode::Positive(_) => 0.,
                                        ELoopMode::Opposite(_) => 1.,
                                        ELoopMode::PositivePly(_) => 1.,
                                        ELoopMode::OppositePly(_) => 0.,
                                    },
                                }
                            } else {
                                group_info.loop_event = true;
                            }
                        },
                        None => {
                            group_info.loop_event = true;
                        },
                    }
                }
    
                let anime_amount = self.amount_calc.calc(amount);
                let amount_in_second = anime_amount + self.from / self.frame_per_second as KeyFrameCurveValue;
    
                log::trace!("once_time {}, delay_time {}, amount {}, anime_amount {}, amount_in_second {}", self.once_time, self.delay_time, amount, anime_amount, amount_in_second);
    
                self.looped_count = loop_count;
                self.amount_in_second = amount_in_second;
    
                group_info.amount_in_second = amount_in_second;
                group_info.looped_count = loop_count;

                self.update_to_infos(runtime_infos);

                self.delay_time += self.detal_ms_record * self.speed;
                self.detal_ms_record = 0.;
            }
        }
    }
    /// 添加 目标动画
    pub fn add_target_animation(
        &mut self,
        target_animation: TargetAnimation<T>,
    ) -> Result<(), EAnimationError> {
        // println!("{}", self.max_frame);
        self.max_frame = KeyFrameCurveValue::max(self.max_frame, target_animation.animation.get_max_frame_for_running_speed(self.frame_per_second));
        // println!("add_target_animation {}", self.max_frame);
        self.animations.push(target_animation);
        Ok(())
    }
    /// 启动动画组 - 完整播放,不关心动画到底设计了多少帧
    /// * `seconds` 播放时长 - 秒
    /// * `loop_mode` 循环模式
    /// * `amount_calc` 播放进度变化控制
    pub fn start_complete(
        &mut self,
        seconds: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        frame_per_second: FramePerSecond,
        amount_calc: AnimationAmountCalc,
        group_info: &mut AnimationGroupRuntimeInfo,
    ) {
        let speed = 1.0 / seconds;
        let from = 0.;
        let to = self.max_frame as KeyFrameCurveValue;
        self.start(speed, loop_mode, from, to, frame_per_second, group_info, amount_calc)
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
        speed: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        from: KeyFrameCurveValue,
        to: KeyFrameCurveValue,
        frame_per_second: FramePerSecond,
        group_info: &mut AnimationGroupRuntimeInfo,
        amount_calc: AnimationAmountCalc,
    ) {
        self.start(speed, loop_mode, from * self.max_frame, to * self.max_frame, frame_per_second, group_info, amount_calc)
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
        speed: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        from: KeyFrameCurveValue,
        to: KeyFrameCurveValue,
        frame_per_second: FramePerSecond,
        group_info: &mut AnimationGroupRuntimeInfo,
        amount_calc: AnimationAmountCalc,
    ) {
        self.is_playing = true;
        self.speed = speed;
        self.delay_time = 0.;
        self.looped_count = 0;
        self.detal_ms_record = 0.;
        self.amount_in_second = 0.;

        let (from, to) = (KeyFrameCurveValue::min(from, to), KeyFrameCurveValue::max(from, to));
        // println!("from {}, to {}", from, to);

        self.frame_per_second(frame_per_second);
        self.loop_mode(loop_mode);
        self.from(from);
        self.to(to);

        self.once_time();

        self.amount_calc = amount_calc;

        match loop_mode {
            ELoopMode::Opposite(v) => {
                self.loop_count = v;
                self.amount_in_second = to / self.frame_per_second as KeyFrameCurveValue;
            },
            ELoopMode::OppositePly(v) => {
                self.loop_count = v;
                self.amount_in_second = to / self.frame_per_second as KeyFrameCurveValue;
            },
            ELoopMode::Positive(v) => {
                self.loop_count = v;
                self.amount_in_second = from / self.frame_per_second as KeyFrameCurveValue;
            },
            ELoopMode::PositivePly(v) => {
                self.loop_count = v;
                self.amount_in_second = from / self.frame_per_second as KeyFrameCurveValue;
            },
            ELoopMode::Not => {
                self.loop_count = Some(1);
                self.amount_in_second = from / self.frame_per_second as KeyFrameCurveValue;
            },
        }

        group_info.is_playing = true;
        group_info.amount_in_second = self.amount_in_second;
        group_info.last_amount_in_second = self.amount_in_second;
        group_info.start_event = false;
        group_info.loop_event = false;
        group_info.end_event = false;
        group_info.looped_count = 0;
    }
    /// 启停止动画组
    pub fn stop(
        &mut self,
    ) {
        self.is_playing = false;
    }
    fn loop_mode(
        &mut self,
        mode: ELoopMode,
    ) {
        self.loop_mode = mode;
        self.amount = get_amount_calc(mode);
    }
    fn once_time(
        &mut self,
    ) {
        // println!("self.from {}, self.to {}", self.from, self.to);
        self.once_time = (self.to - self.from) as KeyFrameCurveValue / self.frame_per_second as KeyFrameCurveValue * 1000.0;
    }
    fn from(
        &mut self,
        from: KeyFrameCurveValue,
    ) {
        self.from = KeyFrameCurveValue::max(0., from as KeyFrameCurveValue);
    }
    fn to(
        &mut self,
        to: KeyFrameCurveValue,
    ) {
        self.to = KeyFrameCurveValue::min(self.max_frame as KeyFrameCurveValue, to);
    }

    fn frame_per_second(
        &mut self,
        frame_per_second: FramePerSecond,
    ) -> Result<(), EAnimationError> {
        if frame_per_second == 0 {
            Err(EAnimationError::AnimationFramePerSecondCannotZero)
        } else {
            self.frame_ms = 1000. / frame_per_second as KeyFrameCurveValue;

            Ok(())
        }
    }
    fn update_to_infos(
        &self,
        runtime_infos: &mut RuntimeInfoMap<T>,
    ) {
        for anime in self.animations.iter() {
            let temp = RuntimeInfo {
                // group_info: AnimationGroupRuntimeInfo {
                //     amount_in_second: self.amount_in_second,
                //     loop_count: self.loop_count,
                //     is_loop: self.is_loop,
                //     is_playing: self.is_playing,
                // },
                amount_in_second: self.amount_in_second,
                // anime: *anime,
                target: anime.target.clone(),
                attr: anime.animation.attr(),
                curve_id: anime.animation.curve_id(),
                group_weight: self.blend_weight,
            };
            runtime_infos.insert(anime.animation.ty(), temp);
        }
    }
}

/// AnimationGroup 的可动画属性的枚举
pub enum AnimationGroupAnimatableAttrSet {
    BlendWeight = 0,
}

/// 为 AnimationGroup 实现 TAnimatableTargetId
// impl<T> TAnimatableTargetId<T> for AnimationGroup<T> {
//     fn anime_target_id(&self) -> T {
//         self.animatable_target_id
//     }
// }
/// 为 AnimationGroup 实现 TAnimatableTargetModifier
impl<T: Clone> TAnimatableTargetModifier<f32> for AnimationGroup<T> {
    fn anime_modify(&mut self, attr: IDAnimatableAttr, value: f32) -> Result<(), EAnimationError> {
        if attr == AnimationGroupAnimatableAttrSet::BlendWeight as IDAnimatableAttr {
            self.blend_weight = value;
        }
        Ok(())
    }
}
