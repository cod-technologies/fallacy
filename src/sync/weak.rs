use crate::clone::TryClone;
use crate::sync::Arc;
use std::fmt;
use std::sync::Weak as StdWeak;

/// `Weak` is a version of [`Arc`] that holds a non-owning reference to the
/// managed allocation. The allocation is accessed by calling [`upgrade`] on the `Weak`
/// pointer, which returns an <code>[Option]<[Arc]\<T>></code>.
///
/// Since a `Weak` reference does not count towards ownership, it will not
/// prevent the value stored in the allocation from being dropped, and `Weak` itself makes no
/// guarantees about the value still being present. Thus it may return [`None`]
/// when [`upgrade`]d. Note however that a `Weak` reference *does* prevent the allocation
/// itself (the backing store) from being deallocated.
///
/// A `Weak` pointer is useful for keeping a temporary reference to the allocation
/// managed by [`Arc`] without preventing its inner value from being dropped. It is also used to
/// prevent circular references between [`Arc`] pointers, since mutual owning references
/// would never allow either [`Arc`] to be dropped. For example, a tree could
/// have strong [`Arc`] pointers from parent nodes to children, and `Weak`
/// pointers from children back to their parents.
///
/// The typical way to obtain a `Weak` pointer is to call [`Arc::downgrade`].
///
/// [`upgrade`]: Weak::upgrade
#[derive(Clone, TryClone, Default)]
#[repr(transparent)]
pub struct Weak<T: ?Sized>(StdWeak<T>);

impl<T> Weak<T> {
    /// Constructs a new `Weak<T>`, without allocating any memory.
    /// Calling [`upgrade`] on the return value always gives [`None`].
    ///
    /// [`upgrade`]: Weak::upgrade
    #[must_use]
    pub fn new() -> Weak<T> {
        Weak(StdWeak::new())
    }
}

impl<T: ?Sized> Weak<T> {
    #[inline]
    pub fn into_std(self) -> StdWeak<T> {
        self.0
    }

    #[inline]
    pub fn from_std(w: StdWeak<T>) -> Self {
        Weak(w)
    }

    /// Attempts to upgrade the `Weak` pointer to an [`Arc`], delaying
    /// dropping of the inner value if successful.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    #[must_use = "this returns a new `Arc`, \
                  without modifying the original weak pointer"]
    #[inline]
    pub fn upgrade(&self) -> Option<Arc<T>> {
        self.0.upgrade().map(Arc::from_std)
    }

    /// Gets the number of strong (`Arc`) pointers pointing to this allocation.
    ///
    /// If `self` was created using [`Weak::new`], this will return 0.
    #[must_use]
    #[inline]
    pub fn strong_count(&self) -> usize {
        self.0.strong_count()
    }

    /// Gets an approximation of the number of `Weak` pointers pointing to this
    /// allocation.
    ///
    /// If `self` was created using [`Weak::new`], or if there are no remaining
    /// strong pointers, this will return 0.
    ///
    /// # Accuracy
    ///
    /// Due to implementation details, the returned value can be off by 1 in
    /// either direction when other threads are manipulating any `Arc`s or
    /// `Weak`s pointing to the same allocation.
    #[must_use]
    #[inline]
    pub fn weak_count(&self) -> usize {
        self.0.weak_count()
    }

    /// Returns `true` if the two `Weak`s point to the same allocation (similar to
    /// [`std::ptr::eq`]), or if both don't point to any allocation
    /// (because they were created with `Weak::new()`).
    #[inline]
    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Weak<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
