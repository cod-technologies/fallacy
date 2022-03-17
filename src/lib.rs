//! A library for fallible allocations, collections and operations.

#![feature(allocator_api)]
#![feature(can_vector)]
#![feature(try_reserve_kind)]
#![feature(fmt_internals)]
#![feature(min_specialization)]

pub mod alloc;
pub mod borrow;
pub mod boxed;
pub mod clone;
pub mod collections;
pub mod fmt;
pub mod string;
pub mod sync;
pub mod vec;
