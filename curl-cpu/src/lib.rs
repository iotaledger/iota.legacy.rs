#![feature(alloc)]
#![no_std]
//#![feature(const_fn)]
extern crate alloc;
#[cfg(feature = "parallel")]
extern crate std;

extern crate iota_trytes as trytes;
extern crate iota_tmath as tmath;
extern crate iota_curl as curl;
extern crate hamming;

mod indices;
mod search;
pub mod cpucurl;
pub mod pow;
pub mod ham;
pub mod simple;
pub mod pair;

pub use cpucurl::CpuCurl;

pub type DefaultCurl = CpuCurl<trytes::BCTrit>;
