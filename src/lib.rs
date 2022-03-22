//! A library for fallible allocations, collections and operations.

#![feature(allocator_api)]
#![feature(can_vector)]
#![feature(fmt_internals)]
#![feature(min_specialization)]

pub mod borrow;
pub mod boxed;
pub mod collections;
pub mod fmt;
pub mod string;
pub mod sync;
pub mod vec;

/// Memory allocation & deallocation.
pub mod alloc {
    pub use fallacy_alloc::AllocError;
}

/// The `TryClone` trait for types that cannot be 'implicitly copied'.
pub mod clone {
    pub use fallacy_clone::TryClone;
}
