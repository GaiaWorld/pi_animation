use std::{mem::replace, hash::Hash};

use pi_curves::curve::frame::{KeyFrameCurveValue, KeyFrameDataType};
use pi_hash::XHashMap;

use crate::{
    error::EAnimationError,
    frame_curve_manager::FrameCurveInfoID,
    target_modifier::{IDAnimatableAttr},
};

/// 一个动画的运行时数据
#[derive(Debug, Clone, Copy)]
pub struct RuntimeInfo {
    // pub group_info: AnimationGroupRuntimeInfo,
    /// 所属动画组的权重
    pub group_weight: f32,
    /// 动画进度
    pub amount_in_second: KeyFrameCurveValue,
    /// 作用的 目标对象 的目标属性 的ID
    pub attr: IDAnimatableAttr,
    /// 在曲线对应的数据类型 曲线信息管理器中 该动画使用的曲线 的 ID
    pub curve_id: FrameCurveInfoID,
    // pub anime: TargetAnimation,
}
// impl<T: Clone + PartialEq + Eq + PartialOrd + Ord> PartialEq for RuntimeInfo<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.target == other.target
//     }
// }
// impl<T: Clone + PartialEq + Eq + PartialOrd + Ord> Eq for RuntimeInfo<T> {
//     fn assert_receiver_is_total_eq(&self) {}
// }
// impl<T: Clone + PartialEq + Eq + PartialOrd + Ord> Ord for RuntimeInfo<T> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.target.cmp(&other.target)
//     }
// }
// impl<T: Clone + PartialEq + Eq + PartialOrd + Ord> PartialOrd for RuntimeInfo<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.target.partial_cmp(&other.target)
//     }
// }

/// 运行时信息表 - 唯一
/// 每个 Vec<RuntimeInfo> 分别对应一个 动画数据类型
pub struct RuntimeInfoMap<T: Clone + PartialEq + Eq + Hash> {
    /// 保证按 动画作用目标排序
    pub list: Vec<XHashMap<T, Vec<RuntimeInfo>>>,
}

impl<T: Clone + PartialEq + Eq + Hash> RuntimeInfoMap<T> {
    pub fn default() -> Self {
        Self { list: vec![], }
    }
    /// 仅在分配 KeyAnimeDataType 后立即调用
    pub fn add_type(&mut self, ty: KeyFrameDataType) {
        if ty >= self.list.len() {
            for _ in self.list.len()..ty + 1 {
                self.list.push(XHashMap::default());
            }
        }
    }
    pub fn get_type_list(&self, ty: KeyFrameDataType) -> Option<&XHashMap<T, Vec<RuntimeInfo>>> {
        self.list.get(ty)
    }
    /// 动画运行时记录
    pub fn insert(
        &mut self,
        ty: KeyFrameDataType,
        target: T,
        info: RuntimeInfo,
    ) -> Result<(), EAnimationError> {
        match self.list.get_mut(ty) {
            Some(map) => {
                let list = if let Some(list) = map.get_mut(&target) {
                    list
                } else {
                    map.insert(target.clone(), vec![]);
                    map.get_mut(&target).unwrap()
                };
                // match list.binary_search(&info.target) {
                //     Ok(idx) => {
                //         list.insert(idx, info.target.clone());
                //         self.list.get_mut(ty).unwrap().insert(idx, info);
                //     },
                //     Err(idx) => {
                //         list.insert(idx, info.target.clone());
                //         self.list.get_mut(ty).unwrap().insert(idx, info);
                //     },
                // }
                list.push(info);
                Ok(())
            }
            None => {
                Err(EAnimationError::RuntimeInfoMapNotFindType)
            },
        }
    }
    pub fn reset(&mut self) {
        self.list.iter_mut().for_each(|x| x.clear());
    }
}
