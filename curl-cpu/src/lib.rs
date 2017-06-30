#![feature(alloc)]
#![no_std]
extern crate alloc;

extern crate iota_trytes as trytes;
extern crate iota_curl as curl;

mod indices;
pub mod cpucurl;
pub mod simple;
pub mod pair;
mod tests;

pub use tests::testsuite;
pub use cpucurl::CpuCurl;

use trytes::BCTrit;
pub type DefaultCurl = CpuCurl<BCTrit>;
