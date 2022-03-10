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

pub use std::any;
pub use std::cmp;
pub use std::convert;
pub use std::error;
pub use std::hash;
pub use std::io;
pub use std::iter;
pub use std::mem;
pub use std::ops;
pub use std::option;
pub use std::ptr;
pub use std::result;
pub use std::slice;
pub use std::str;
