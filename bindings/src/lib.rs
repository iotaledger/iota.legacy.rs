#![feature(start)]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(link_args)]
#![no_std]
#![crate_type = "staticlib"]

#![cfg(not(test))]
#![feature(core_intrinsics)]

#[macro_use]
extern crate alloc;

extern crate cty;

extern crate iota_sign;
extern crate iota_trytes;
extern crate iota_curl_cpu;
extern crate iota_curl;

#[cfg(any(target_os = "emscripten", target_arch = "wasm32"))]
#[link_args = "-s EXPORTED_FUNCTIONS=['_curl_pair_new','_curl_pair_absorb','_curl_pair_squeeze','_curl_pair_reset','_curl_pair_delete'\
,'_curl_simple_new','_curl_simple_absorb','_curl_simple_squeeze','_curl_simple_reset','_curl_simple_delete'\
,'_subseed','_key','_digest_key','_address','_signature','_digest_bundle_signature'\
]"]

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
