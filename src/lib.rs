//! A library for fallible allocations, collections and operations.

#![feature(allocator_api)]
#![feature(can_vector)]
#![feature(fmt_internals)]
#![feature(min_specialization)]
#![feature(unicode_internals)]

extern crate core;

pub mod borrow;
pub mod collections;
pub mod fmt;
pub mod prelude;
pub mod str;
pub mod string;
pub mod sync;
pub mod vec;

/// Memory allocation & deallocation.
pub mod alloc {
    pub use fallacy_alloc::AllocError;
    pub use std::alloc::{Allocator, Global, Layout};
}

/// The `TryClone` trait for types that cannot be 'implicitly copied'.
pub mod clone {
    pub use fallacy_clone::TryClone;
}

/// A pointer type for heap allocation.
pub mod boxed {
    pub use fallacy_box::Box;
}

mod sealed {
    pub trait Sealed {}
}
