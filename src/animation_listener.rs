use crate::{animation_group::AnimationGroupID, error::EAnimationError};

#[derive(Debug, Clone, Copy)]
pub enum EAnimationEventResult {
    None,
    RemoveListen,
}

pub type OnStart = Box<dyn Fn() -> Result<EAnimationEventResult, EAnimationError>>;
pub type OnEnd = Box<dyn Fn() -> Result<EAnimationEventResult, EAnimationError>>;
pub type OnLoop = Box<dyn Fn(u32) -> Result<EAnimationEventResult, EAnimationError>>;
pub type OnFrameEvent<D> = Box<dyn Fn(Vec<D>) -> Result<EAnimationEventResult, EAnimationError>>;

#[derive(Debug, Clone, Copy)]
pub enum EAnimationEvent {
    None,
    Start,
    End,
    Loop,
    FrameEvent,
}

pub struct AnimationListener<D: Clone> {
    pub group: AnimationGroupID,
    pub on_start: Option<OnStart>,
    pub on_end: Option<OnEnd>,
    pub on_loop: Option<OnLoop>,
    pub on_frame_event: Option<OnFrameEvent<D>>,
}

impl<D: Clone> AnimationListener<D> {
    pub fn on_start(
        &mut self,
    ) {
        match &self.on_start {
            Some(call) => match call() {
                Ok(result) => match result {
                    EAnimationEventResult::None => {},
                    EAnimationEventResult::RemoveListen => { self.on_start = None; },
                },
                Err(_) => {},
            },
            None => {},
        }
    }
    pub fn on_end(
        &mut self,
    ) {
        match &self.on_end {
            Some(call) => match call() {
                Ok(result) => match result {
                    EAnimationEventResult::None => {},
                    EAnimationEventResult::RemoveListen => { self.on_end = None; },
                },
                Err(_) => {},
            },
            None => {},
        }
    }
    pub fn on_loop(
        &mut self,
        loop_count: u32,
    ) {
        match &self.on_loop {
            Some(call) => match call(loop_count) {
                Ok(result) => match result {
                    EAnimationEventResult::None => {},
                    EAnimationEventResult::RemoveListen => { self.on_loop = None; },
                },
                Err(_) => {},
            },
            None => {},
        }
    }
    pub fn on_frame(
        &mut self,
        frame_datas: Vec<D>,
    ) {
        match &self.on_frame_event {
            Some(call) => {
                match call(frame_datas) {
                    Ok(result) => match result {
                        EAnimationEventResult::None => {},
                        EAnimationEventResult::RemoveListen => { self.on_frame_event = None; },
                    },
                    Err(_) => {},
                }
            },
            None => {},
        }
    }
}