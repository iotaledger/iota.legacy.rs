#![no_std]
#![feature(fixed_size_array)]
#![cfg_attr(any(test, feature = "alloc"), feature(alloc))]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

extern crate iota_trytes as trytes;

#[macro_use]
mod macros;

mod inner;
pub mod v1;
pub mod v2;

pub use v2::*;
