#![feature(alloc)]
#![no_std]
extern crate iota_trytes as trytes;
extern crate alloc;

pub mod increment;
pub use increment::*;
