use pi_slotmap::{SlotMap, DefaultKey};

use crate::{target_modifier::{IDAnimatableTargetAllocator, TAnimatableTargetId}, animation_group::{AnimationGroupID, AnimationGroup}};


pub trait AnimationGroupManager<T: Clone> {
	fn create(&mut self) -> AnimationGroupID;
    // fn create<R: IDAnimatableTargetAllocator>(&mut self, target_allocator: &mut R) -> AnimationGroupID;
    fn del(&mut self, id: AnimationGroupID);
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
    ) {
		self.groups.remove(id);
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