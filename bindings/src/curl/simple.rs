use iota_trytes::*;
use iota_curl::*;
use curl::iota_curl_cpu::*;
use cty::*;
use alloc::Vec;
use alloc::boxed::Box;

use util::c_str_to_static_slice;

#[no_mangle]
pub fn curl_simple_new() -> *mut c_void {
    let curl = Box::new(CpuCurl::<Trit>::default());
    Box::into_raw(curl) as *mut c_void
}

#[no_mangle]
pub fn curl_simple_delete(c_curl: *mut c_void) {
    // Deallocate c_curl
    unsafe { Box::from_raw(c_curl as *mut CpuCurl<Trit>) };
}

#[no_mangle]
pub fn curl_simple_absorb(c_curl: *mut c_void, trinary: *const c_char) {
    let trinary_str = unsafe { c_str_to_static_slice(trinary) };
    let trits: Vec<Trit> = trinary_str.trits();

    let curl: &mut CpuCurl<Trit> = unsafe { &mut *(c_curl as *mut CpuCurl<Trit>) };
    curl.absorb(&trits);
}

#[no_mangle]
pub fn curl_simple_reset(c_curl: *mut c_void) {
    let curl: &mut CpuCurl<Trit> = unsafe { &mut *(c_curl as *mut CpuCurl<Trit>) };
    curl.reset();
}

#[no_mangle]
pub fn curl_simple_squeeze(c_curl: *mut c_void, trit_count: isize) -> *const u8 {
    let curl: &mut CpuCurl<Trit> = unsafe { &mut *(c_curl as *mut CpuCurl<Trit>) };
    let trits = curl.squeeze(trit_count as usize);

    let trinary_str = Box::new(trits_to_string(trits.as_slice()).unwrap() + "\0");

    &trinary_str.as_bytes()[0] as *const u8
}
