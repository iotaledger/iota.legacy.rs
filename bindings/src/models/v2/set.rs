use alloc::Vec;
use alloc::boxed::Box;
use core::slice;
use shared::*;

use iota_kerl::Kerl;
use iota_models::v2;

#[no_mangle]
pub fn iota_models_v2_tx_set_signature_or_message(tx: &mut CTrits, ctrits: &CTrits) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    let in_slice = ctrits_slice_trits(ctrits);

    v2::set::tx_set_signature_or_message(tx_slice, in_slice);
}

#[no_mangle]
pub unsafe fn iota_models_v2_tx_set_address(tx: &mut CTrits, ctrits: &CTrits) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    let in_slice = ctrits_slice_trits(ctrits);

    v2::set::tx_set_address_raw(tx_slice, in_slice);
}

#[no_mangle]
pub fn iota_models_v2_tx_set_value(tx: &mut CTrits, v: i64) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    v2::set::tx_set_value(tx_slice, v);
}

#[no_mangle]
pub unsafe fn iota_models_v2_tx_set_obsolete_tag(tx: &mut CTrits, ctrits: &CTrits) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    let in_slice = ctrits_slice_trits(ctrits);

    v2::set::tx_set_obsolete_tag_raw(tx_slice, in_slice);
}

#[no_mangle]
pub fn iota_models_v2_tx_set_timestamp(tx: &mut CTrits, timestamp: u64) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    v2::set::tx_set_timestamp(tx_slice, timestamp);
}

#[no_mangle]
pub fn iota_models_v2_tx_set_current_index(tx: &mut CTrits, index: u64) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    v2::set::tx_set_current_index(tx_slice, index);
}

#[no_mangle]
pub fn iota_models_v2_tx_set_last_index(tx: &mut CTrits, index: u64) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    v2::set::tx_set_current_index(tx_slice, index);
}

#[no_mangle]
pub unsafe fn iota_models_v2_tx_set_bundle(tx: &mut CTrits, ctrits: &CTrits) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    let in_slice = ctrits_slice_trits(ctrits);

    v2::set::tx_set_bundle_raw(tx_slice, in_slice);
}

#[no_mangle]
pub unsafe fn iota_models_v2_tx_set_trunk(tx: &mut CTrits, ctrits: &CTrits) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    let in_slice = ctrits_slice_trits(ctrits);

    v2::set::tx_set_trunk_raw(tx_slice, in_slice);
}

#[no_mangle]
pub unsafe fn iota_models_v2_tx_set_branch(tx: &mut CTrits, ctrits: &CTrits) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    let in_slice = ctrits_slice_trits(ctrits);

    v2::set::tx_set_branch_raw(tx_slice, in_slice);
}

#[no_mangle]
pub unsafe fn iota_models_v2_tx_set_tag(tx: &mut CTrits, ctrits: &CTrits) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    let in_slice = ctrits_slice_trits(ctrits);

    v2::set::tx_set_tag_raw(tx_slice, in_slice);
}

#[no_mangle]
pub fn iota_models_v2_tx_set_attachment_timestamp(tx: &mut CTrits, timestamp: u64) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    v2::set::tx_set_attachment_timestamp(tx_slice, timestamp);
}

#[no_mangle]
pub fn iota_models_v2_tx_set_attachment_timestamp_lower(tx: &mut CTrits, timestamp: u64) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    v2::set::tx_set_attachment_timestamp_lower(tx_slice, timestamp);
}

#[no_mangle]
pub fn iota_models_v2_tx_set_attachment_timestamp_upper(tx: &mut CTrits, timestamp: u64) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    v2::set::tx_set_attachment_timestamp_upper(tx_slice, timestamp);
}

#[no_mangle]
pub unsafe fn iota_models_v2_tx_set_nonce(tx: &mut CTrits, ctrits: &CTrits) {
    let tx_slice = ctrits_slice_trits_mut(tx);
    let in_slice = ctrits_slice_trits(ctrits);

    v2::set::tx_set_nonce_raw(tx_slice, in_slice);
}
