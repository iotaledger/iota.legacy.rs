#![no_std]
#![feature(fixed_size_array)]
#![cfg_attr(any(test, feature = "alloc"), feature(alloc))]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

extern crate iota_trytes as trytes;
extern crate iota_curl as curl;
extern crate iota_kerl as kerl;

#[macro_use]
extern crate std;

#[macro_use]
mod macros;
mod error;

mod inner;
pub use error::*;
pub use inner::*;

pub mod v1;
pub mod v2;

pub use v1::*;
