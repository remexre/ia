use crate::{unsafe_option_vec::UnsafeOptionVec, Component, Entity, System};
use frunk::{hlist, Hlist};
use std::{
    any::TypeId, cell::UnsafeCell, collections::HashMap, marker::PhantomData, num::NonZeroUsize,
};

/// A container for components.
#[derive(Debug)]
pub struct ComponentStore {
    components: UnsafeCell<HashMap<TypeId, UnsafeOptionVec>>,
    next_entity: usize,
}

impl ComponentStore {
    /// Creates a new, empty ComponentStore.
    pub fn new() -> ComponentStore {
        ComponentStore {
            components: UnsafeCell::new(HashMap::new()),
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
        unsafe {
            self.components
                .get()
                .as_ref()
                .unwrap()
                .get(&TypeId::of::<T>())
                .and_then(|vec| vec.get::<T>(entity.0.get()))
        }
    }

    /// Gets a component for a given entity.
    pub fn get_mut_component<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        unsafe { self.unsafe_get_mut_component(entity) }
    }

    /// Sets a component for a given entity.
    pub fn set_component<T: Component>(&mut self, entity: Entity, component: T) {
        unsafe {
            self.components
                .get()
                .as_mut()
                .unwrap()
                .entry(TypeId::of::<T>())
                .or_insert_with(UnsafeOptionVec::new::<T>)
                .set(entity.0.get(), Some(component))
        }
    }

    /// Removes a component from a given entity.
    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        unsafe {
            self.components
                .get()
                .as_mut()
                .unwrap()
                .entry(TypeId::of::<T>())
                .or_insert_with(UnsafeOptionVec::new::<T>)
                .set::<T>(entity.0.get(), None)
        }
    }

    /// Gets a component for a given entity. This is unsafe since it makes it possible to have two
    /// mutable references to the same component if called twice with the same T.
    pub unsafe fn unsafe_get_mut_component<T: Component>(&self, entity: Entity) -> Option<&mut T> {
        self.components
            .get()
            .as_mut()
            .unwrap()
            .get_mut(&TypeId::of::<T>())
            .and_then(|vec| vec.get_mut::<T>(entity.0.get()))
    }
}

unsafe impl Send for ComponentStore {}
unsafe impl Sync for ComponentStore {}

/// A wrapper around a function to make it act as a `System`.
#[derive(Clone, Copy, Debug)]
pub struct SystemFunc<F, T> {
    func: F,
    phantom: PhantomData<fn(T)>,
}

impl<F, T> System for SystemFunc<F, T>
where
    F: for<'a> FnMut(<T as IterComponents<'a>>::Out) + Send + Sync,
    T: for<'a> IterComponents<'a>,
{
    fn run<'a>(&mut self, cs: &'a ComponentStore) {
        for e in cs.iter_entities() {
            if let Some(args) = <T as IterComponents<'a>>::extract_entity(cs, e) {
                (self.func)(args)
            }
        }
    }
}

impl<F, T> From<F> for SystemFunc<F, T> {
    fn from(func: F) -> SystemFunc<F, T> {
        SystemFunc {
            func,
            phantom: PhantomData,
        }
    }
}

/// A list of components that can be iterated over when iterating over a `ComponentStore`.
///
/// This is any `Hlist` of references to components.
pub trait IterComponents<'a> {
    type Out;

    fn extract_entity(cs: &'a ComponentStore, entity: Entity) -> Option<Self::Out>;
}

impl<'a, H: Component, T: IterComponents<'a>> IterComponents<'a> for Hlist![H, ...T] {
    type Out = Hlist![&'a H, ...T::Out];

    fn extract_entity(cs: &'a ComponentStore, entity: Entity) -> Option<Self::Out> {
        cs.get_component(entity)
            .and_then(|h| T::extract_entity(cs, entity).map(|t| hlist![h, ...t]))
    }
}

impl<'a> IterComponents<'a> for Hlist![] {
    type Out = Hlist![Entity];

    fn extract_entity(_: &'a ComponentStore, entity: Entity) -> Option<Self::Out> {
        Some(hlist![entity])
    }
}
