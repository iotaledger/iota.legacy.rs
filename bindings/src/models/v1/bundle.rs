use alloc::Vec;
use alloc::boxed::Box;
use core::slice;
use shared::*;

use iota_kerl::Kerl;
use iota_models::v1;

#[no_mangle]
pub fn iota_tx_v1_tx_signature_or_message(ctrits: &CTrits) -> *const CTrits {
    let slice = v1::get::tx_signature_or_message(ctrits_slice_trits(ctrits));
    let out = Box::new(ctrits_from_trits(slice.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_tx_v1_tx_address(ctrits: &CTrits) -> *const CTrits {
    let slice = v1::get::tx_address(ctrits_slice_trits(ctrits));
    let out = Box::new(ctrits_from_trits(slice.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_tx_v1_tx_value(ctrits: &CTrits) -> isize {
    v1::get::tx_value(ctrits_slice_trits(ctrits))
}

#[no_mangle]
pub fn iota_tx_v1_tx_tag(ctrits: &CTrits) -> *const CTrits {
    let slice = v1::get::tx_tag(ctrits_slice_trits(ctrits));
    let out = Box::new(ctrits_from_trits(slice.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_tx_v1_tx_timestamp(ctrits: &CTrits) -> usize {
    v1::get::tx_timestamp(ctrits_slice_trits(ctrits))
}

#[no_mangle]
pub fn iota_tx_v1_tx_current_index(ctrits: &CTrits) -> usize {
    v1::get::tx_current_index(ctrits_slice_trits(ctrits))
}

#[no_mangle]
pub fn iota_tx_v1_tx_last_index(ctrits: &CTrits) -> usize {
    v1::get::tx_last_index(ctrits_slice_trits(ctrits))
}

#[no_mangle]
pub fn iota_tx_v1_tx_bundle(ctrits: &CTrits) -> *const CTrits {
    let slice = v1::get::tx_bundle(ctrits_slice_trits(ctrits));
    let out = Box::new(ctrits_from_trits(slice.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_tx_v1_tx_trunk(ctrits: &CTrits) -> *const CTrits {
    let slice = v1::get::tx_trunk(ctrits_slice_trits(ctrits));
    let out = Box::new(ctrits_from_trits(slice.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_tx_v1_tx_branch(ctrits: &CTrits) -> *const CTrits {
    let slice = v1::get::tx_branch(ctrits_slice_trits(ctrits));
    let out = Box::new(ctrits_from_trits(slice.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_tx_v1_tx_nonce(ctrits: &CTrits) -> *const CTrits {
    let slice = v1::get::tx_nonce(ctrits_slice_trits(ctrits));
    let out = Box::new(ctrits_from_trits(slice.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_tx_v1_tx_essence(ctrits: &CTrits) -> *const CTrits {
    let slice = v1::get::tx_essence(ctrits_slice_trits(ctrits));
    let out = Box::new(ctrits_from_trits(slice.to_vec()));
    Box::into_raw(out)
}
