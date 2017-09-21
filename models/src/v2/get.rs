use trytes::*;
use super::constants::*;

pub fn tx_signature_or_message(trits: &[Trit]) -> &[Trit] {
    &trits[..ADDRESS_OFFSET]
}

pub fn tx_address<'a>(trits: &'a [Trit]) -> &'a [Trit] {
    &trits[ADDRESS_OFFSET..VALUE_OFFSET]
}

pub fn tx_value(trits: &[Trit]) -> isize {
    num::trits2int(&trits[VALUE_OFFSET..OBSOLETE_TAG_OFFSET])
}

pub fn tx_obsolete_tag(trits: &[Trit]) -> &[Trit] {
    &trits[OBSOLETE_TAG_OFFSET..TIMESTAMP_OFFSET]
}

pub fn tx_timestamp(trits: &[Trit]) -> usize {
    num::trits2int(&trits[TIMESTAMP_OFFSET..CURRENT_INDEX_OFFSET]) as usize
}

pub fn tx_current_index(trits: &[Trit]) -> usize {
    num::trits2int(&trits[CURRENT_INDEX_OFFSET..LAST_INDEX_OFFSET]) as usize
}

pub fn tx_last_index(trits: &[Trit]) -> usize {
    num::trits2int(&trits[LAST_INDEX_OFFSET..BUNDLE_OFFSET]) as usize
}

pub fn tx_bundle(trits: &[Trit]) -> &[Trit] {
    &trits[BUNDLE_OFFSET..TRUNK_OFFSET]
}

pub fn tx_trunk(trits: &[Trit]) -> &[Trit] {
    &trits[TRUNK_OFFSET..BRANCH_OFFSET]
}

pub fn tx_branch(trits: &[Trit]) -> &[Trit] {
    &trits[BRANCH_OFFSET..TAG_OFFSET]
}

pub fn tx_tag(trits: &[Trit]) -> &[Trit] {
    &trits[TAG_OFFSET..ATTACHMENT_TIMESTAMP_OFFSET]
}

pub fn tx_attachment_timestamp(trits: &[Trit]) -> usize {
    num::trits2int(
        &trits[ATTACHMENT_TIMESTAMP_OFFSET..ATTACHMENT_TIMESTAMP_LOWER_OFFSET],
    ) as usize
}

pub fn tx_attachment_timestamp_lower(trits: &[Trit]) -> usize {
    num::trits2int(
        &trits[ATTACHMENT_TIMESTAMP_LOWER_OFFSET..ATTACHMENT_TIMESTAMP_UPPER_OFFSET],
    ) as usize
}

pub fn tx_attachment_timestamp_upper(trits: &[Trit]) -> usize {
    num::trits2int(&trits[ATTACHMENT_TIMESTAMP_UPPER_OFFSET..NONCE_OFFSET]) as usize
}

pub fn tx_nonce<'a>(trits: &'a [Trit]) -> &'a [Trit] {
    &trits[NONCE_OFFSET..TRANSACTION_LEN_TRITS]
}

pub fn tx_essence(trits: &[Trit]) -> &[Trit] {
    &trits[ESSENCE_OFFSET..][..ESSENCE_TRITS]
}
