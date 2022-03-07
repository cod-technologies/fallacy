//! A library for fallible allocations, collections and operations.

#![feature(allocator_api)]
#![feature(const_fn_trait_bound)]
#![feature(can_vector)]

extern crate alloc as alloc_crate;

pub mod alloc;
pub mod boxed;
pub mod clone;
pub mod cmp;
pub mod collections;
pub mod fmt;
pub mod hash;
pub mod ops;
pub mod result;
pub mod sync;

pub use core::convert;
pub use core::iter;
pub use core::option;
pub use core::slice;
pub use std::io;
