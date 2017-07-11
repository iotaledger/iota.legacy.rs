#![feature(start)]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(link_args)]
#![no_std]
#![crate_type = "staticlib"]

#![cfg(not(test))]
#![feature(core_intrinsics)]

extern crate alloc;
extern crate cty;

extern crate iota_sign;
extern crate iota_trytes;
extern crate iota_curl_cpu;
extern crate iota_curl;

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    use core::intrinsics;
    unsafe {
        intrinsics::abort();
    }
}

pub mod util;
pub mod sign;
pub mod curl;
