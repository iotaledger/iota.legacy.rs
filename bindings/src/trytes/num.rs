use alloc::boxed::Box;
use iota_curl_cpu::CpuCurl;
use iota_trytes::num::*;
use shared::*;

#[no_mangle]
pub fn iota_trytes_num_trits2int(ctrits: &CTrits) -> i64 {
    let slice = ctrits_slice_trits(ctrits);
    trits2int(slice)
}

#[no_mangle]
pub fn iota_trytes_num_int2trits(v: i64, ctrits: &mut CTrits) {
    let slice = ctrits_slice_trits_mut(ctrits);
    int2trits(v, slice);
}
