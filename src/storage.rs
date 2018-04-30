//!
//! Storage structures for use with Spawning Pool
//!

use std::collections::{HashMap};
use super::{EntityId};

///
/// Storage trait for component storage
///
pub trait Storage<T: Clone> {
    fn new() -> Self;
    fn get(&self, EntityId) -> Option<&T>;
    fn get_all(&self) -> Vec<(EntityId, &T)>;
    fn get_mut(&mut self, EntityId) -> Option<&mut T>;
    fn set(&mut self, EntityId, T);
    fn remove(&mut self, EntityId);
}

///
/// Hash map implementation of the storage trait, probably the best default storage to use
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashMapStorage<T: Clone> {
    storage: HashMap<EntityId, T>
}

impl<T: Clone> Storage<T> for HashMapStorage<T> {
    fn new() -> Self {
        HashMapStorage {
            storage: HashMap::new()
        }
    }

    fn get(&self, id: EntityId) -> Option<&T> {
        self.storage.get(&id)
    }

    fn get_mut(&mut self, id: EntityId) -> Option<&mut T> {
        self.storage.get_mut(&id)
    }

    fn get_all(&self) -> Vec<(EntityId, &T)> {
        let mut all = vec![];
        for (k, v) in &self.storage {
            all.push((*k, v));
        }
        all
    }

    fn set(&mut self, id: EntityId, comp: T) {
        self.storage.insert(id, comp);
    }

    fn remove(&mut self, id: EntityId) {
        self.storage.remove(&id);
    }
}

///
/// Vector implementation of the storage trait, best used for components that most entities have
/// and where fast access is important
///

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStorage<T: Clone> {
    size: u64,
    storage: Vec<Option<T>>
}

impl<T: Clone> Storage<T> for VectorStorage<T> {
    fn new() -> Self {
        VectorStorage {
            size: 100,
            storage: vec![None; 100]
        }
    }

    fn get(&self, id: EntityId) -> Option<&T> {
        if id >= self.size {
            return None;
        }
        match self.storage.get(id as usize) {
            Some(c) => c.as_ref(),
            None => None
        }
    }

    fn get_mut(&mut self, id: EntityId) -> Option<&mut T> {
        if id >= self.size {
            return None;
        }
        match self.storage.get_mut(id as usize) {
            Some(c) => c.as_mut(),
            None => None
        }
    }

    fn get_all(&self) -> Vec<(EntityId, &T)> {
        let mut all = vec![];
        for (id, comp) in self.storage.iter().enumerate() {
            if let Some(ref c) = *comp {
                all.push((id as EntityId, c));
            }
        }
        all
    }

    fn set(&mut self, id: EntityId, comp: T) {
        if id >= self.size {
            self.storage.resize((id * 2) as usize, None);
            self.size = id * 2;
        }
        self.storage[id as usize] = Some(comp);
    }

    fn remove(&mut self, id: EntityId) {
        if id < self.size {
            self.storage[id as usize] = None;
        }
    }
}
