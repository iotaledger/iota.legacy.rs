use trytes::*;

use super::constants::*;

pub fn tx_signature_or_message(trits: &[Trit]) -> &[Trit] {
    &trits[..ADDRESS_OFFSET]
}

pub fn tx_address<'a>(trits: &'a [Trit]) -> &'a [Trit] {
    &trits[ADDRESS_OFFSET..VALUE_OFFSET]
}

pub fn tx_value(trits: &[Trit]) -> i64 {
    num::trits2int(&trits[VALUE_OFFSET..TAG_OFFSET])
}

pub fn tx_tag(trits: &[Trit]) -> &[Trit] {
    &trits[TAG_OFFSET..TIMESTAMP_OFFSET]
}

pub fn tx_timestamp(trits: &[Trit]) -> u64 {
    num::trits2int(&trits[TIMESTAMP_OFFSET..CURRENT_INDEX_OFFSET]) as u64
}

pub fn tx_current_index(trits: &[Trit]) -> u64 {
    num::trits2int(&trits[CURRENT_INDEX_OFFSET..LAST_INDEX_OFFSET]) as u64
}

pub fn tx_last_index(trits: &[Trit]) -> u64 {
    num::trits2int(&trits[LAST_INDEX_OFFSET..BUNDLE_OFFSET]) as u64
}

pub fn tx_bundle(trits: &[Trit]) -> &[Trit] {
    &trits[BUNDLE_OFFSET..TRUNK_OFFSET]
}

pub fn tx_trunk(trits: &[Trit]) -> &[Trit] {
    &trits[TRUNK_OFFSET..BRANCH_OFFSET]
}

pub fn tx_branch(trits: &[Trit]) -> &[Trit] {
    &trits[BRANCH_OFFSET..NONCE_OFFSET]
}

pub fn tx_nonce<'a>(trits: &'a [Trit]) -> &'a [Trit] {
    &trits[NONCE_OFFSET..TRANSACTION_LEN_TRITS]
}

pub fn tx_essence(trits: &[Trit]) -> &[Trit] {
    &trits[ESSENCE_OFFSET..][..ESSENCE_TRITS]
}
