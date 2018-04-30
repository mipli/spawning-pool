//!
//! Spawning Pool
//!
//! A library for creating generic entities and attaching data components to them.
//!
//! Kind of like an Entity Component System, but without the system part.
//!
//! Components needs to implement `Clone`, `Debug`, `Serialize` and `Deserialize`
//! 
//! # Examples
//! ```
//! # #[macro_use] extern crate serde_derive;
//! #[macro_use] extern crate spawning_pool;
//! # fn main() {
//!
//! use spawning_pool::EntityId;
//! use spawning_pool::storage::{Storage, VectorStorage};
//!
//! #[derive(Clone, Debug, Serialize, Deserialize)]
//! struct Pos {
//!     x: i32,
//!     y: i32
//! }
//!
//! create_spawning_pool!(
//!     (Pos, pos, VectorStorage)
//! );
//! let mut pool = SpawningPool::new();
//! let entity = pool.spawn_entity();
//! pool.set(entity, Pos{x: 4, y: 2});
//! # }
//! ```
//!

#[macro_use] extern crate serde_derive;

pub mod storage;

/// Entity ID
pub type EntityId = u64;

#[macro_export]
macro_rules! create_spawning_pool {
    ($((
        // component type
        $component:ty,
        // internal storage container name
        $store_name: ident,
        // storage type, implements storage::Storage trait
        $storage: ident
        )), +)
        => (
            use std::collections::HashSet;
            #[derive(Debug, Serialize, Deserialize)]
            pub struct SpawningPool {
                next_id: u64,
                removed: HashSet<EntityId>,
            $(
                $store_name: $storage<$component>,
            )+
            }

            impl SpawningPool {
                #[allow(dead_code)]
                pub fn new() -> Self {
                    SpawningPool{
                        next_id: 1,
                        removed: Default::default(),
                        $(
                            $store_name: $storage::new(),
                        )+
                    }
                }

                #[allow(dead_code)]
                pub fn cleanup_removed(&mut self) {
                    for id in &self.removed {
                        $(
                            self.$store_name.remove(*id);
                        )+
                    }
                    self.removed.clear();
                }

                #[allow(dead_code)]
                pub fn spawn_entity(&mut self) -> EntityId {
                    let id = self.next_id;
                    self.next_id += 1;
                    id
                }

                #[allow(dead_code)]
                pub fn remove_entity(&mut self, id: EntityId) {
                    self.removed.insert(id);
                }

                #[allow(dead_code)]
                pub fn set<T>(&mut self, id: EntityId, component: T) where Self: ComponentLoader<T> {
                    if self.removed.get(&id).is_none() {
                        self.set_overloaded(id, component);
                    }
                }

                #[allow(dead_code)]
                pub fn get<T>(&self, id: EntityId) -> Option<&T> where Self: ComponentLoader<T> {
                    if self.removed.get(&id).is_none() {
                        self.get_overloaded(id)
                    } else {
                        None
                    }
                }

                #[allow(dead_code)]
                pub fn force_get<T>(&self, id: EntityId) -> Option<&T> where Self: ComponentLoader<T> {
                    self.get_overloaded(id)
                }

                #[allow(dead_code)]
                pub fn get_mut<T>(&mut self, id: EntityId) -> Option<&mut T> where Self: ComponentLoader<T> {
                    if self.removed.get(&id).is_none() {
                        self.get_mut_overloaded(id)
                    } else {
                        None
                    }
                }

                #[allow(dead_code)]
                pub fn remove<T>(&mut self, id: EntityId) where Self: ComponentLoader<T> {
                    if self.removed.get(&id).is_none() {
                        self.remove_overloaded(id);
                    }
                }

                #[allow(dead_code)]
                pub fn get_all<T>(&self) -> Vec<(EntityId, &T)> where Self: ComponentLoader<T> {
                    let ids = self.get_all_overloaded();
                    ids.iter()
                        .filter(|(id, _)| self.removed.get(id).is_none())
                        .map(|i| *i)
                        .collect()
                }
            }

            pub trait ComponentLoader<T> {
                fn get_overloaded(&self, id: EntityId) -> Option<&T>;
                fn get_all_overloaded(&self) -> Vec<(EntityId, &T)>;
                fn get_mut_overloaded(&mut self, id: EntityId) -> Option<&mut T>;
                fn set_overloaded(&mut self, id: EntityId, component: T);
                fn remove_overloaded(&mut self, id: EntityId);
            }

            $(
            impl ComponentLoader<$component> for SpawningPool {
                fn get_overloaded(&self, id: EntityId) -> Option<&$component> {
                    self.$store_name.get(id)
                }
                fn get_all_overloaded(&self) -> Vec<(EntityId, &$component)> {
                    self.$store_name.get_all()
                }
                fn get_mut_overloaded(&mut self, id: EntityId) -> Option<&mut $component> {
                    self.$store_name.get_mut(id)
                }
                fn set_overloaded(&mut self, id: EntityId, component: $component) {
                    self.$store_name.set(id, component);
                }
                fn remove_overloaded(&mut self, id: EntityId) {
                    self.$store_name.remove(id);
                }
            }
            )+
    )
}

