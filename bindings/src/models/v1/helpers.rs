use alloc::boxed::Box;
use iota_curl_cpu::CpuCurl;
use iota_trytes::Trit;
use iota_models::v1;
use shared::*;

#[no_mangle]
pub fn iota_models_v1_tx_hash(tx: &CTrits, curl: &mut CpuCurl<Trit>) -> *const CTrits {
    let slice = ctrits_slice_trits(tx);
    let hash = v1::tx_hash(slice, curl);

    let out = Box::new(ctrits_from_trits(hash.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_models_v1_tx_alloc_heap() -> *const CTrits {
    let tx = v1::tx_alloc_heap();
    let out = Box::new(ctrits_from_trits(tx));
    Box::into_raw(out)
}
