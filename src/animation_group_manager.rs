use pi_slotmap::{SlotMap, DefaultKey};

use crate::{animation_group::{AnimationGroupID, AnimationGroup}, animation::AnimationInfo};


pub trait AnimationGroupManager<T: Clone> {
	fn create(&mut self) -> AnimationGroupID;
    // fn create<R: IDAnimatableTargetAllocator>(&mut self, target_allocator: &mut R) -> AnimationGroupID;
    fn del(&mut self, id: AnimationGroupID) -> Vec<AnimationInfo>;
    fn get_mut(&mut self, id: AnimationGroupID) -> Option<&mut AnimationGroup<T>>;
    fn get(&self, id: AnimationGroupID) -> Option<&AnimationGroup<T>>;
}

pub struct AnimationGroupManagerDefault<T: Clone> {
    // id_pool: SlotMap<DefaultKey, ()>,
    // counter: AnimationGroupID,
    groups: SlotMap<DefaultKey, AnimationGroup<T>>,
}
impl<T: Clone> Default for AnimationGroupManagerDefault<T> {
    fn default() -> Self {
        Self {
            // id_pool: SlotMap::default(),
            groups: SlotMap::default(),
            // counter: 0,
        }
    }
}

impl<T: Clone> AnimationGroupManager<T> for AnimationGroupManagerDefault<T> {
	#[inline]
    fn create(
        &mut self,
        // target_allocator: &mut R,
    ) -> AnimationGroupID {
		let id = self.groups.insert(AnimationGroup::new());
		self.groups[id].set_id(id);
		id
        // // let animatable_target_id = target_allocator.allocat().unwrap();
        // let id = match self.id_pool.pop() {
        //     Some(id) => {
        //         self.groups[id] = AnimationGroup::new(animatable_target_id, id);
        //         id
        //     },
        //     None => {
        //         let id = self.counter;
        //         self.counter += 1;
        //         self.groups.push(AnimationGroup::new(animatable_target_id, id));
        //         id
        //     },
        // };
        // id
    }
	#[inline]
    fn del(
        &mut self,
        // target_allocator: &mut R,
        id: AnimationGroupID,
    ) -> Vec<AnimationInfo> {
		if let Some(mut group) = self.groups.remove(id) {
            group.clear()
        } else {
            vec![]
        }
        
        // match self.groups.get_mut(id) {
        //     Some(group) => {
        //         group.stop();
        //         target_allocator.collect(group.anime_target_id());
        //         self.id_pool.push(id);
        //     },
        //     None => {},
        // }
    }
    fn get_mut(
        &mut self,
        id: AnimationGroupID,
    ) -> Option<&mut AnimationGroup<T>> {
        self.groups.get_mut(id)
    }
    fn get(
        &self,
        id: AnimationGroupID,
    ) -> Option<&AnimationGroup<T>> {
        self.groups.get(id)
    }
}