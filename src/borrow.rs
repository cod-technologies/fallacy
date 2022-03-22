//! A module for working with borrowed data.

use crate::alloc::AllocError;
use crate::clone::TryClone;
use crate::string::String;
use crate::vec::Vec;
use std::borrow::Borrow;
use std::ops::Deref;

/// A generalization of `TryClone` to borrowed data.
///
/// Some types make it possible to go from borrowed to owned, usually by
/// implementing the `TryClone` trait. But `TryClone` works only for going from `&T`
/// to `T`. The `TryToOwned` trait generalizes `TryClone` to construct owned data
/// from any borrow of a given type.
pub trait TryToOwned {
    /// The resulting type after obtaining ownership.
    type Owned: Borrow<Self>;

    /// Creates owned data from borrowed data, usually by cloning.
    fn try_to_owned(&self) -> Result<Self::Owned, AllocError>;

    /// Uses borrowed data to replace owned data, usually by cloning.
    ///
    /// This is borrow-generalized version of `TryClone::try_clone_from`.
    #[inline]
    fn try_clone_into(&self, target: &mut Self::Owned) -> Result<(), AllocError> {
        *target = self.try_to_owned()?;
        Ok(())
    }
}

impl<T> TryToOwned for T
where
    T: TryClone,
{
    type Owned = T;

    #[inline]
    fn try_to_owned(&self) -> Result<Self::Owned, AllocError> {
        self.try_clone()
    }

    #[inline]
    fn try_clone_into(&self, target: &mut Self::Owned) -> Result<(), AllocError> {
        target.try_clone_from(self)
    }
}

impl TryToOwned for str {
    type Owned = String;

    #[inline]
    fn try_to_owned(&self) -> Result<Self::Owned, AllocError> {
        let mut s = String::new();
        s.try_push_str(self)?;
        Ok(s)
    }

    #[inline]
    fn try_clone_into(&self, target: &mut String) -> Result<(), AllocError> {
        target.clear();
        target.try_push_str(self)
    }
}

impl TryToOwned for [u8] {
    type Owned = Vec<u8>;

    #[inline]
    fn try_to_owned(&self) -> Result<Self::Owned, AllocError> {
        let mut s = Vec::new();
        s.try_copy_from_slice(self)?;
        Ok(s)
    }

    #[inline]
    fn try_clone_into(&self, target: &mut Vec<u8>) -> Result<(), AllocError> {
        target.clear();
        target.try_copy_from_slice(self)
    }
}

/// A clone-on-write smart pointer.
///
/// The type `Cow` is a smart pointer providing clone-on-write functionality: it
/// can enclose and provide immutable access to borrowed data, and clone the
/// data lazily when mutation or ownership is required. The type is designed to
/// work with general borrowed data via the `Borrow` trait.
///
/// `Cow` implements `Deref`, which means that you can call
/// non-mutating methods directly on the data it encloses. If mutation
/// is desired, `to_mut` will obtain a mutable reference to an owned
/// value, cloning if necessary.
pub enum Cow<'a, B: ?Sized + 'a>
where
    B: TryToOwned,
{
    /// Borrowed data.
    Borrowed(&'a B),
    /// Owned data.
    Owned(<B as TryToOwned>::Owned),
}

impl<B: ?Sized + TryToOwned> TryClone for Cow<'_, B> {
    #[inline]
    fn try_clone(&self) -> Result<Self, AllocError> {
        match self {
            Cow::Borrowed(b) => Ok(Cow::Borrowed(*b)),
            Cow::Owned(o) => {
                let b: &B = o.borrow();
                Ok(Cow::Owned(b.try_to_owned()?))
            }
        }
    }

    #[inline]
    fn try_clone_from(&mut self, source: &Self) -> Result<(), AllocError> {
        match (self, source) {
            (&mut Cow::Owned(ref mut dest), &Cow::Owned(ref o)) => o.borrow().try_clone_into(dest)?,
            (t, s) => *t = s.try_clone()?,
        }
        Ok(())
    }
}

impl<B: ?Sized + TryToOwned> Cow<'_, B> {
    /// Returns true if the data is borrowed, i.e. if `to_mut` would require additional work.
    #[inline]
    pub const fn is_borrowed(&self) -> bool {
        match *self {
            Cow::Borrowed(_) => true,
            Cow::Owned(_) => false,
        }
    }

    /// Returns true if the data is owned, i.e. if `to_mut` would be a no-op.
    #[inline]
    pub const fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    /// Acquires a mutable reference to the owned form of the data.
    ///
    /// Clones the data if it is not already owned.
    #[inline]
    pub fn to_mut(&mut self) -> Result<&mut <B as TryToOwned>::Owned, AllocError> {
        match self {
            Cow::Borrowed(borrowed) => {
                *self = Cow::Owned(borrowed.try_to_owned()?);
                match self {
                    Cow::Borrowed(..) => unreachable!(),
                    Cow::Owned(owned) => Ok(owned),
                }
            }
            Cow::Owned(owned) => Ok(owned),
        }
    }

    /// Extracts the owned data.
    ///
    /// Clones the data if it is not already owned.
    #[inline]
    pub fn into_owned(self) -> Result<<B as TryToOwned>::Owned, AllocError> {
        match self {
            Cow::Borrowed(borrowed) => borrowed.try_to_owned(),
            Cow::Owned(owned) => Ok(owned),
        }
    }
}

impl<B: ?Sized + TryToOwned> Deref for Cow<'_, B>
where
    B::Owned: Borrow<B>,
{
    type Target = B;

    #[inline]
    fn deref(&self) -> &B {
        match self {
            Cow::Borrowed(borrowed) => borrowed,
            Cow::Owned(owned) => owned.borrow(),
        }
    }
}

impl<T: ?Sized + TryToOwned> AsRef<T> for Cow<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}
