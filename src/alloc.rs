//! Memory allocation & deallocation.

use std::alloc::Layout;
use std::collections::{TryReserveError, TryReserveErrorKind};
use std::error::Error;
use std::fmt;

/// The error type for allocation failure.
#[derive(Debug)]
#[repr(transparent)]
pub struct AllocationError(Layout);

impl AllocationError {
    #[inline]
    pub(crate) const fn new(layout: Layout) -> Self {
        AllocationError(layout)
    }
}

impl Error for AllocationError {}

impl fmt::Display for AllocationError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to allocate memory, required layout {{size: {}, align: {}}}",
            self.0.size(),
            self.0.align()
        )
    }
}

impl From<TryReserveError> for AllocationError {
    #[inline]
    fn from(e: TryReserveError) -> Self {
        match e.kind() {
            TryReserveErrorKind::AllocError { layout, .. } => AllocationError::new(layout),
            TryReserveErrorKind::CapacityOverflow => {
                unreachable!("unexpected capacity overflow occurred while cloning")
            }
        }
    }
}
