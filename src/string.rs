//! A UTF-8–encoded, growable string.

pub use std::string::FromUtf8Error;

use crate::alloc::AllocError;
use crate::borrow::{Cow, TryToOwned};
use crate::clone::TryClone;
use crate::vec::Vec;
use std::borrow::Borrow;
use std::fmt;
use std::mem;
use std::ops;
use std::str;
use std::string::String as StdString;

/// A UTF-8–encoded, growable string.
///
/// The `String` type is the most common string type that has ownership over the
/// contents of the string. It has a close relationship with its borrowed
/// counterpart, the primitive `str`.
#[derive(Ord, PartialOrd, Eq, PartialEq)]
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
    pub fn try_with_capacity(capacity: usize) -> Result<String, AllocError> {
        let mut s = StdString::new();
        s.try_reserve(capacity)?;
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

    /// Converts a vector of bytes to a `String`.
    ///
    /// A string ([`String`]) is made of bytes ([`u8`]), and a vector of bytes
    /// ([`Vec<u8>`]) is made of bytes, so this function converts between the
    /// two. Not all byte slices are valid `String`s, however: `String`
    /// requires that it is valid UTF-8. `from_utf8()` checks to ensure that
    /// the bytes are valid UTF-8, and then does the conversion.
    ///
    /// If you are sure that the byte slice is valid UTF-8, and you don't want
    /// to incur the overhead of the validity check, there is an unsafe version
    /// of this function, [`String::from_utf8_unchecked`], which has the same behavior
    /// but skips the check.
    ///
    /// This method will take care to not copy the vector, for efficiency's
    /// sake.
    ///
    /// If you need a [`&str`] instead of a `String`, consider
    /// [`str::from_utf8`].
    ///
    /// The inverse of this method is [`String::into_bytes`].
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the slice is not UTF-8 with a description as to why the
    /// provided bytes are not UTF-8. The vector you moved in is also included.
    #[inline]
    pub fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error> {
        Ok(String(StdString::from_utf8(vec.into_std())?))
    }

    /// Converts a vector of bytes to a `String` without checking that the
    /// string contains valid UTF-8.
    ///
    /// See the safe version, [`from_utf8`], for more details.
    ///
    /// [`from_utf8`]: String::from_utf8
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed
    /// to it are valid UTF-8. If this constraint is violated, it may cause
    /// memory unsafety issues with future users of the `String`, as the rest of
    /// the standard library assumes that `String`s are valid UTF-8.
    #[must_use]
    #[inline]
    pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> String {
        String(StdString::from_utf8_unchecked(bytes.into_std()))
    }

    /// Converts a `String` into a byte vector.
    ///
    /// This consumes the `String`, so we do not need to copy its contents.
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_bytes(self) -> Vec<u8> {
        Vec::from_std(self.0.into_bytes())
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

    /// Returns a mutable reference to the contents of this `String`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the returned `&mut Vec` allows writing
    /// bytes which are not valid UTF-8. If this constraint is violated, using
    /// the original `String` after dropping the `&mut Vec` may violate memory
    /// safety, as the rest of the standard library assumes that `String`s are
    /// valid UTF-8.
    #[inline]
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        // Vec has the same memory layout as StdVec
        std::mem::transmute(self.0.as_mut_vec())
    }

    /// Appends a given string slice onto the end of this `String`.
    #[inline]
    pub fn try_push_str(&mut self, string: &str) -> Result<(), AllocError> {
        self.0.try_reserve(string.len())?;
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
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
        self.0.try_reserve(additional)?;
        Ok(())
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
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), AllocError> {
        self.0.try_reserve_exact(additional)?;
        Ok(())
    }

    /// Appends the given [`char`] to the end of this `String`.
    #[inline]
    pub fn try_push(&mut self, ch: char) -> Result<(), AllocError> {
        self.try_reserve(mem::size_of::<char>())?;
        self.0.push(ch);
        Ok(())
    }

    /// Notes: This function has OOM panic problem.
    #[inline]
    pub(crate) fn push(&mut self, ch: char) {
        self.0.push(ch);
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

impl Borrow<str> for String {
    #[inline]
    fn borrow(&self) -> &str {
        self
    }
}

impl TryClone for String {
    #[inline]
    fn try_clone(&self) -> Result<Self, AllocError> {
        let mut s = String::new();
        s.try_push_str(self)?;
        Ok(s)
    }

    #[inline]
    fn try_clone_from(&mut self, source: &Self) -> Result<(), AllocError> {
        self.clear();
        self.try_push_str(source)?;
        Ok(())
    }
}

impl AsRef<str> for String {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsMut<str> for String {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        self
    }
}

impl AsRef<[u8]> for String {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl TryFrom<&str> for String {
    type Error = AllocError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.try_to_owned()
    }
}

impl TryFrom<&mut str> for String {
    type Error = AllocError;

    #[inline]
    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.try_to_owned()
    }
}

impl TryFrom<&String> for String {
    type Error = AllocError;

    #[inline]
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        s.try_clone()
    }
}

impl<'a> TryFrom<Cow<'a, str>> for String {
    type Error = AllocError;

    #[inline]
    fn try_from(s: Cow<'a, str>) -> Result<Self, Self::Error> {
        s.into_owned()
    }
}

impl<'a> From<&'a str> for Cow<'a, str> {
    #[inline]
    fn from(s: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(s)
    }
}

impl<'a> From<String> for Cow<'a, str> {
    #[inline]
    fn from(s: String) -> Cow<'a, str> {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a String> for Cow<'a, str> {
    #[inline]
    fn from(s: &'a String) -> Cow<'a, str> {
        Cow::Borrowed(s.as_str())
    }
}

/// A trait for converting a value to a `String`.
pub trait TryToString {
    /// Converts the given value to a `String`.
    fn try_to_string(&self) -> Result<String, AllocError>;
}

impl TryToString for str {
    #[inline]
    fn try_to_string(&self) -> Result<String, AllocError> {
        self.try_to_owned()
    }
}

impl TryToString for Cow<'_, str> {
    #[inline]
    fn try_to_string(&self) -> Result<String, AllocError> {
        self.as_ref().try_to_owned()
    }
}

impl TryToString for String {
    #[inline]
    fn try_to_string(&self) -> Result<String, AllocError> {
        self.try_clone()
    }
}
