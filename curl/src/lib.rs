#![feature(alloc)]
#![no_std]
extern crate alloc;

extern crate iota_trytes as trytes;

mod indices;
pub mod constants;
pub mod curl;
pub mod simple;
pub mod pair;
mod tests;

pub use tests::testsuite;
pub use curl::*;

use trytes::BCTrit;
pub type DefaultCurl = Curl<BCTrit>;
