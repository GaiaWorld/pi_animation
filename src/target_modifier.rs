use pi_curves::curve::frame::FrameDataValue;

use crate::{error::EAnimationError};

/// 一个可动画的对象的ID - [0-usize::MAX]
/// * 即同时最多支持 usize::MAX 个目标对象的动画
pub type IDAnimatableTarget = usize;
/// 一种类型的可动画的对象的属性枚举范围 [0-255]
/// * 即每个对象上最多支持256个属性的动画
pub type IDAnimatableAttr = u8;

/// 可动画目标对象ID分配器
/// * 对每个目标而言应当是唯一的
pub trait IDAnimatableTargetAllocator {
    /// 分配 ID
    fn allocat(&mut self) -> Result<IDAnimatableTarget, EAnimationError>;
    /// 回收 ID
    fn collect(&mut self, id: IDAnimatableTarget,);
}

/// 动画目标ID 分配器
pub struct IDAnimatableTargetAllocatorDefault {
    id_pool: Vec<IDAnimatableTarget>,
    counter: IDAnimatableTarget,
}
impl Default for IDAnimatableTargetAllocatorDefault {
    fn default() -> Self {
        Self {
            id_pool: vec![],
            counter: 0
        }
    }
}
impl IDAnimatableTargetAllocator for IDAnimatableTargetAllocatorDefault {
    fn allocat(&mut self) -> Result<IDAnimatableTarget, EAnimationError> {
        match self.id_pool.pop() {
            Some(id) => {
                Ok(id)
            },
            None => {
                if self.counter == IDAnimatableTarget::MAX {
                    Err(EAnimationError::KeyTargetCannotAllocMore)
                } else {
                    let id = self.counter;
                    self.counter += 1;
                    Ok(id)
                }
            }
        }
    }
    fn collect(
        &mut self,
        id: IDAnimatableTarget,
    ) {
        self.id_pool.push(id);
    }
}

// /// 动画目标属性分配器
// pub struct KeyTargetAttrAllocator {
//     counter: IDAnimatableAttr,
// }

// impl KeyTargetAttrAllocator {
//     pub fn default() -> Self {
//         Self {
//             counter: 0
//         }
//     }
//     pub fn alloc(&mut self) -> Result<IDAnimatableAttr, EAnimationError> {
//         if self.counter == IDAnimatableAttr::MAX {
//             Err(EAnimationError::KeyTargetAttrCannotAllocMore)
//         } else {
//             let id = self.counter;
//             self.counter += 1;
//             Ok(id)
//         }
//     }
// }

/// 可应用动画的结果的目标特征
pub trait TAnimatableTargetModifier<T: FrameDataValue> {
    fn anime_modify(&mut self, attr: IDAnimatableAttr, value: T) -> Result<(), EAnimationError>;
}
/// 可进行动画的目标ID特征
pub trait TAnimatableTargetId<T> {
    fn anime_target_id(&self) -> T;
}