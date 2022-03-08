//! A pointer type for heap allocation.

use crate::alloc::{AllocationError, Allocator, Global, Layout};
use crate::clone::TryClone;
use crate::cmp::Ordering;
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::ops::{Deref, DerefMut};
use crate::result::Result;
use alloc_crate::boxed::Box as StdBox;

/// A pointer type for heap allocation.
#[repr(transparent)]
pub struct Box<T: ?Sized, A: Allocator = Global>(StdBox<T, A>);

impl<T> Box<T> {
    /// Allocates memory on the heap then places `x` into it,
    /// returning an error if the allocation fails
    ///
    /// This doesn't actually allocate if `T` is zero-sized.
    #[inline]
    pub fn try_new(x: T) -> Result<Self, AllocationError> {
        Ok(Box(
            StdBox::try_new(x).map_err(|_| AllocationError::AllocError(Layout::new::<T>()))?
        ))
    }
}

impl<T: ?Sized> Box<T> {
    /// Constructs a box from a raw pointer.
    ///
    /// After calling this function, the raw pointer is owned by the
    /// resulting `Box`. Specifically, the `Box` destructor will call
    /// the destructor of `T` and free the allocated memory. For this
    /// to be safe, the memory must have been allocated in accordance
    /// with the [memory layout] used by `Box` .
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    #[inline]
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Box(StdBox::from_raw(raw))
    }
}

impl<T, A: Allocator> Box<T, A> {
    /// Allocates memory in the given allocator then places `x` into it,
    /// returning an error if the allocation fails
    ///
    /// This doesn't actually allocate if `T` is zero-sized.
    #[inline]
    pub fn try_new_in(x: T, alloc: A) -> Result<Self, AllocationError> {
        Ok(Box(
            StdBox::try_new_in(x, alloc).map_err(|_| AllocationError::AllocError(Layout::new::<T>()))?
        ))
    }
}

impl<T: ?Sized, A: Allocator> Box<T, A> {
    /// Constructs a box from a raw pointer in the given allocator.
    ///
    /// After calling this function, the raw pointer is owned by the
    /// resulting `Box`. Specifically, the `Box` destructor will call
    /// the destructor of `T` and free the allocated memory. For this
    /// to be safe, the memory must have been allocated in accordance
    /// with the [memory layout] used by `Box` .
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    #[inline]
    pub unsafe fn from_raw_in(raw: *mut T, alloc: A) -> Self {
        Box(StdBox::from_raw_in(raw, alloc))
    }

    /// Consumes the `Box`, returning a wrapped raw pointer.
    ///
    /// The pointer will be properly aligned and non-null.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `Box`. In particular, the
    /// caller should properly destroy `T` and release the memory, taking
    /// into account the [memory layout] used by `Box`. The easiest way to
    /// do this is to convert the raw pointer back into a `Box` with the
    /// [`Box::from_raw`] function, allowing the `Box` destructor to perform
    /// the cleanup.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::into_raw(b)` instead of `b.into_raw()`. This
    /// is so that there is no conflict with a method on the inner type.
    #[inline]
    pub fn into_raw(b: Self) -> *mut T {
        StdBox::into_raw(b.0)
    }

    /// Consumes the `Box`, returning a wrapped raw pointer and the allocator.
    ///
    /// The pointer will be properly aligned and non-null.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `Box`. In particular, the
    /// caller should properly destroy `T` and release the memory, taking
    /// into account the [memory layout] used by `Box`. The easiest way to
    /// do this is to convert the raw pointer back into a `Box` with the
    /// [`Box::from_raw_in`] function, allowing the `Box` destructor to perform
    /// the cleanup.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::into_raw_with_allocator(b)` instead of `b.into_raw_with_allocator()`. This
    /// is so that there is no conflict with a method on the inner type.
    #[inline]
    pub fn into_raw_with_allocator(b: Self) -> (*mut T, A) {
        StdBox::into_raw_with_allocator(b.0)
    }

    /// Returns a reference to the underlying allocator.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::allocator(&b)` instead of `b.allocator()`. This
    /// is so that there is no conflict with a method on the inner type.
    #[inline]
    pub fn allocator(b: &Self) -> &A {
        StdBox::allocator(&b.0)
    }

    /// Consumes and leaks the `Box`, returning a mutable reference,
    /// `&'a mut T`. Note that the type `T` must outlive the chosen lifetime
    /// `'a`. If the type has only static references, or none at all, then this
    /// may be chosen to be `'static`.
    ///
    /// This function is mainly useful for data that lives for the remainder of
    /// the program's life. Dropping the returned reference will cause a memory
    /// leak. If this is not acceptable, the reference should first be wrapped
    /// with the [`Box::from_raw`] function producing a `Box`. This `Box` can
    /// then be dropped which will properly destroy `T` and release the
    /// allocated memory.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::leak(b)` instead of `b.leak()`. This
    /// is so that there is no conflict with a method on the inner type.
    #[inline]
    pub fn leak<'a>(b: Self) -> &'a mut T
    where
        A: 'a,
    {
        StdBox::leak(b.0)
    }

    #[inline]
    pub fn into_std(self) -> StdBox<T, A> {
        self.0
    }

    #[inline]
    pub fn from_std(b: StdBox<T, A>) -> Self {
        Box(b)
    }
}

impl<T: ?Sized, A: Allocator> Deref for Box<T, A> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T: ?Sized, A: Allocator> DerefMut for Box<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}

impl<T: ?Sized, A: Allocator> AsRef<T> for Box<T, A> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T: ?Sized, A: Allocator> AsMut<T> for Box<T, A> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.0.as_mut()
    }
}

impl<T: ?Sized + PartialEq, A: Allocator> PartialEq for Box<T, A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: ?Sized + PartialOrd, A: Allocator> PartialOrd for Box<T, A> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
}

impl<T: ?Sized + Ord, A: Allocator> Ord for Box<T, A> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ?Sized + Eq, A: Allocator> Eq for Box<T, A> {}

impl<T: ?Sized + Hash, A: Allocator> Hash for Box<T, A> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T: ?Sized + Hasher, A: Allocator> Hasher for Box<T, A> {
    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.0.write_u8(i)
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.0.write_u16(i)
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.0.write_u32(i)
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0.write_u64(i)
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.0.write_u128(i)
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.0.write_usize(i)
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.0.write_i8(i)
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.0.write_i16(i)
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.0.write_i32(i)
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.0.write_i64(i)
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.0.write_i128(i)
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.0.write_isize(i)
    }
}

impl<T: fmt::Display + ?Sized, A: Allocator> fmt::Display for Box<T, A> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: fmt::Debug + ?Sized, A: Allocator> fmt::Debug for Box<T, A> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: ?Sized, A: Allocator> fmt::Pointer for Box<T, A> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

impl<T: TryClone> TryClone for Box<T> {
    #[inline]
    fn try_clone(&self) -> Result<Self, AllocationError> {
        let clone = self.0.try_clone()?;
        Self::try_new(clone)
    }
}
