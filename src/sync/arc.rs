//! A thread-safe reference-counting pointer.

use crate::alloc::AllocError;
use crate::clone::TryClone;
use std::alloc::Layout;
use std::fmt;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc as StdArc;

/// A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically
/// Reference Counted'.
///
/// The type `Arc<T>` provides shared ownership cod
/// of a value of type `T`,
/// allocated in the heap. Invoking `clone` on `Arc` produces
/// a new `Arc` instance, which points to the same allocation on the heap as the
/// source `Arc`, while increasing a reference count. When the last `Arc`
/// pointer to a given allocation is destroyed, the value stored in that allocation (often
/// referred to as "inner value") is also dropped.
#[derive(Clone, TryClone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Arc<T: ?Sized>(StdArc<T>);

impl<T> Arc<T> {
    /// Constructs a new `Arc<T>`, returning an error if allocation fails.
    #[inline]
    pub fn try_new(data: T) -> Result<Arc<T>, AllocError> {
        Ok(Arc(
            StdArc::try_new(data).map_err(|_| AllocError::new(Layout::new::<T>()))?
        ))
    }

    #[inline]
    pub fn into_std(self) -> StdArc<T> {
        self.0
    }

    #[inline]
    pub fn from_std(a: StdArc<T>) -> Self {
        Arc(a)
    }
}

impl<T: ?Sized> Deref for Arc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T: ?Sized> AsRef<T> for Arc<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for Arc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Arc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: ?Sized> fmt::Pointer for Arc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}
