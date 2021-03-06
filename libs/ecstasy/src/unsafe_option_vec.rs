use safety_guard::safety;
use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    fmt::{Debug, Formatter, Result as FmtResult},
    ptr::{drop_in_place, write, NonNull},
};

/// A very unsafe vector whose elements are all `Option<T>`'s, where `T` is chosen with each `get`
/// and `set` operation.
pub struct UnsafeOptionVec {
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
    pub fn new<T: 'static + Send + Sync>() -> UnsafeOptionVec {
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
    fn grow_to<T: 'static + Send + Sync>(&mut self, mut n: usize) {
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

    /// Reads the `n`th value from the `UnsafeOptionVec`. This will extend the underlying
    /// allocation if `n` is out of bounds.
    #[safety(eq(self.layout, Layout::new::<Option<T>>()),
        "T must have the same layout as the type that was given to `UnsafeOptionVec::new`")]
    #[safety("T must be the same type as was given to `UnsafeOptionVec::new`")]
    pub unsafe fn get_mut<T: 'static + Send + Sync>(&mut self, n: usize) -> &mut Option<T> {
        self.grow_to::<T>(
            n.checked_add(1)
                .expect("overflow of size of component store"),
        );
        let ptr = self.ptr(n).cast::<Option<T>>().as_ptr();
        &mut *ptr
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

unsafe impl Send for UnsafeOptionVec {}
unsafe impl Sync for UnsafeOptionVec {}
