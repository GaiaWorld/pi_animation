use std::mem::replace;

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
    pub on_start: Vec<OnStart>,
    pub on_end: Vec<OnEnd>,
    pub on_loop: Vec<OnLoop>,
    pub on_frame_event: Vec<OnFrameEvent<D>>,
}

impl<D: Clone> AnimationListener<D> {
    pub fn new(group: AnimationGroupID) -> Self {
        Self {
            group,
            on_start: vec![],
            on_end: vec![],
            on_loop: vec![],
            on_frame_event: vec![],
        }
    }
    pub fn on_start(
        &mut self,
    ) {
        let mut temp = replace(&mut self.on_start, vec![]);
        temp.drain(..).for_each(|call| {
            match call() {
                Ok(result) => match result {
                    EAnimationEventResult::None => {
                        self.on_start.push(call)
                    },
                    EAnimationEventResult::RemoveListen => {
                    },
                },
                Err(_) => {},
            }
        });
        // match &self.on_start {
        //     Some(call) => match call() {
        //         Ok(result) => match result {
        //             EAnimationEventResult::None => {},
        //             EAnimationEventResult::RemoveListen => { self.on_start = None; },
        //         },
        //         Err(_) => {},
        //     },
        //     None => {},
        // }
    }
    pub fn on_end(
        &mut self,
    ) {
        let mut temp = replace(&mut self.on_end, vec![]);
        temp.drain(..).for_each(|call| {
            match call() {
                Ok(result) => match result {
                    EAnimationEventResult::None => {
                        self.on_end.push(call)
                    },
                    EAnimationEventResult::RemoveListen => {
                    },
                },
                Err(_) => {},
            }
        });
    }
    pub fn on_loop(
        &mut self,
        loop_count: u32,
    ) {
        let mut temp = replace(&mut self.on_loop, vec![]);
        temp.drain(..).for_each(|call| {
            match call(loop_count) {
                Ok(result) => match result {
                    EAnimationEventResult::None => {
                        self.on_loop.push(call)
                    },
                    EAnimationEventResult::RemoveListen => {
                    },
                },
                Err(_) => {},
            }
        });
    }
    pub fn on_frame(
        &mut self,
        frame_datas: Vec<D>,
    ) {
        let mut temp = replace(&mut self.on_frame_event, vec![]);
        temp.drain(..).for_each(|call| {
            match call(frame_datas.clone()) {
                Ok(result) => match result {
                    EAnimationEventResult::None => {
                        self.on_frame_event.push(call)
                    },
                    EAnimationEventResult::RemoveListen => {
                    },
                },
                Err(_) => {},
            }
        });
    }
}