#[cfg(test)]
mod tests {
    use super::{EntityId};
    use storage::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct Position {
        pub x: i32,
        pub y: i32
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct Velocity {
        pub x: i32,
        pub y: i32
    }


    #[test]
    fn create_entity() {
        create_spawning_pool!(
            (Position, pos, HashMapStorage)
        );
        let mut pool = SpawningPool::new();
        assert_eq!(pool.spawn_entity(), 1u64);
        assert_eq!(pool.spawn_entity(), 2u64);
    }

    #[test]
    fn test_set() {
        create_spawning_pool!(
            (Position, pos, HashMapStorage),
            (Velocity, vel, HashMapStorage)
        );
        let mut pool = SpawningPool::new();
        let id = pool.spawn_entity();
        assert!(pool.get::<Position>(id).is_none());

        pool.set(id, Velocity{x: 1, y: 2});

        match pool.get::<Velocity>(id) {
            Some(vel) => {
                assert_eq!(vel.x, 1);
                assert_eq!(vel.y, 2);
            }
            None => assert!(false)
        }

        assert_eq!(pool.get_all::<Velocity>().len(), 1);
    }

    #[test]
    fn test_remove_entity() {
        create_spawning_pool!(
            (Position, pos, HashMapStorage),
            (Velocity, vel, HashMapStorage)
        );
        let mut pool = SpawningPool::new();
        let id = pool.spawn_entity();

        pool.set(id, Velocity{x: 1, y: 2});

        match pool.get::<Velocity>(id) {
            Some(vel) => {
                assert_eq!(vel.x, 1);
                assert_eq!(vel.y, 2);
            }
            None => assert!(false)
        }

        pool.remove_entity(id);

        assert!(pool.get::<Velocity>(id).is_none());
    }

    #[test]
    fn test_force_get() {
        create_spawning_pool!(
            (Position, pos, HashMapStorage),
            (Velocity, vel, HashMapStorage)
        );
        let mut pool = SpawningPool::new();
        let id = pool.spawn_entity();

        pool.set(id, Velocity{x: 1, y: 2});

        match pool.get::<Velocity>(id) {
            Some(vel) => {
                assert_eq!(vel.x, 1);
                assert_eq!(vel.y, 2);
            }
            None => assert!(false)
        }

        pool.remove_entity(id);

        assert!(pool.get::<Velocity>(id).is_none());
        assert!(pool.force_get::<Velocity>(id).is_some());
        pool.cleanup_removed();
        assert!(pool.force_get::<Velocity>(id).is_none());
    }

    #[test]
    fn test_get_mut() {
        create_spawning_pool!(
            (Position, pos, HashMapStorage),
            (Velocity, vel, HashMapStorage)
        );
        let mut pool = SpawningPool::new();
        let id = pool.spawn_entity();
        assert!(pool.get::<Position>(id).is_none());

        pool.set(id, Velocity{x: 1, y: 2});

        match pool.get::<Velocity>(id) {
            Some(vel) => {
                assert_eq!(vel.x, 1);
                assert_eq!(vel.y, 2);
            }
            None => assert!(false)
        }

        match pool.get_mut::<Velocity>(id) {
            Some(vel) => {
                vel.x = 3;
                vel.y = 4;
            }
            None => assert!(false)
        }

        match pool.get::<Velocity>(id) {
            Some(vel) => {
                assert_eq!(vel.x, 3);
                assert_eq!(vel.y, 4);
            }
            None => assert!(false)
        }
    }

    #[test]
    fn test_remove() {
        create_spawning_pool!(
            (Position, pos, HashMapStorage),
            (Velocity, vel, HashMapStorage)
        );
        let mut pool = SpawningPool::new();
        let id = pool.spawn_entity();
        assert!(pool.get::<Position>(id).is_none());

        pool.set(id, Velocity{x: 1, y: 2});

        match pool.get::<Velocity>(id) {
            Some(vel) => {
                assert_eq!(vel.x, 1);
                assert_eq!(vel.y, 2);
            }
            None => assert!(false)
        }

        pool.remove::<Velocity>(id);

       assert!( pool.get::<Velocity>(id).is_none());
    }

    #[test]
    fn test_get_mut_vector_storage() {
        create_spawning_pool!(
            (Position, pos, VectorStorage),
            (Velocity, vel, VectorStorage)
        );
        let mut pool = SpawningPool::new();
        let id = pool.spawn_entity();
        assert!(pool.get::<Position>(id).is_none());

        pool.set(id, Velocity{x: 1, y: 2});

        match pool.get::<Velocity>(id) {
            Some(vel) => {
                assert_eq!(vel.x, 1);
                assert_eq!(vel.y, 2);
            }
            None => assert!(false)
        }

        match pool.get_mut::<Velocity>(id) {
            Some(vel) => {
                vel.x = 3;
                vel.y = 4;
            }
            None => assert!(false)
        }

        match pool.get::<Velocity>(id) {
            Some(vel) => {
                assert_eq!(vel.x, 3);
                assert_eq!(vel.y, 4);
            }
            None => assert!(false)
        }
    }
}
