use crate::{unsafe_option_vec::UnsafeOptionVec, Component, Entity};
use std::{any::TypeId, collections::HashMap, num::NonZeroUsize};

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
            next_entity: 1,
        }
    }

    /// Returns an iterator over all entities.
    pub fn iter_entities(&self) -> impl Clone + Iterator<Item = Entity> {
        (1..self.next_entity).map(|n| {
            NonZeroUsize::new(n)
                .map(Entity)
                .expect("impossible case? entity 0")
        })
    }

    /// Creates a new entity.
    pub fn new_entity(&mut self) -> Entity {
        let n = self.next_entity;
        self.next_entity = self
            .next_entity
            .checked_add(1)
            .expect("too many entities allocated");
        NonZeroUsize::new(n)
            .map(Entity)
            .expect("impossible case? entity 0")
    }

    /// Gets a component for a given entity.
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|vec| unsafe { vec.get::<T>(entity.0.get()) })
    }

    /// Gets a component for a given entity.
    pub fn get_mut_component<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|vec| unsafe { vec.get_mut::<T>(entity.0.get()) })
    }

    /// Sets a component for a given entity.
    pub fn set_component<T: Component>(&mut self, entity: Entity, component: T) {
        unsafe {
            self.components
                .entry(TypeId::of::<T>())
                .or_insert_with(UnsafeOptionVec::new::<T>)
                .set(entity.0.get(), Some(component))
        }
    }

    /// Removes a component from a given entity.
    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        unsafe {
            self.components
                .entry(TypeId::of::<T>())
                .or_insert_with(UnsafeOptionVec::new::<T>)
                .set::<T>(entity.0.get(), None)
        }
    }
}
