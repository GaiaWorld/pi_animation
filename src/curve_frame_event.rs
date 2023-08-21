use pi_curves::curve::{FrameIndex, frame::KeyFrameCurveValue};

pub type FrameEventData = u16;

pub struct CurveFrameEvent<D: Clone> {
    total_frame: KeyFrameCurveValue,
    events: Vec<FrameIndex>,
    datas: Vec<D>,
}

impl<D: Clone> CurveFrameEvent<D> {
    pub fn new(total_frame: KeyFrameCurveValue) -> Self {
        Self {
            total_frame,
            events: vec![],
            datas: vec![],
        }
    }

    pub fn add(
        &mut self,
        frame: FrameIndex,
        data: D,
    ) {
        match self.events.binary_search(&frame) {
            Ok(index) => {
                self.events.insert(index, frame);
                self.datas.insert(index, data);
            },
            Err(index) => {
                self.events.insert(index, frame);
                self.datas.insert(index, data);
            },
        }
    }

    pub fn query(
        &self,
        amount_last: KeyFrameCurveValue,
        amount: KeyFrameCurveValue,
    ) -> Option<Vec<D>> {
        if amount_last != amount {
            let last = (amount_last * self.total_frame as KeyFrameCurveValue) as FrameIndex;
            let curr = (amount * self.total_frame as KeyFrameCurveValue) as FrameIndex;
            let last_index = match self.events.binary_search(&last) {
                Ok(index) => index,
                Err(index) => index,
            };
            let curr_index = match self.events.binary_search(&curr) {
                Ok(index) => index,
                Err(index) => index,
            };

            let mut result: Vec<D> = vec![];
            for i in last_index..curr_index {
                match self.datas.get(i) {
                    Some(data) => {
                        result.push(data.clone());
                    },
                    None => {},
                }
            }

            if result.len() > 0 {
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }
}