use trytes::*;

use {HashView, TagView};
use super::NonceView;

use super::constants::*;

#[inline]
pub fn tx_set_signature_or_message(tx: &mut [Trit], t: &[Trit]) {
    let subslice = &mut tx[0..ADDRESS_OFFSET];
    let tlen = t.len();

    subslice[..tlen].clone_from_slice(t);

    if tlen != ADDRESS_OFFSET {
        for i in subslice[tlen..].iter_mut() {
            *i = 0;
        }
    }
}

#[inline]
pub fn tx_set_address(tx: &mut [Trit], h: &HashView) {
    tx[ADDRESS_OFFSET..VALUE_OFFSET].clone_from_slice(h);
}

#[inline]
pub unsafe fn tx_set_address_raw(tx: &mut [Trit], h: &[Trit]) {
    tx[ADDRESS_OFFSET..VALUE_OFFSET].clone_from_slice(h);
}

#[inline]
pub fn tx_set_value(tx: &mut [Trit], v: isize) {
    num::int2trits(v, &mut tx[VALUE_OFFSET..TAG_OFFSET]);
}

#[inline]
pub fn tx_set_obsolete_tag(tx: &mut [Trit], t: &TagView) {
    tx[OBSOLETE_TAG_OFFSET..TIMESTAMP_OFFSET].clone_from_slice(t);
}

#[inline]
pub unsafe fn tx_set_obsolete_tag_raw(tx: &mut [Trit], t: &[Trit]) {
    tx[OBSOLETE_TAG_OFFSET..TIMESTAMP_OFFSET].clone_from_slice(t);
}

#[inline]
pub fn tx_set_timestamp(tx: &mut [Trit], t: usize) {
    num::int2trits(t as isize, &mut tx[TIMESTAMP_OFFSET..CURRENT_INDEX_OFFSET]);
}

#[inline]
pub fn tx_set_current_index(tx: &mut [Trit], idx: usize) {
    num::int2trits(
        idx as isize,
        &mut tx[CURRENT_INDEX_OFFSET..LAST_INDEX_OFFSET],
    );
}

#[inline]
pub fn tx_set_last_index(tx: &mut [Trit], idx: usize) {
    num::int2trits(idx as isize, &mut tx[LAST_INDEX_OFFSET..BUNDLE_OFFSET]);
}

#[inline]
pub fn tx_set_bundle(tx: &mut [Trit], h: &HashView) {
    tx[BUNDLE_OFFSET..TRUNK_OFFSET].clone_from_slice(h);
}

#[inline]
pub unsafe fn tx_set_bundle_raw(tx: &mut [Trit], h: &[Trit]) {
    tx[BUNDLE_OFFSET..TRUNK_OFFSET].clone_from_slice(h);
}

#[inline]
pub fn tx_set_trunk(tx: &mut [Trit], h: &HashView) {
    tx[TRUNK_OFFSET..BRANCH_OFFSET].clone_from_slice(h);
}

#[inline]
pub unsafe fn tx_set_trunk_raw(tx: &mut [Trit], h: &[Trit]) {
    tx[TRUNK_OFFSET..BRANCH_OFFSET].clone_from_slice(h);
}


#[inline]
pub fn tx_set_branch(tx: &mut [Trit], h: &HashView) {
    tx[BRANCH_OFFSET..TAG_OFFSET].clone_from_slice(h);
}

#[inline]
pub unsafe fn tx_set_branch_raw(tx: &mut [Trit], h: &[Trit]) {
    tx[BRANCH_OFFSET..TAG_OFFSET].clone_from_slice(h);
}


#[inline]
pub fn tx_set_tag(tx: &mut [Trit], t: &TagView) {
    tx[TAG_OFFSET..ATTACHMENT_TIMESTAMP_OFFSET].clone_from_slice(t);
}

#[inline]
pub unsafe fn tx_set_tag_raw(tx: &mut [Trit], t: &[Trit]) {
    tx[TAG_OFFSET..ATTACHMENT_TIMESTAMP_OFFSET].clone_from_slice(t);
}

#[inline]
pub fn tx_set_attachment_timestamp(tx: &mut [Trit], timestamp: usize) {
    num::int2trits(
        timestamp as isize,
        &mut tx[ATTACHMENT_TIMESTAMP_OFFSET..ATTACHMENT_TIMESTAMP_LOW_OFFSET],
    );
}

#[inline]
pub fn tx_set_attachment_timestamp_low(tx: &mut [Trit], timestamp: usize) {
    num::int2trits(
        timestamp as isize,
        &mut tx[ATTACHMENT_TIMESTAMP_LOW_OFFSET..ATTACHMENT_TIMESTAMP_HIGH_OFFSET],
    );
}

#[inline]
pub fn tx_set_attachment_timestamp_high(tx: &mut [Trit], timestamp: usize) {
    num::int2trits(
        timestamp as isize,
        &mut tx[ATTACHMENT_TIMESTAMP_HIGH_OFFSET..NONCE_OFFSET],
    );
}

#[inline]
pub fn tx_set_nonce(tx: &mut [Trit], h: &NonceView) {
    tx[NONCE_OFFSET..TRANSACTION_LEN_TRITS].clone_from_slice(h);
}

#[inline]
pub unsafe fn tx_set_nonce_raw(tx: &mut [Trit], h: &[Trit]) {
    tx[NONCE_OFFSET..TRANSACTION_LEN_TRITS].clone_from_slice(h);
}
