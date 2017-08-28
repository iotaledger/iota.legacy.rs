use alloc::Vec;
use alloc::boxed::Box;

use iota_trytes::*;
use iota_curl::*;
use iota_curl_cpu::*;

use shared::*;

#[no_mangle]
pub fn iota_curl_bctrit_new() -> *const CpuCurl<BCTrit> {
    let curl = Box::new(CpuCurl::<BCTrit>::default());
    Box::into_raw(curl)
}

#[no_mangle]
pub fn iota_curl_bctrit_delete(c_curl: *mut CpuCurl<BCTrit>) {
    unsafe { Box::from_raw(c_curl) };
}

#[no_mangle]
pub fn iota_curl_bctrit_absorb(c_curl: &mut CpuCurl<BCTrit>, trinary: &CTrits) {
    let trits: Vec<BCTrit> = {
        if trinary.encoding == TritEncoding::TRIT {
            ctrits_slice_trits(trinary)
                .iter()
                .map(|&t| trit_to_bct(t))
                .collect()
        } else {
            ctrits_to_trits(trinary)
                .into_iter()
                .map(trit_to_bct)
                .collect()
        }
    };

    c_curl.absorb(&trits);
}

#[no_mangle]
pub fn iota_curl_bctrit_reset(c_curl: &mut CpuCurl<BCTrit>) {
    c_curl.reset();
}

#[no_mangle]
pub fn iota_curl_bctrit_squeeze(c_curl: &mut CpuCurl<BCTrit>, trit_count: usize) -> *const CTrits {
    let trits: Vec<Trit> = {
        let mut bctrits = vec![(0, 0); trit_count];
        c_curl.squeeze(&mut bctrits);
        bctrits.into_iter().map(bct_to_trit).collect()
    };

    let ctrits = Box::new(ctrits_from_trits(trits));
    Box::into_raw(ctrits)
}
