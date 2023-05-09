use bevy::prelude::*; 
//use crate::ship::*;

pub struct CollisionPair(Entity, Entity);

/* impl CollisionPair {
    fn find_pair(&self, entity1: Entity, entity2: Entity) -> bool {
        if self.0 == entity1 || self.0 == entity2 {
            if self.1 == entity1 || self.1 == entity2 {
                return true;
            }
        }
        return false;
    }
}

#[derive(Resource)]
pub struct Detections {
    pairs: Vec<CollisionPair>
}

impl Default for Detections {
    fn default() -> Self {
        Self{pairs: vec![]}
    }
}

impl Detections {
    pub fn add(&mut self, entity1: Entity, entity2: Entity) {
        let pair = CollisionPair{0: entity1, 1: entity2};
        self.pairs.append(&mut vec![pair]);
    }
    pub fn del(&mut self, entity1: Entity, entity2: Entity) {
        for pair in self.pairs.iter_mut() {
            if pair.find_pair(entity1, entity2) {
                self.pairs.
            }
        }
    }
} */