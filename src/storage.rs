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
    fn get(&self, EntityId) -> Option<T>;
    fn get_all(&self) -> Vec<(EntityId, T)>;
    fn add(&mut self, EntityId, T);
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

    fn get(&self, id: EntityId) -> Option<T> {
        let comp = self.storage.get(&id)?;
        Some(comp.clone())
    }

    fn get_all(&self) -> Vec<(EntityId, T)> {
        let mut all: Vec<(EntityId, T)> = vec![];
        for (k, v) in &self.storage {
            all.push((*k, v.clone()));
        }
        all
    }

    fn add(&mut self, id: EntityId, comp: T) {
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

    fn get(&self, id: EntityId) -> Option<T> {
        if id >= self.size {
            return None;
        }
        match self.storage[id as usize] {
            Some(ref c) => {
                Some(c.clone())
            }
            None => None
        }
    }

    fn get_all(&self) -> Vec<(EntityId, T)> {
        let mut all: Vec<(EntityId, T)> = vec![];
        for (id, comp) in self.storage.iter().enumerate() {
            if let Some(ref c) = *comp {
                all.push((id as EntityId, c.clone()));
            }
        }
        all
    }

    fn add(&mut self, id: EntityId, comp: T) {
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
