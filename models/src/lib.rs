#![no_std]
#![feature(alloc)]

extern crate alloc;
extern crate iota_trytes as trytes;

pub mod v1;
pub mod v2;

pub use v2::*;
