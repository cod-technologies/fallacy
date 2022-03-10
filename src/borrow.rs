//! A module for working with borrowed data.

pub use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

use crate::alloc::AllocationError;
use crate::clone::TryClone;

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
    fn try_to_owned(&self) -> Result<Self::Owned, AllocationError>;

    /// Uses borrowed data to replace owned data, usually by cloning.
    ///
    /// This is borrow-generalized version of `TryClone::try_clone_from`.
    #[inline]
    fn try_clone_into(&self, target: &mut Self::Owned) -> Result<(), AllocationError> {
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
    fn try_to_owned(&self) -> Result<Self::Owned, AllocationError> {
        self.try_clone()
    }

    #[inline]
    fn try_clone_into(&self, target: &mut Self::Owned) -> Result<(), AllocationError> {
        target.try_clone_from(self)
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
    fn try_clone(&self) -> Result<Self, AllocationError> {
        match self {
            Cow::Borrowed(b) => Ok(Cow::Borrowed(*b)),
            Cow::Owned(o) => {
                let b: &B = o.borrow();
                Ok(Cow::Owned(b.try_to_owned()?))
            }
        }
    }

    #[inline]
    fn try_clone_from(&mut self, source: &Self) -> Result<(), AllocationError> {
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
    pub fn to_mut(&mut self) -> Result<&mut <B as TryToOwned>::Owned, AllocationError> {
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
    pub fn into_owned(self) -> Result<<B as TryToOwned>::Owned, AllocationError> {
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
