#![no_std]
#![cfg_attr(test, feature(alloc))]

#[cfg(test)]
#[macro_use]
extern crate alloc;

extern crate iota_trytes as trytes;
extern crate iota_tmath as tmath;
extern crate iota_curl as curl;
extern crate iota_kerl as kerl;

#[cfg(test)]
extern crate iota_curl_cpu as curl_cpu;


pub mod iss;
pub mod checksum;

pub use checksum::*;
