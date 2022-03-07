//! A contiguous growable array type with heap-allocated contents, written
//! `Vec<T>`.
//!
//! Vectors have *O*(1) indexing, amortized *O*(1) push (to the end) and
//! *O*(1) pop (from the end).
//!
//! Vectors ensure they never allocate more than `isize::MAX` bytes.

use crate::alloc::{Allocator, Global};
use crate::cmp::Ordering;
use crate::collections::TryReserveError;
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::io;
use crate::ops::{Deref, DerefMut, Index, IndexMut, RangeBounds};
use crate::slice::SliceIndex;
use alloc_crate::vec::Vec as StdVec;

pub use alloc_crate::vec::{Drain, IntoIter};

/// A contiguous growable array type, written as `Vec<T>`, short for 'vector'.
#[repr(transparent)]
pub struct Vec<T, A: Allocator = Global>(StdVec<T, A>);

impl<T> Vec<T> {
    /// Constructs a new, empty `Vec<T>`.
    ///
    /// The vector will not allocate until elements are pushed onto it.
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Vec(StdVec::new())
    }

    /// Constructs a new, empty `Vec<T>` with the specified capacity.
    ///
    /// The vector will be able to hold exactly `capacity` elements without
    /// reallocating. If `capacity` is 0, the vector will not allocate.
    #[inline]
    pub fn try_with_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        Self::try_with_capacity_in(capacity, Global)
    }
}

impl<T, A: Allocator> Vec<T, A> {
    /// Constructs a new, empty `Vec<T, A>`.
    ///
    /// The vector will not allocate until elements are pushed onto it.
    #[inline]
    pub const fn new_in(alloc: A) -> Self {
        Vec(StdVec::new_in(alloc))
    }

    /// Constructs a new, empty `Vec<T, A>` with the specified capacity with the provided
    /// allocator.
    ///
    /// The vector will be able to hold exactly `capacity` elements without
    /// reallocating. If `capacity` is 0, the vector will not allocate.
    #[inline]
    pub fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, TryReserveError> {
        let mut vec = Vec::new_in(alloc);
        vec.try_reserve(capacity)?;
        Ok(vec)
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given `Vec<T>`. The collection may reserve more space to avoid
    /// frequent reallocations. After calling `try_reserve`, capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve(additional)
    }

    /// Tries to reserve the minimum capacity for exactly `additional`
    /// elements to be inserted in the given `Vec<T>`. After calling
    /// `try_reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional` if it returns `Ok(())`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore, capacity can not be relied upon to be precisely
    /// minimal.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve_exact(additional)
    }

    #[inline]
    pub fn into_std(self) -> StdVec<T, A> {
        self.0
    }

    #[inline]
    pub fn from_std(v: StdVec<T, A>) -> Self {
        Vec(v)
    }

    /// Returns the number of elements the vector can hold without
    /// reallocating.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the vector's current length, this has no
    /// effect.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the vector.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len)
    }

    /// Extracts a slice containing the entire vector.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Extracts a mutable slice of the entire vector.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }

    /// Returns a raw pointer to the vector's buffer.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }

    /// Returns a reference to the underlying allocator.
    #[inline]
    pub fn allocator(&self) -> &A {
        self.0.allocator()
    }

    /// Forces the length of the vector to `new_len`.
    ///
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to `capacity()`.
    /// - The elements at `old_len..new_len` must be initialized.
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.0.set_len(new_len);
    }

    /// Removes an element from the vector and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// This does not preserve ordering, but is *O*(1).
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.0.swap_remove(index)
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// Note: Because this shifts over the remaining elements, it has a
    /// worst-case performance of O(n). If you don't need the order of elements
    /// to be preserved, use `swap_remove` instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }

    /// Appends an element to the back of a collection.
    #[inline]
    pub fn try_push(&mut self, value: T) -> Result<(), TryReserveError> {
        if self.0.len() == self.0.capacity() {
            self.0.try_reserve(1)?;
        }
        self.0.push(value);
        Ok(())
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// Creates a draining iterator that removes the specified range in the vector
    /// and yields the removed items.
    ///
    /// When the iterator **is** dropped, all elements in the range are removed
    /// from the vector, even if the iterator was not fully consumed. If the
    /// iterator **is not** dropped (with `mem::forget` for example), it is
    /// unspecified how many elements are removed.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point or if
    /// the end point is greater than the length of the vector.
    #[inline]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
    where
        R: RangeBounds<usize>,
    {
        self.0.drain(range)
    }

    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the vector.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns the number of elements in the vector, also referred to
    /// as its 'length'.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the vector contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with the result of
    /// calling the closure `f`. The return values from `f` will end up
    /// in the `Vec` in the order they have been generated.
    ///
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    #[inline]
    pub fn try_resize_with<F>(&mut self, new_len: usize, f: F) -> Result<(), TryReserveError>
    where
        F: FnMut() -> T,
    {
        let len = self.0.len();
        if new_len > len {
            self.0.try_reserve(new_len - len)?;
            self.0.resize_with(new_len, f);
        } else {
            self.0.truncate(new_len);
        }
        Ok(())
    }
}

impl<T, A: Allocator> Deref for Vec<T, A> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.0.deref()
    }
}

impl<T, A: Allocator> DerefMut for Vec<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.0.deref_mut()
    }
}

impl<T: Hash, A: Allocator> Hash for Vec<T, A> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T, I: SliceIndex<[T]>, A: Allocator> Index<I> for Vec<T, A> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut<I> for Vec<T, A> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T, A: Allocator> IntoIterator for Vec<T, A> {
    type Item = T;
    type IntoIter = IntoIter<T, A>;

    #[inline]
    fn into_iter(self) -> IntoIter<T, A> {
        self.0.into_iter()
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> core::slice::Iter<'a, T> {
        self.0.iter()
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a mut Vec<T, A> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> core::slice::IterMut<'a, T> {
        self.0.iter_mut()
    }
}

impl<T: PartialEq, A: Allocator> PartialEq for Vec<T, A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: PartialOrd, A: Allocator> PartialOrd for Vec<T, A> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Eq, A: Allocator> Eq for Vec<T, A> {}

impl<T: Ord, A: Allocator> Ord for Vec<T, A> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: fmt::Debug, A: Allocator> fmt::Debug for Vec<T, A> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T, A: Allocator> AsRef<Vec<T, A>> for Vec<T, A> {
    #[inline]
    fn as_ref(&self) -> &Vec<T, A> {
        self
    }
}

impl<T, A: Allocator> AsMut<Vec<T, A>> for Vec<T, A> {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<T, A> {
        self
    }
}

impl<T, A: Allocator> AsRef<[T]> for Vec<T, A> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T, A: Allocator> AsMut<[T]> for Vec<T, A> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<A: Allocator> io::Write for Vec<u8, A> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0
            .try_reserve(buf.len())
            .map_err(|_| io::Error::from(io::ErrorKind::OutOfMemory))?;
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        let len = bufs.iter().map(|b| b.len()).sum();
        self.0
            .try_reserve(len)
            .map_err(|_| io::Error::from(io::ErrorKind::OutOfMemory))?;
        for buf in bufs {
            self.0.extend_from_slice(buf);
        }
        Ok(len)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        true
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0
            .try_reserve(buf.len())
            .map_err(|_| io::Error::from(io::ErrorKind::OutOfMemory))?;
        self.0.extend_from_slice(buf);
        Ok(())
    }
}
