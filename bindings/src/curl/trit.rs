use alloc::boxed::Box;
use shared::*;

use curl::iota_curl_cpu::*;
use iota_trytes::*;
use iota_curl::*;

#[no_mangle]
pub fn iota_curl_trit_new() -> *const CpuCurl<Trit> {
    let curl = Box::new(CpuCurl::<Trit>::default());
    Box::into_raw(curl)
}

#[no_mangle]
pub fn iota_curl_trit_delete(c_curl: *mut CpuCurl<Trit>) {
    // Deallocate c_curl
    unsafe { Box::from_raw(c_curl) };
}

#[no_mangle]
pub fn iota_curl_trit_absorb(c_curl: &mut CpuCurl<Trit>, trinary: &CTrits) {
    if trinary.encoding == TritEncoding::TRIT {
        c_curl.absorb(ctrits_slice_trits(trinary));
    } else {
        c_curl.absorb(&ctrits_to_trits(trinary));
    }
}

#[no_mangle]
pub fn iota_curl_trit_reset(c_curl: &mut CpuCurl<Trit>) {
    c_curl.reset();
}

#[no_mangle]
pub fn iota_curl_trit_squeeze(c_curl: &mut CpuCurl<Trit>, trit_count: usize) -> *const CTrits {
    let mut trits = vec![0 as Trit; trit_count];
    c_curl.squeeze(&mut trits);

    let ctrits = Box::new(ctrits_from_trits(trits));
    Box::into_raw(ctrits)
}
