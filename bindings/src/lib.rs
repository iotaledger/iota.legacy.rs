#![feature(alloc)]
#![no_std]

#[macro_use]
extern crate alloc;

extern crate cty;

extern crate iota_sign;
extern crate iota_trytes;
extern crate iota_curl_cpu;
extern crate iota_curl;

mod util;
pub mod sign;
pub mod curl;
