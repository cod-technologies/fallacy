//! A thread-safe reference-counting pointer.

use crate::alloc::AllocationError;
use crate::cmp::Ordering;
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::ops::Deref;
use alloc_crate::sync::Arc as StdArc;
use std::alloc::Layout;

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
#[repr(transparent)]
pub struct Arc<T: ?Sized>(StdArc<T>);

impl<T> Arc<T> {
    /// Constructs a new `Arc<T>`, returning an error if allocation fails.
    #[inline]
    pub fn try_new(data: T) -> Result<Arc<T>, AllocationError> {
        Ok(Arc(
            StdArc::try_new(data).map_err(|_| AllocationError::AllocError(Layout::new::<T>()))?
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

impl<T: ?Sized> Clone for Arc<T> {
    /// Makes a clone of the `Arc` pointer.
    ///
    /// This creates another pointer to the same allocation, increasing the
    /// strong reference count.
    #[inline]
    fn clone(&self) -> Arc<T> {
        Arc(self.0.clone())
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

impl<T: ?Sized + PartialEq> PartialEq for Arc<T> {
    #[inline]
    fn eq(&self, other: &Arc<T>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: ?Sized + PartialOrd> PartialOrd for Arc<T> {
    #[inline]
    fn partial_cmp(&self, other: &Arc<T>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    #[inline]
    fn lt(&self, other: &Arc<T>) -> bool {
        self.0.lt(&other.0)
    }

    #[inline]
    fn le(&self, other: &Arc<T>) -> bool {
        self.0.le(&other.0)
    }

    #[inline]
    fn gt(&self, other: &Arc<T>) -> bool {
        self.0.gt(&other.0)
    }

    #[inline]
    fn ge(&self, other: &Arc<T>) -> bool {
        self.0.ge(&other.0)
    }
}

impl<T: ?Sized + Ord> Ord for Arc<T> {
    #[inline]
    fn cmp(&self, other: &Arc<T>) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ?Sized + Eq> Eq for Arc<T> {}

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

impl<T: ?Sized + Hash> Hash for Arc<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
