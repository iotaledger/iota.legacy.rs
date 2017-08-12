#![cfg_attr(test, feature(alloc))]
#![no_std]

#[cfg(feature = "parallel")]
#[macro_use]
extern crate std;

#[cfg(feature = "parallel")]
extern crate num_cpus;

#[cfg(test)]
extern crate alloc;

extern crate iota_trytes as trytes;
extern crate iota_tmath as tmath;
extern crate iota_curl as curl;

#[cfg(test)]
extern crate iota_curl_tests as curl_tests;

mod indices;
mod search;
pub mod copy;
pub mod cpucurl;
pub mod pow;
pub mod ham;
pub mod simple;
pub mod pair;

pub use cpucurl::CpuCurl;

pub type DefaultCurl = CpuCurl<trytes::BCTrit>;
pub type SimpleCurl = CpuCurl<trytes::BCTrit>;
pub use ham::CpuHam;
