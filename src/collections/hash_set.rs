//! A hash set implemented as a `HashMap` where the value is `()`.

pub use std::collections::hash_set::{Drain, Iter};

use crate::alloc::AllocError;
use crate::collections::hash_map::FxBuildHasher;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashSet as StdHashSet;
use std::fmt;
use std::hash::{BuildHasher, Hash};

/// A hash set implemented as a `HashMap` where the value is `()`.
#[repr(transparent)]
pub struct HashSet<T, S = FxBuildHasher>(StdHashSet<T, S>);

impl<T> HashSet<T, FxBuildHasher> {
    /// Creates an empty `HashSet`.
    ///
    /// The hash set is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    #[must_use]
    #[inline]
    pub fn new() -> HashSet<T, FxBuildHasher> {
        HashSet::default()
    }
}

impl<T, S> HashSet<T, S> {
    #[inline]
    pub fn from_std(hash_set: StdHashSet<T, S>) -> Self {
        HashSet(hash_set)
    }

    #[inline]
    pub fn into_std(self) -> StdHashSet<T, S> {
        self.0
    }

    /// Returns the number of elements the set can hold without reallocating.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }

    /// Returns the number of elements in the set.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the set contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clears the set, returning all elements as an iterator. Keeps the
    /// allocated memory for reuse.
    ///
    /// If the returned iterator is dropped before being fully consumed, it
    /// drops the remaining elements. The returned iterator keeps a mutable
    /// borrow on the vector to optimize its implementation.
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, T> {
        self.0.drain()
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// The elements are visited in unsorted (and unspecified) order.
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.0.retain(f)
    }

    /// Clears the set, removing all values.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Creates a new empty hash set which will use the given hasher to hash
    /// keys.
    ///
    /// The hash set is also created with the default initial capacity.
    ///
    /// Warning: `hasher` is normally randomly generated, and
    /// is designed to allow `HashSet`s to be resistant to attacks that
    /// cause many collisions and very poor performance. Setting it
    /// manually using this function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the HashMap to be useful, see its documentation for details.
    #[inline]
    pub fn with_hasher(hasher: S) -> HashSet<T, S> {
        HashSet(StdHashSet::with_hasher(hasher))
    }

    /// Returns a reference to the set's [`BuildHasher`].
    #[inline]
    pub fn hasher(&self) -> &S {
        self.0.hasher()
    }
}

impl<T, S> HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    /// Creates an empty `HashSet` with the specified capacity.
    ///
    /// The hash set will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash set will not allocate.
    #[inline]
    pub fn try_with_capacity(capacity: usize) -> Result<HashSet<T, RandomState>, AllocError> {
        let mut set = StdHashSet::new();
        set.try_reserve(capacity)?;
        Ok(HashSet(set))
    }

    /// Creates an empty `HashSet` with the specified capacity, using
    /// `hasher` to hash the keys.
    ///
    /// The hash set will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash set will not allocate.
    ///
    /// Warning: `hasher` is normally randomly generated, and
    /// is designed to allow `HashSet`s to be resistant to attacks that
    /// cause many collisions and very poor performance. Setting it
    /// manually using this function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the HashMap to be useful, see its documentation for details.
    #[inline]
    pub fn try_with_capacity_and_hasher(capacity: usize, hasher: S) -> Result<HashSet<T, S>, AllocError> {
        let mut set = StdHashSet::with_hasher(hasher);
        set.try_reserve(capacity)?;
        Ok(HashSet(set))
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given `HashSet<K, V>`. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
        self.0.try_reserve(additional)?;
        Ok(())
    }

    /// Returns `true` if the set contains a value.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    #[inline]
    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.contains(value)
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    #[inline]
    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(value)
    }

    /// Adds a value to the set.
    ///
    /// If the set did not have this value present, `true` is returned.
    ///
    /// If the set did have this value present, `false` is returned.
    #[inline]
    pub fn try_insert(&mut self, value: T) -> Result<bool, AllocError> {
        self.0.try_reserve(1)?;
        Ok(self.0.insert(value))
    }

    /// Adds a value to the set, replacing the existing value, if any, that is equal to the given
    /// one. Returns the replaced value.
    #[inline]
    pub fn replace(&mut self, value: T) -> Result<Option<T>, AllocError> {
        self.0.try_reserve(1)?;
        Ok(self.0.replace(value))
    }

    /// Removes a value from the set. Returns whether the value was
    /// present in the set.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    #[inline]
    pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.remove(value)
    }

    /// Removes and returns the value in the set, if any, that is equal to the given one.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    #[inline]
    pub fn take<Q: ?Sized>(&mut self, value: &Q) -> Option<T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.take(value)
    }
}

impl<T, S> PartialEq for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    #[inline]
    fn eq(&self, other: &HashSet<T, S>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T, S> Eq for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

impl<T, S> fmt::Debug for HashSet<T, S>
where
    T: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T, S> Default for HashSet<T, S>
where
    S: Default,
{
    /// Creates an empty `HashSet<T, S>` with the `Default` value for the hasher.
    #[inline]
    fn default() -> HashSet<T, S> {
        HashSet::with_hasher(Default::default())
    }
}
