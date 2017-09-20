use super::constants::TRANSACTION_LEN_TRITS;

use trytes::Trit;

#[cfg(feature = "alloc")]
use alloc::Vec;

#[cfg(feature = "alloc")]
pub fn tx_alloc_heap() -> Vec<Trit> {
    [0; TRANSACTION_LEN_TRITS].to_vec()
}

pub fn tx_alloc_stack() -> [Trit; TRANSACTION_LEN_TRITS] {
    [0; TRANSACTION_LEN_TRITS]
}
