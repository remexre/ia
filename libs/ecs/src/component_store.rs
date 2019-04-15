use crate::{unsafe_option_vec::UnsafeOptionVec, Component, Entity};
use std::{any::TypeId, collections::HashMap};

/// A container for components.
#[derive(Debug)]
pub struct ComponentStore {
    components: HashMap<TypeId, UnsafeOptionVec>,
    next_entity: usize,
}

impl ComponentStore {
    /// Creates a new, empty ComponentStore.
    pub fn new() -> ComponentStore {
        ComponentStore {
            components: HashMap::new(),
            next_entity: 0,
        }
    }

    /// Creates a new entity.
    pub fn new_entity(&mut self) -> Entity {
        let n = self.next_entity;
        self.next_entity = self
            .next_entity
            .checked_add(1)
            .expect("too many entities allocated");
        Entity(n)
    }

    /// Gets a component for a given entity.
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|vec| unsafe { vec.get::<T>(entity.0) })
    }

    /// Sets a component for a given entity.
    pub fn set_component<T: Component>(&mut self, entity: Entity, component: T) {
        unsafe {
            self.components
                .entry(TypeId::of::<T>())
                .or_insert_with(UnsafeOptionVec::new::<T>)
                .set(entity.0, Some(component))
        }
    }

    /// Removes a component from a given entity.
    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        unsafe {
            self.components
                .entry(TypeId::of::<T>())
                .or_insert_with(UnsafeOptionVec::new::<T>)
                .set::<T>(entity.0, None)
        }
    }
}
