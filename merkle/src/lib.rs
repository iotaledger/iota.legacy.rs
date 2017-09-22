#![no_std]
#![feature(alloc)]
#![feature(const_fn)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate alloc;

extern crate iota_trytes as trytes;
extern crate iota_tmath as tmath;
extern crate iota_curl as curl;
extern crate iota_sign as sign;

extern crate iota_curl_cpu as curl_cpu;
mod simple;

pub use simple::*;
