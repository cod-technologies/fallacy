//! The `Clone` trait for types that cannot be 'implicitly copied'.

use crate::alloc::AllocationError;
pub use core::clone::Clone;

/// Tries to clone, return an error instead of panic if allocation failed.
pub trait TryClone: Sized {
    fn try_clone(&self) -> Result<Self, AllocationError>;
}

macro_rules! impl_try_clone {
    ($($val: ty),*) => {
        $(impl TryClone for $val {
            #[inline(always)]
            fn try_clone(&self) -> Result<Self, AllocationError> {
                Ok(*self)
            }
        })*
    }
}

impl_try_clone!(bool, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize);

impl<T: TryClone> TryClone for Option<T> {
    #[inline]
    fn try_clone(&self) -> Result<Self, AllocationError> {
        Ok(match self {
            Some(t) => Some(t.try_clone()?),
            None => None,
        })
    }
}
