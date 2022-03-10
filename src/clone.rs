//! The `Clone` trait for types that cannot be 'implicitly copied'.

pub use std::clone::Clone;

use crate::alloc::AllocationError;
use std::alloc::Layout;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

/// Error occurred while cloning.
#[derive(Debug)]
pub enum CloneError {
    AllocError(Layout),
}

impl Error for CloneError {}

impl fmt::Display for CloneError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CloneError::AllocError(layout) => {
                write!(
                    f,
                    "failed to clone because of memory allocation failure, required layout {{size: {}, align: {}}}",
                    layout.size(),
                    layout.align()
                )
            }
        }
    }
}

impl From<AllocationError> for CloneError {
    #[inline]
    fn from(e: AllocationError) -> Self {
        match e {
            AllocationError::AllocError(layout) => CloneError::AllocError(layout),
            AllocationError::CapacityOverflow(_) => unreachable!("unexpected capacity overflow occurred while cloning"),
        }
    }
}

/// Tries to clone, return an error instead of panic if allocation failed.
pub trait TryClone: Sized {
    fn try_clone(&self) -> Result<Self, CloneError>;
}

macro_rules! impl_try_clone {
    ($($val: ty),*) => {
        $(impl TryClone for $val {
            #[inline(always)]
            fn try_clone(&self) -> Result<Self, CloneError> {
                Ok(*self)
            }
        })*
    }
}

impl_try_clone!(bool, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize);

impl<T: TryClone> TryClone for Option<T> {
    #[inline]
    fn try_clone(&self) -> Result<Self, CloneError> {
        Ok(match self {
            Some(t) => Some(t.try_clone()?),
            None => None,
        })
    }
}
