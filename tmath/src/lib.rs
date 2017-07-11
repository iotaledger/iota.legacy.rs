#![feature(alloc)]
#![no_std]
extern crate iota_trytes as trytes;
extern crate alloc;

mod increment;
mod sum;
pub use increment::*;
pub use sum::*;
