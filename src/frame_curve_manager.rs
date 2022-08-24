use std::sync::Arc;

use pi_curves::curve::{FrameIndex, frame::{FrameDataValue, KeyFrameDataType}, frame_curve::FrameCurve, FramePerSecond};
use pi_hash::XHashMap;

use crate::error::EAnimationError;

pub type FrameCurveInfoID = usize;


/// 针对某种数据类型对应的帧曲线描述信息管理器
/// * 每个曲线的 ID 一旦分配便不会变化
/// * 每个曲线的 数据是不可变的
/// * 通过 add 接口获得 ID
/// * 通过 del 接口删除 曲线
pub struct TypeFrameCurveInfoManager {
    id_pool: Vec<FrameCurveInfoID>,
    counter: FrameCurveInfoID,
    curve_infos: Vec<FrameCurveInfo>,
}

impl TypeFrameCurveInfoManager {
    pub fn default() -> Self {
        Self { id_pool: vec![], counter: 0, curve_infos: vec![] }
    }
    pub fn add(
        &mut self,
        curve: FrameCurveInfo,
    ) -> FrameCurveInfoID {
        let id = match self.id_pool.pop() {
            Some(id) => {
                let info = self.curve_infos.get_mut(id).unwrap();
                info.max_frame = curve.max_frame;
                info.min_frame = curve.min_frame;
                info.design_frame_per_second = curve.design_frame_per_second;

                id
            },
            None => {
                let id = self.counter;
                self.counter += 1;

                self.curve_infos.push(curve);
                id
            },
        };

        id
    }
    pub fn del(
        &mut self,
        id: FrameCurveInfoID,
    ) -> Result<(), EAnimationError> {
        if id < self.counter {
            // 回收 ID
            if !self.id_pool.contains(&id) {
                self.id_pool.push(id);
            }
            Ok(())
        } else {
            Err(EAnimationError::FrameCurveNotFound)
        }
    }
    pub fn get(
        &self,
        id: FrameCurveInfoID,
    ) -> Result<FrameCurveInfo, EAnimationError> {
        match self.curve_infos.get(id) {
            Some(v) => Ok(*v),
            None => Err(EAnimationError::FrameCurveNotFound),
        }
    }
}

/// 针对各种数据类型对应的帧曲线描述信息管理器
/// * 第一层序号对应 数据类型 分配到的 KeyFrameDataType
pub struct  FrameCurveInfoManager {
    list: Vec<TypeFrameCurveInfoManager>,
}

impl FrameCurveInfoManager {
    pub fn default() -> Self {
        Self {
            list: vec![],
        }
    }
    pub fn add_type(
        &mut self,
        ty: KeyFrameDataType
    ) -> Result<(), EAnimationError> {
        if ty == self.list.len() {
            self.list.push(TypeFrameCurveInfoManager::default());
        }
        Ok(())
    }
    pub fn add(
        &mut self,
        ty: KeyFrameDataType,
        curve: FrameCurveInfo,
    ) -> FrameCurveInfoID {
        self.list.get_mut(ty).unwrap().add(curve)
    }
    pub fn del(
        &mut self,
        ty: KeyFrameDataType,
        id: FrameCurveInfoID,
    ) -> Result<(), EAnimationError> {
        match self.list.get_mut(ty) {
            Some(mgr) => {
                mgr.del(id)
            },
            None => {
                Ok(())
            },
        }
    }
    pub fn get(
        &self,
        ty: KeyFrameDataType,
        id: FrameCurveInfoID,
    ) -> Result<FrameCurveInfo, EAnimationError> {
        match self.list.get(ty) {
            Some(mgr) => {
                mgr.get(id)
            },
            None => {
                Err(EAnimationError::FrameCurveNotFound)
            },
        }
    }
}
/// 关键帧曲线描述信息
#[derive(Debug, Clone, Copy)]
pub struct FrameCurveInfo {
    max_frame: FrameIndex,
    min_frame: FrameIndex,
    design_frame_per_second: FramePerSecond,
}

impl FrameCurveInfo {
    pub fn new(
        max_frame: FrameIndex,
        min_frame: FrameIndex,
        design_frame_per_second: FramePerSecond,
    ) -> Self {
        Self {
            max_frame,
            min_frame,
            design_frame_per_second,
        }
    }
    pub fn get_max_frame_for_running_speed(&self, running_frame_per_second: FramePerSecond) -> FramePerSecond {
        (self.max_frame as f32 / self.design_frame_per_second as f32 * running_frame_per_second as f32) as FramePerSecond
    }
    pub fn max_frame(
        &self,
    ) -> FrameIndex {
        self.max_frame
    }
    pub fn min_frame(
        &self,
    ) -> FrameIndex {
        self.min_frame
    }
    pub fn design_frame_per_second(
        &self,
    ) -> FrameIndex {
        self.design_frame_per_second
    }
}

/// 对应动画数据类型的一个曲线池
/// 存放序号由 FrameCurveInfoManager 分配获得
pub struct FrameCurvePool<T: FrameDataValue> {
    arcs: Vec<Arc<FrameCurve<T>>>,
    infos: Vec<FrameCurveInfoID>,
    // arcs: XHashMap<FrameCurveInfoID, Arc<FrameCurve<T>>>,
}

impl<T: FrameDataValue> FrameCurvePool<T> {
    pub fn curve_info(curve: &FrameCurve<T>) -> FrameCurveInfo {
        FrameCurveInfo {
            max_frame: curve.max_frame,
            min_frame: curve.min_frame,
            design_frame_per_second: curve.design_frame_per_second,
        }
    }
    pub fn default() -> Self {
        Self {
            arcs: vec![],
            infos: vec![],
            // arcs: XHashMap::default(),
        }
    }
    pub fn add(
        &mut self,
        id: FrameCurveInfoID,
        curve: FrameCurve<T>,
    ) {
        let arc = Arc::new(curve);

        self.infos.push(id);
        self.arcs.push(arc);

        // self.arcs.insert(id, arc);
    }
    pub fn del(
        &mut self,
        id: FrameCurveInfoID,
    ) -> Result<(), EAnimationError> {
        match self.infos.binary_search(&id) {
            Ok(index) => {
                self.infos.swap_remove(index);
                self.arcs.swap_remove(index);
                Ok(())
            },
            Err(_) => Err(EAnimationError::FrameCurveNotFound),
        }

        // self.arcs.remove(&id);
        // Ok(())
    }
    pub fn get(
        &self,
        id: FrameCurveInfoID,
    ) -> Result<Arc<FrameCurve<T>>, EAnimationError> {
        match self.infos.binary_search(&id) {
            Ok(index) => {
                Ok(self.arcs.get(index).unwrap().clone())
            },
            Err(_) => Err(EAnimationError::FrameCurveNotFound),
        }
        // match self.arcs.get(&id) {
        //     Some(v) => {
        //         Ok(v.clone())
        //     },
        //     None => Err(EAnimationError::FrameCurveNotFound),
        // }
    }
}