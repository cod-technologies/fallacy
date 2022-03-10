//! A library for fallible allocations, collections and operations.

#![feature(allocator_api)]
#![feature(can_vector)]
#![feature(try_reserve_kind)]

pub mod alloc;
pub mod boxed;
pub mod clone;
pub mod collections;
pub mod fmt;
pub mod string;
pub mod sync;
pub mod vec;

#[cfg(feature = "export-std")]
pub use std::any;
#[cfg(feature = "export-std")]
pub use std::cmp;
#[cfg(feature = "export-std")]
pub use std::convert;
#[cfg(feature = "export-std")]
pub use std::error;
#[cfg(feature = "export-std")]
pub use std::hash;
#[cfg(feature = "export-std")]
pub use std::io;
#[cfg(feature = "export-std")]
pub use std::iter;
#[cfg(feature = "export-std")]
pub use std::mem;
#[cfg(feature = "export-std")]
pub use std::ops;
#[cfg(feature = "export-std")]
pub use std::option;
#[cfg(feature = "export-std")]
pub use std::ptr;
#[cfg(feature = "export-std")]
pub use std::result;
#[cfg(feature = "export-std")]
pub use std::slice;
#[cfg(feature = "export-std")]
pub use std::str;
