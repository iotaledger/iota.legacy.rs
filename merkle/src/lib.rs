#![feature(alloc)]
#![feature(const_fn)]
#![no_std]
extern crate alloc;
extern crate std;
extern crate iota_trytes as trytes;
extern crate iota_tmath as tmath;
extern crate iota_curl as curl;
extern crate iota_sign as sign;

#[cfg(feature = "default")]
extern crate iota_curl_cpu as curl_cpu;
#[cfg(feature = "default")]
mod simple;

//pub mod merkle;
pub use simple::*;
//pub mod createmerkletree;
