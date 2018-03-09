//!
//! Spawning Pool
//!
//! A library for creating generic entities and attaching data components to them.
//!
//! Kind of like an Entity Component System, but without the system part.
//!
//! Components needs to implement `Clone`, `Serialize` and `Deserialize`
//! 
//! # Examples
//! ```
//! # #[macro_use] extern crate serde_derive;
//! #[macro_use] extern crate spawning_pool;
//! # fn main() {
//!
//! use spawning_pool::EntityId;
//! use spawning_pool::storage::{Storage,VectorStorage};
//!
//! #[derive(Clone, Serialize, Deserialize)]
//! struct Pos {
//!     x: i32,
//!     y: i32
//! }
//!
//! create_spawning_pool!(
//!     (Pos, VectorStorage, pos, get_pos, get_pos_all, add_pos, remove_pos)
//! );
//! let mut pool = SpawningPool::new();
//! let entity = pool.spawn_entity();
//! pool.add_pos(entity, Pos{x: 4, y: 2});
//! # }
//! ```
//!

#[macro_use] extern crate serde_derive;

/// Entity ID
pub type EntityId = u64;

pub mod storage;

/// Create a spawning pool with the given components
#[macro_export]
macro_rules! create_spawning_pool {
    ($((
        // Component type
        $comp:ty,
        // Storage type to use for this component
        $store: ident,
        // Internal of storage
        $store_name: ident,
        // Name of get function, returns Option<$comp>
        $get:ident,
        // Name of get_all function, returns Vec<$comp>
        $get_all: ident,
        // Add component to an entity
        $add: ident,
        // Remove component from entity
        $remove: ident)), +)
    => (
        #[derive(Clone, Serialize, Deserialize)]
        pub struct SpawningPool {
            next_id: u64,
        $(
            $store_name: $store<$comp>,
        )+
        }

        impl SpawningPool {
            #[allow(dead_code)]
            pub fn new() -> Self {
                SpawningPool{
                    next_id: 1,
                    $(
                        $store_name: $store::new(),
                    )+
                }
            }

            #[allow(dead_code)]
            pub fn spawn_entity(&mut self) -> EntityId {
                let id = self.next_id;
                self.next_id += 1;
                id
            }

            #[allow(dead_code)]
            pub fn remove_entity(&mut self, i: EntityId) {
                $(
                    self.$store_name.remove(i);
                )+
            }

        $(
            #[allow(dead_code)]
            pub fn $get(&self, i: EntityId) -> Option<$comp> {
                self.$store_name.get(i)
            }

            #[allow(dead_code)]
            pub fn $get_all(&self) -> Vec<(EntityId, $comp)> {
                self.$store_name.get_all()
            }

            #[allow(dead_code)]
            pub fn $add(&mut self, i: EntityId, comp: $comp) {
                self.$store_name.add(i, comp);
            }

            #[allow(dead_code)]
            pub fn $remove(&mut self, i: EntityId) {
                self.$store_name.remove(i);
            }
        )+
        }
    )
}

#[cfg(test)]
mod tests {
    use storage::*;
    use super::{EntityId};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Pos {
        pub x: i32,
        pub y: i32
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Glyph {
        pub glyph: char
    }

    #[test]
    fn entity_id_increment() {
        create_spawning_pool!(
            (Pos, VectorStorage, pos, get_pos, get_pos_all, add_pos, remove_pos)
        );
        let mut pool = SpawningPool::new();

        assert_eq!(pool.next_id, 1);
        let entity = pool.spawn_entity();
        assert_eq!(pool.next_id, 2);
        assert_eq!(entity, 1);
    }

    #[test]
    fn single_components() {
        create_spawning_pool!(
            (Pos, VectorStorage, pos, get_pos, get_pos_all, add_pos, remove_pos)
        );
        let mut pool = SpawningPool::new();

        let _ = pool.spawn_entity();
        let entity = pool.spawn_entity();

        let positions = pool.get_pos_all();
        assert_eq!(positions.len(), 0);

        pool.add_pos(entity, Pos{x: 0, y: 1});

        let positions = pool.get_pos_all();
        assert_eq!(positions.len(), 1);

        match pool.get_pos(entity) {
            Some(pos) => {
                assert_eq!(pos.x, 0);
                assert_eq!(pos.y, 1);
            },
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn multiple_components() {
        create_spawning_pool!(
            (Pos, VectorStorage, pos, get_pos, get_pos_all, add_pos, remove_pos),
            (Glyph, HashMapStorage, glyph, get_glyph, get_glyph_all, add_glyph, remove_glyph)
        );
        let mut pool = SpawningPool::new();

        let a = pool.spawn_entity();
        let b = pool.spawn_entity();

        let positions = pool.get_pos_all();
        let glyphs = pool.get_glyph_all();
        assert_eq!(positions.len(), 0);
        assert_eq!(glyphs.len(), 0);

        pool.add_pos(a, Pos{x: 0, y: 1});
        pool.add_pos(b, Pos{x: 2, y: 3});
        pool.add_glyph(b, Glyph{glyph: 'b'});

        let positions = pool.get_pos_all();
        let glyphs = pool.get_glyph_all();
        assert_eq!(positions.len(), 2);
        assert_eq!(glyphs.len(), 1);

        match pool.get_pos(a) {
            Some(pos) => {
                assert_eq!(pos.x, 0);
                assert_eq!(pos.y, 1);
            },
            None => {
                assert!(false);
            }
        }

        match pool.get_pos(b) {
            Some(pos) => {
                assert_eq!(pos.x, 2);
                assert_eq!(pos.y, 3);
            },
            None => {
                assert!(false);
            }
        }

        match pool.get_glyph(a) {
            Some(_) => {
                assert!(false);
            },
            None => { }
        }

        match pool.get_glyph(b) {
            Some(glyph) => {
                assert_eq!(glyph.glyph, 'b');
            },
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn remove_entity() {
        create_spawning_pool!(
            (Pos, VectorStorage, pos, get_pos, get_pos_all, add_pos, remove_pos)
        );
        let mut pool = SpawningPool::new();

        let _ = pool.spawn_entity();
        let entity = pool.spawn_entity();

        pool.add_pos(entity, Pos{x: 0, y: 1});

        assert!(pool.get_pos(entity).is_some());

        pool.remove_entity(entity);
        assert!(!pool.get_pos(entity).is_some());
    }
}
