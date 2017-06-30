#![feature(alloc)]
#![no_std]
extern crate alloc;

extern crate iota_trytes as trytes;

pub mod constants;
pub mod curl;
mod tests;

pub use tests::testsuite;
pub use curl::*;
