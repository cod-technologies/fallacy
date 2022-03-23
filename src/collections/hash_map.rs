//! A hash map implemented with quadratic probing.

pub use std::collections::hash_map::{
    Drain, Entry, IntoKeys, IntoValues, Iter, IterMut, Keys, RandomState, Values, ValuesMut,
};

use crate::alloc::AllocError;
use std::borrow::Borrow;
use std::collections::HashMap as StdHashMap;
use std::hash::{BuildHasher, Hash};

/// A hash map implemented with quadratic probing.
#[repr(transparent)]
pub struct HashMap<K, V, S = RandomState>(StdHashMap<K, V, S>);

impl<K, V> HashMap<K, V, RandomState> {
    /// Creates an empty `HashMap`.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    #[must_use]
    #[inline]
    pub fn new() -> HashMap<K, V, RandomState> {
        HashMap(StdHashMap::new())
    }
}

impl<K, V, S> HashMap<K, V, S> {
    #[inline]
    pub fn from_std(hash_map: StdHashMap<K, V, S>) -> Self {
        HashMap(hash_map)
    }

    #[inline]
    pub fn into_std(self) -> StdHashMap<K, V, S> {
        self.0
    }

    /// Creates an empty `HashMap` which will use the given hash builder to hash
    /// keys.
    ///
    /// The created map has the default initial capacity.
    ///
    /// Warning: `hash_builder` is normally randomly generated, and
    /// is designed to allow HashMaps to be resistant to attacks that
    /// cause many collisions and very poor performance. Setting it
    /// manually using this function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the HashMap to be useful, see its documentation for details.
    #[inline]
    pub fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap(StdHashMap::with_hasher(hash_builder))
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `HashMap<K, V>` might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// An iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&'a K`.
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        self.0.keys()
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `K`.
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        self.0.into_keys()
    }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        self.0.values()
    }

    /// An iterator visiting all values mutably in arbitrary order.
    /// The iterator element type is `&'a mut V`.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.0.values_mut()
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `V`.
    #[inline]
    pub fn into_values(self) -> IntoValues<K, V> {
        self.0.into_values()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.0.iter()
    }

    /// An iterator visiting all key-value pairs in arbitrary order,
    /// with mutable references to the values.
    /// The iterator element type is `(&'a K, &'a mut V)`.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.0.iter_mut()
    }

    /// Returns the number of elements in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clears the map, returning all key-value pairs as an iterator. Keeps the
    /// allocated memory for reuse.
    ///
    /// If the returned iterator is dropped before being fully consumed, it
    /// drops the remaining key-value pairs. The returned iterator keeps a
    /// mutable borrow on the vector to optimize its implementation.
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, K, V> {
        self.0.drain()
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` for which `f(&k, &mut v)` returns `false`.
    /// The elements are visited in unsorted (and unspecified) order.
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.0.retain(f);
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory
    /// for reuse.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns a reference to the map's [`BuildHasher`].
    #[inline]
    pub fn hasher(&self) -> &S {
        self.0.hasher()
    }
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    /// Creates an empty `HashMap` with the specified capacity.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    #[inline]
    pub fn try_with_capacity(capacity: usize) -> Result<HashMap<K, V, S>, AllocError>
    where
        S: Default,
    {
        let mut m = StdHashMap::with_hasher(Default::default());
        m.try_reserve(capacity)?;
        Ok(HashMap(m))
    }

    /// Creates an empty `HashMap` with the specified capacity, using `hash_builder`
    /// to hash the keys.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    ///
    /// Warning: `hash_builder` is normally randomly generated, and
    /// is designed to allow HashMaps to be resistant to attacks that
    /// cause many collisions and very poor performance. Setting it
    /// manually using this function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the HashMap to be useful, see its documentation for details.
    #[inline]
    pub fn try_with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Result<HashMap<K, V, S>, AllocError> {
        let mut m = StdHashMap::with_hasher(hash_builder);
        m.try_reserve(capacity)?;
        Ok(HashMap(m))
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given `HashMap<K, V>`. The collection may reserve more space to avoid
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

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    #[inline]
    pub fn try_entry(&mut self, key: K) -> Result<Entry<'_, K, V>, AllocError> {
        self.0.try_reserve(1)?;
        Ok(self.0.entry(key))
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(k)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_key_value(k)
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.contains_key(k)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_mut(k)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    #[inline]
    pub fn try_insert(&mut self, k: K, v: V) -> Result<Option<V>, AllocError> {
        self.0.try_reserve(1)?;
        Ok(self.0.insert(k, v))
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.remove(k)
    }

    /// Removes a key from the map, returning the stored key and value if the
    /// key was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.remove_entry(k)
    }
}

impl<K, V, S> Default for HashMap<K, V, S>
where
    S: Default,
{
    #[inline]
    fn default() -> Self {
        HashMap::with_hasher(Default::default())
    }
}
