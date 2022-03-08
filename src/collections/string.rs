//! A UTF-8–encoded, growable string.

use crate::alloc::AllocationError;
use crate::fmt;
use crate::mem;
use crate::ops;
use crate::str;
use alloc_crate::string::String as StdString;

#[repr(transparent)]
pub struct String(StdString);

impl String {
    /// Creates a new empty `String`.
    #[inline]
    pub const fn new() -> String {
        String(StdString::new())
    }

    /// Creates a new empty `String` with a particular capacity.
    #[inline]
    pub fn try_with_capacity(capacity: usize) -> Result<String, AllocationError> {
        let mut s = StdString::new();
        s.try_reserve(capacity)
            .map_err(|e| AllocationError::from_try_reserve_error(e, usize::MAX))?;
        Ok(String(s))
    }

    #[inline]
    pub fn from_std(s: StdString) -> Self {
        String(s)
    }

    #[inline]
    pub fn into_std(self) -> StdString {
        self.0
    }

    /// Returns this `String`'s capacity, in bytes.
    #[must_use]
    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the length of this `String`, in bytes, not [`char`]s or
    /// graphemes. In other words, it might not be what a human considers the
    /// length of the string.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if this `String` has a length of zero, and `false` otherwise.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Extracts a string slice containing the entire `String`.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Converts a `String` into a mutable string slice.
    #[must_use]
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        self.0.as_mut_str()
    }

    /// Appends a given string slice onto the end of this `String`.
    #[inline]
    pub fn try_push_str(&mut self, string: &str) -> Result<(), AllocationError> {
        self.0
            .try_reserve(string.len())
            .map_err(|e| AllocationError::from_try_reserve_error(e, usize::MAX))?;
        self.0.push_str(string);
        Ok(())
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given `String`. The collection may reserve more space to avoid
    /// frequent reallocations. After calling `reserve`, capacity will be
    /// greater than or equal to `self.len() + additional`. Does nothing if
    /// capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocationError> {
        self.0
            .try_reserve(additional)
            .map_err(|e| AllocationError::from_try_reserve_error(e, usize::MAX))
    }

    /// Tries to reserve the minimum capacity for exactly `additional` more elements to
    /// be inserted in the given `String`. After calling `reserve_exact`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore, capacity can not be relied upon to be precisely
    /// minimal. Prefer [`try_reserve`] if future insertions are expected.
    ///
    /// [`try_reserve`]: String::try_reserve
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), AllocationError> {
        self.0
            .try_reserve_exact(additional)
            .map_err(|e| AllocationError::from_try_reserve_error(e, usize::MAX))
    }

    /// Appends the given [`char`] to the end of this `String`.
    #[inline]
    pub fn try_push(&mut self, ch: char) -> Result<(), AllocationError> {
        self.try_reserve(mem::size_of::<char>())?;
        self.0.push(ch);
        Ok(())
    }

    /// Returns a byte slice of this `String`'s contents.
    #[must_use]
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Shortens this `String` to the specified length.
    ///
    /// If `new_len` is greater than the string's current length, this has no
    /// effect.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the string
    ///
    /// # Panics
    ///
    /// Panics if `new_len` does not lie on a [`char`] boundary.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.0.truncate(new_len);
    }

    /// Truncates this `String`, removing all contents.
    ///
    /// While this means the `String` will have a length of zero, it does not
    /// touch its capacity.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl PartialEq for String {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.0.eq(&other.0)
    }
}

macro_rules! impl_eq {
    ($lhs:ty, $rhs: ty) => {
        #[allow(unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                PartialEq::eq(&self[..], &other[..])
            }
        }

        #[allow(unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                PartialEq::eq(&self[..], &other[..])
            }
        }
    };
}

impl_eq! { String, str }
impl_eq! { String, &'a str }

impl ops::Index<ops::Range<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &str {
        &self[..][index]
    }
}

impl ops::Index<ops::RangeTo<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &str {
        &self[..][index]
    }
}

impl ops::Index<ops::RangeFrom<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &str {
        &self[..][index]
    }
}

impl ops::Index<ops::RangeFull> for String {
    type Output = str;

    #[inline]
    fn index(&self, _index: ops::RangeFull) -> &str {
        self.as_str()
    }
}

impl ops::Index<ops::RangeInclusive<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeInclusive<usize>) -> &str {
        self.0.index(index)
    }
}

impl ops::Index<ops::RangeToInclusive<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: ops::RangeToInclusive<usize>) -> &str {
        self.0.index(index)
    }
}

impl ops::IndexMut<ops::Range<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut str {
        &mut self[..][index]
    }
}

impl ops::IndexMut<ops::RangeTo<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeTo<usize>) -> &mut str {
        &mut self[..][index]
    }
}

impl ops::IndexMut<ops::RangeFrom<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFrom<usize>) -> &mut str {
        &mut self[..][index]
    }
}

impl ops::IndexMut<ops::RangeFull> for String {
    #[inline]
    fn index_mut(&mut self, _index: ops::RangeFull) -> &mut str {
        self.as_mut_str()
    }
}

impl ops::IndexMut<ops::RangeInclusive<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeInclusive<usize>) -> &mut str {
        self.0.index_mut(index)
    }
}

impl ops::IndexMut<ops::RangeToInclusive<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeToInclusive<usize>) -> &mut str {
        self.0.index_mut(index)
    }
}

impl ops::Deref for String {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.0.deref()
    }
}

impl ops::DerefMut for String {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        self.0.deref_mut()
    }
}

impl fmt::Display for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Write for String {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.try_push_str(s).map_err(|_| fmt::Error)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.try_push(c).map_err(|_| fmt::Error)
    }
}
