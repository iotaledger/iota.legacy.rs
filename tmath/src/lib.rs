#![cfg_attr(test, feature(alloc))]
#![no_std]
extern crate iota_trytes as trytes;


#[cfg(test)]
extern crate alloc;

mod increment;
mod sum;
mod offset;
pub use offset::*;
pub use increment::*;
pub use sum::*;
