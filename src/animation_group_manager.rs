use crate::{target_modifier::{IDAnimatableTargetAllocator, TAnimatableTargetId}, animation_group::{AnimationGroupID, AnimationGroup}};


pub trait AnimationGroupManager {
    fn create<R: IDAnimatableTargetAllocator>(&mut self, target_allocator: &mut R) -> AnimationGroupID;
    fn del<R: IDAnimatableTargetAllocator>(&mut self, target_allocator: &mut R, id: AnimationGroupID,);
    fn get_mut(&mut self, id: AnimationGroupID) -> Option<&mut AnimationGroup>;
    fn get(&self, id: AnimationGroupID) -> Option<&AnimationGroup>;
}

pub struct AnimationGroupManagerDefault {
    id_pool: Vec<AnimationGroupID>,
    counter: AnimationGroupID,
    groups: Vec<AnimationGroup>,
}
impl Default for AnimationGroupManagerDefault {
    fn default() -> Self {
        Self {
            id_pool: vec![],
            groups: vec![],
            counter: 0,
        }
    }
}

impl AnimationGroupManager for AnimationGroupManagerDefault {
    fn create<R: IDAnimatableTargetAllocator>(
        &mut self,
        target_allocator: &mut R,
    ) -> AnimationGroupID {
        let animatable_target_id = target_allocator.allocat().unwrap();
        let id = match self.id_pool.pop() {
            Some(id) => {
                self.groups[id] = AnimationGroup::new(animatable_target_id, id);
                id
            },
            None => {
                let id = self.counter;
                self.counter += 1;
                self.groups.push(AnimationGroup::new(animatable_target_id, id));
                id
            },
        };
        id
    }
    fn del<R: IDAnimatableTargetAllocator>(
        &mut self,
        target_allocator: &mut R,
        id: AnimationGroupID,
    ) {
        match self.groups.get_mut(id) {
            Some(group) => {
                group.stop();
                target_allocator.collect(group.anime_target_id());
                self.id_pool.push(id);
            },
            None => {},
        }
    }
    fn get_mut(
        &mut self,
        id: AnimationGroupID,
    ) -> Option<&mut AnimationGroup> {
        self.groups.get_mut(id)
    }
    fn get(
        &self,
        id: AnimationGroupID,
    ) -> Option<&AnimationGroup> {
        self.groups.get(id)
    }
}