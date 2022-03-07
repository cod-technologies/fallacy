//! A library for fallible allocations, collections and operations.

#![feature(allocator_api)]

extern crate alloc as alloc_crate;

pub mod alloc;
pub mod boxed;
pub mod clone;
pub mod cmp;
pub mod fmt;
pub mod hash;
pub mod ops;
pub mod result;
pub mod sync;

pub use core::convert;
