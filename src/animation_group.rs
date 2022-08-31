use pi_curves::{curve::{frame::KeyFrameCurveValue, FramePerSecond, FrameIndex}};

use crate::{error::EAnimationError, loop_mode::{ELoopMode, get_amount_calc}, target_modifier::{IDAnimatableTarget, TAnimatableTargetId, TAnimatableTargetModifier, IDAnimatableAttr}, runtime_info::{RuntimeInfo, RuntimeInfoMap}, target_animation::TargetAnimation, amount::AnimationAmountCalc};

pub type AnimationGroupID = usize;

#[derive(Debug, Clone, Copy)]
pub struct AnimationGroupRuntimeInfo {
    /// 在 秒 级比例下的进度
    pub last_amount_in_second: KeyFrameCurveValue,
    /// 在 秒 级比例下的进度
    pub amount_in_second: KeyFrameCurveValue,
    /// 循环次数
    pub looped_count: u16,
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
pub struct AnimationGroup {
    animatable_target_id: IDAnimatableTarget,
    id: AnimationGroupID,
    animations: Vec<TargetAnimation>,
    is_loop: bool,
    /// 动画组速度
    pub speed: KeyFrameCurveValue,
    from: KeyFrameCurveValue,
    to: KeyFrameCurveValue,
    /// 动画组有效运行时间
    delay_time: KeyFrameCurveValue,
    /// 动画组循环记录
    looped_count: u16,
    /// 动画组循环模式
    loop_mode: ELoopMode,
    /// 每秒运行多少帧 - 速度为 1 的情况下
    frame_per_second: FramePerSecond,
    /// 动画组使用的动画集合中最大帧数
    max_frame: FrameIndex,
    /// 动画组运行一次的时间 - ms
    once_time: KeyFrameCurveValue,
    is_playing: bool,
    /// 动画组的混合权重
    blend_weight: f32,
    /// 动画组的在秒单位下的进度
    amount_in_second: KeyFrameCurveValue,
    amount: fn(KeyFrameCurveValue, KeyFrameCurveValue) -> (KeyFrameCurveValue, u16),
    amount_calc: AnimationAmountCalc,
}

impl AnimationGroup {
    pub fn new(animatable_target_id: IDAnimatableTarget, id: AnimationGroupID) -> Self {
        Self {
            animatable_target_id,
            id,
            animations: vec![],
            is_loop: false,
            speed: 1.,
            from: 0.,
            to: 1.,
            delay_time: 0.,
            looped_count: 0,
            loop_mode: ELoopMode::Not,
            frame_per_second: 30,
            max_frame: 0,
            once_time: 1.,
            is_playing: false,
            blend_weight: 1.0,
            amount_in_second: 0.,
            
            amount: get_amount_calc(ELoopMode::Not),
            amount_calc: AnimationAmountCalc::default(),
        }
    }

