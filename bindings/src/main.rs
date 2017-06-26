#![feature(start)]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(core_intrinsics)]
#![no_std]

extern crate alloc;

extern crate cty;

extern crate iota_sign;
extern crate iota_trytes;
extern crate iota_curl;

use core::intrinsics;

pub mod util;
pub mod sign;
pub mod curl;

#[cfg(any(target_os = "emscripten", target_arch="wasm32"))]
#[link_args = "-s EXPORTED_FUNCTIONS=['_curl_pair_new','_curl_pair_absorb','_curl_pair_squeeze','_curl_pair_reset','_curl_pair_delete'\
,'_curl_simple_new','_curl_simple_absorb','_curl_simple_squeeze','_curl_simple_reset','_curl_simple_delete'\
,'_subseed','_key','_digest_key','_address','_signature','_digest_bundle_signature'\
]"]
extern "C" {}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn rust_eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
) -> ! {
    unsafe { intrinsics::abort() }
}

#[start]
pub fn main(_: isize, _: *const *const u8) -> isize {
    0
}
