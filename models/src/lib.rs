#![no_std]
#![feature(fixed_size_array)]
#![cfg_attr(any(test, feature = "alloc"), feature(alloc))]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

extern crate iota_trytes as trytes;
extern crate iota_curl as curl;
extern crate iota_kerl as kerl;

#[cfg(test)]
extern crate iota_curl_cpu as curl_cpu;

#[macro_use]
mod macros;
mod error;

mod inner;
pub use error::*;
pub use inner::*;

pub mod v1;
pub mod v2;
pub mod vnext;

pub use v2::*;