    /// 动画组运行接口
    pub fn anime(
        &mut self,
        runtime_infos: &mut RuntimeInfoMap,
        delta_ms: KeyFrameCurveValue,
        group_info: &mut AnimationGroupRuntimeInfo,
    ) {
        group_info.last_amount_in_second = self.amount_in_second;


        // println!(">>>>>>{}", self.is_playing);
        if self.is_playing {
            if self.delay_time.abs() < 0.00001 {
                group_info.start_event = true;
            }

            let amount_call = &self.amount;

            // println!("{}, {}", self.once_time, self.delay_time);

            let (amount, loop_count) = amount_call(self.once_time, self.delay_time);

            let anime_amount = self.amount_calc.calc(amount);
            let amount_in_second = anime_amount + self.from / self.frame_per_second as KeyFrameCurveValue;
    
            if self.is_loop {
                group_info.loop_event = self.looped_count != loop_count;
                group_info.looped_count = loop_count;
                self.looped_count = loop_count;
            } else {
                if amount >= 1. {
                    group_info.end_event = true;
                    self.is_playing = false;
                }
            }
    
            self.looped_count = loop_count;
            self.amount_in_second = amount_in_second;

            group_info.amount_in_second = amount_in_second;
    
            self.update_to_infos(runtime_infos);

            self.delay_time += delta_ms * self.speed;
        }
    }
    /// 添加 目标动画
    pub fn add_target_animation(
        &mut self,
        target_animation: TargetAnimation,
    ) -> Result<(), EAnimationError> {
        // println!("{}", self.max_frame);
        self.max_frame = FramePerSecond::max(self.max_frame, target_animation.animation.get_max_frame_for_running_speed(self.frame_per_second));
        // println!("add_target_animation {}", self.max_frame);
        self.animations.push(target_animation);
        Ok(())
    }
    /// 启动动画组
    pub fn start(
        &mut self,
        is_loop: bool,
        speed: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        from: KeyFrameCurveValue,
        to: KeyFrameCurveValue,
        frame_per_second: FramePerSecond,
        group_info: &mut AnimationGroupRuntimeInfo,
        amount_calc: AnimationAmountCalc,
    ) {
        self.is_loop = is_loop;
        self.is_playing = true;
        self.delay_time = 0.;
        self.speed = speed;

        let (from, to) = (KeyFrameCurveValue::min(from, to), KeyFrameCurveValue::max(from, to));
        // println!("from {}, to {}", from, to);

        self.frame_per_second(frame_per_second);
        self.loop_mode(loop_mode);
        self.from(from);
        self.to(to);

        self.once_time();

        self.amount_calc = amount_calc;

        match loop_mode {
            ELoopMode::Opposite => {
                self.amount_in_second = to / self.frame_per_second as KeyFrameCurveValue;
            },
            ELoopMode::OppositePly => {
                self.amount_in_second = to / self.frame_per_second as KeyFrameCurveValue;
            },
            _ => {
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
        self.once_time = (self.to - self.from) as KeyFrameCurveValue / self.frame_per_second as KeyFrameCurveValue / self.speed.abs() * 1000.0;
    }
    fn from(
        &mut self,
        from: KeyFrameCurveValue,
    ) {
        self.from = KeyFrameCurveValue::max(0 as KeyFrameCurveValue, from as KeyFrameCurveValue);
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
        if self.frame_per_second == 0 {
            Err(EAnimationError::AnimationFramePerSecondCannotZero)
        } else {
            let max_frame = self.max_frame as f32 * (frame_per_second as f32 / self.frame_per_second as f32);
    
            self.frame_per_second = frame_per_second;
            self.max_frame = max_frame as FrameIndex;

            Ok(())
        }
    }
    fn update_to_infos(
        &self,
        runtime_infos: &mut RuntimeInfoMap,
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
                target: anime.target,
                attr: anime.animation.attr(),
                curve_id: anime.animation.curve_id(),
                group_weight: self.blend_weight,
            };
            // println!("{:?}", temp);
            runtime_infos.insert(anime.animation.ty(), temp);
        }
    }
}

/// AnimationGroup 的可动画属性的枚举
pub enum AnimationGroupAnimatableAttrSet {
    BlendWeight = 0,
}

/// 为 AnimationGroup 实现 TAnimatableTargetId
impl TAnimatableTargetId for AnimationGroup {
    fn anime_target_id(&self) -> IDAnimatableTarget {
        self.animatable_target_id
    }
}
/// 为 AnimationGroup 实现 TAnimatableTargetModifier
impl TAnimatableTargetModifier<f32> for AnimationGroup {
    fn anime_modify(&mut self, attr: IDAnimatableAttr, value: f32) -> Result<(), EAnimationError> {
        if attr == AnimationGroupAnimatableAttrSet::BlendWeight as IDAnimatableAttr {
            self.blend_weight = value;
        }
        Ok(())
    }
}
