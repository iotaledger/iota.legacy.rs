#![feature(alloc)]
#![no_std]
extern crate alloc;
extern crate iota_trytes as trytes;
extern crate iota_curl as curl;
extern crate iota_sign as sign;

#[cfg(feature = "default")]
extern crate iota_curl_cpu as curl_cpu;

pub mod merkle;
pub use merkle::*;
