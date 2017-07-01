use cty::*;
use alloc::Vec;
use alloc::string::ToString;
use alloc::boxed::Box;

use iota_trytes::*;
use iota_curl::*;
use curl::iota_curl_cpu::*;

use util::c_str_to_static_slice;

#[no_mangle]
pub fn curl_pair_new() -> *mut c_void {
    let curl = Box::new(CpuCurl::<BCTrit>::default());
    Box::into_raw(curl) as *mut c_void
}

#[no_mangle]
pub fn curl_pair_delete(c_curl: *mut c_void) {
    unsafe { Box::from_raw(c_curl as *mut CpuCurl<BCTrit>) };
}

#[no_mangle]
pub fn curl_pair_absorb(c_curl: *mut c_void, trinary: *const c_char) {
    let trinary_str = unsafe { c_str_to_static_slice(trinary) };
    let trinary: Trinary = trinary_str.chars().collect();

    let curl: &mut CpuCurl<BCTrit>= unsafe { &mut *(c_curl as *mut CpuCurl<BCTrit>) };
    let trits: Vec<BCTrit> = trinary.trits();
    curl.absorb(trits.as_slice());
}

#[no_mangle]
pub fn curl_pair_reset(c_curl: *mut c_void) {
    let curl: &mut CpuCurl<BCTrit>= unsafe { &mut *(c_curl as *mut CpuCurl<BCTrit>) };
    curl.reset();
}

#[no_mangle]
pub fn curl_pair_squeeze(c_curl: *mut c_void, trit_count: isize) -> *const u8 {
    let curl: &mut CpuCurl<BCTrit>= unsafe { &mut *(c_curl as *mut CpuCurl<BCTrit>) };
    let trits = curl.squeeze(trit_count as usize);

    let trinary: Trinary = trits.into_iter().collect();
    let trinary_str = Box::new(trinary.to_string() + "\0");

    &trinary_str.as_bytes()[0] as *const u8
}
