#![feature(alloc)]
#![no_std]
extern crate alloc;

extern crate iota_trytes as trytes;
extern crate iota_tmath as tmath;
extern crate iota_curl as curl;

mod indices;
mod search;
pub mod cpucurl;
pub mod pow;
pub mod simple;
pub mod pair;

pub use cpucurl::CpuCurl;

pub type DefaultCurl = CpuCurl<trytes::BCTrit>;
