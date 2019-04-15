use crate::{Component, Entity};
use safety_guard::safety;
use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    any::TypeId,
    collections::HashMap,
    fmt::{Debug, Formatter, Result as FmtResult},
    ptr::{drop_in_place, write, NonNull},
};

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

/// A very unsafe vector whose elements are all `Option<T>`'s, where `T` is chosen with each `get`
/// and `set` operation.
///
/// TODO: This needs to be audited.
struct UnsafeOptionVec {
    /// If `len != 0`, the address at which the allocated `Option<T>`'s start.
    ptr: NonNull<u8>,
    /// The number of allocated `Option<T>`'s.
    len: usize,
    /// The layout of each `Option<T>`.
    layout: Layout,
    /// The destructor for `Option<T>`.
    dtor: unsafe fn(NonNull<u8>),
}

impl UnsafeOptionVec {
    /// Creates a new, empty `UnsafeOptionVec`.
    pub fn new<T: 'static>() -> UnsafeOptionVec {
        unsafe fn dtor<T>(ptr: NonNull<u8>) {
            drop_in_place::<T>(ptr.cast().as_ptr())
        }

        UnsafeOptionVec {
            ptr: NonNull::dangling(),
            len: 0,
            layout: Layout::new::<Option<T>>(),
            dtor: dtor::<Option<T>>,
        }
    }

    /// Grows the vector to (at least) the given size.
    fn grow_to<T: 'static>(&mut self, mut n: usize) {
        let old_len = self.len;
        if n < old_len {
            return;
        }

        if let Some(new_n) = n.checked_next_power_of_two() {
            n = new_n;
        }

        let new_layout = self.layout_with_len(n);
        let ptr = if old_len == 0 {
            unsafe { alloc(new_layout) }
        } else {
            unsafe { realloc(self.ptr.as_ptr(), self.layout(), new_layout.size()) }
        };

        match NonNull::new(ptr) {
            Some(ptr) => {
                self.ptr = ptr;
                self.len = n;
            }
            None => {
                // `self` is safe to drop. Either `alloc` failed, and `self.len` is therefore `0`,
                // so no `dealloc` will occur, or `realloc` failed, and `self.ptr` is still
                // `dealloc`atable.
                panic!("allocation failure in component store")
            }
        }

        for i in old_len..n {
            unsafe { write(self.ptr(i).cast::<Option<T>>().as_ptr(), None) };
        }
    }

    /// Returns the current layout of the vector.
    fn layout(&self) -> Layout {
        // This is a good indication we've forgotten to check for an empty vector.
        debug_assert_ne!(self.len, 0);

        self.layout_with_len(self.len)
    }

    /// Returns the layout the vector would have if it were of length `n`.
    fn layout_with_len(&self, n: usize) -> Layout {
        // All the code here is copied from `libcore/alloc.rs.html`, but is unstable there. This
        // will just be
        //
        // ```rust
        // self.layout
        //     .repeat(n)
        //     .map(|(l, _)| l)
        //     .expect("overflow of size of component store")
        // ```
        //
        // once that method is stable.

        let align = self.layout.align();
        let size = self.layout.size();
        let size_rounded_up = size.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
        let alloc_size = size
            .checked_add(size_rounded_up.wrapping_sub(size))
            .and_then(|padded_size| padded_size.checked_mul(n));

        if let Some(alloc_size) = alloc_size {
            unsafe { Layout::from_size_align_unchecked(alloc_size, align) }
        } else {
            panic!("overflow of size of component store")
        }
    }

    /// Returns a pointer to the `n`th item of the vector.
    #[safety(assert(n < self.len), "`n` must be less than the allocated length of the vector")]
    unsafe fn ptr(&self, n: usize) -> NonNull<u8> {
        let size = self.layout_with_len(n).size();
        NonNull::new_unchecked(self.ptr.as_ptr().add(size))
    }

    /// Reads the `n`th value from the `UnsafeOptionVec`. This will return `None` if the given
    /// index is out of bounds.
    #[safety(eq(self.layout, Layout::new::<Option<T>>()),
        "T must have the same layout as the type that was given to `UnsafeOptionVec::new`")]
    #[safety("T must be the same type as was given to `UnsafeOptionVec::new`")]
    pub unsafe fn get<T: 'static>(&self, n: usize) -> Option<&T> {
        if n > self.len {
            return None;
        }
        let ptr = self.ptr(n).cast::<Option<T>>().as_ptr();
        (&*ptr).as_ref()
    }

    /// Sets the `n`th value from the `UnsafeOptionVec` to `Some` value. This will extend the
    /// underlying allocation if `n` is out of bounds.
    #[safety(eq(self.layout, Layout::new::<Option<T>>()),
        "T must have the same layout as the type that was given to `UnsafeOptionVec::new`")]
    #[safety("T must be the same type as was given to `UnsafeOptionVec::new`")]
    pub unsafe fn set<T: 'static>(&mut self, n: usize, value: Option<T>) {
        self.grow_to::<T>(
            n.checked_add(1)
                .expect("overflow of size of component store"),
        );
        *self.ptr(n).cast().as_mut() = value;
    }
}

impl Debug for UnsafeOptionVec {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("UnsafeOptionVec")
            .field("len", &self.len)
            .finish()
    }
}

impl Drop for UnsafeOptionVec {
    fn drop(&mut self) {
        if self.len != 0 {
            unsafe {
                for i in 0..self.len {
                    (self.dtor)(self.ptr(i))
                }
                dealloc(self.ptr.as_ptr(), self.layout())
            }
        }
    }
}
