//! Memory allocation & deallocation.

pub use std::alloc::{AllocError, Allocator, Global, Layout, LayoutError};

use std::collections::{TryReserveError, TryReserveErrorKind};
use std::error::Error;
use std::fmt;

/// The error type for allocation failure.
#[derive(Debug)]
pub enum AllocationError {
    AllocError(Layout),
    CapacityOverflow(usize),
}

impl Error for AllocationError {}

impl fmt::Display for AllocationError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocationError::AllocError(layout) => {
                write!(
                    f,
                    "failed to allocate memory, required layout {{size: {}, align: {}}}",
                    layout.size(),
                    layout.align()
                )
            }
            AllocationError::CapacityOverflow(cap) => {
                write!(f, "the computed capacity exceeded the collection's maximum({})", cap)
            }
        }
    }
}

impl AllocationError {
    #[inline]
    pub(crate) fn from_try_reserve_error(e: TryReserveError, cap: usize) -> Self {
        match e.kind() {
            TryReserveErrorKind::CapacityOverflow => AllocationError::CapacityOverflow(cap),
            TryReserveErrorKind::AllocError { layout, .. } => AllocationError::AllocError(layout),
        }
    }
}
