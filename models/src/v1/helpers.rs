use super::constants::TRANSACTION_LEN_TRITS;

use trytes::Trit;
use curl::*;
use Hash;

#[cfg(feature = "alloc")]
use alloc::Vec;

#[cfg(feature = "alloc")]
pub fn tx_alloc_heap() -> Vec<Trit> {
    [0; TRANSACTION_LEN_TRITS].to_vec()
}

pub fn tx_alloc_stack() -> [Trit; TRANSACTION_LEN_TRITS] {
    [0; TRANSACTION_LEN_TRITS]
}


const TX_HASH_ROUNDS : u8 = 81;
pub fn tx_hash<C: Curl<Trit>>(tx: &[Trit], curl: &mut C) -> Hash {
    let mut hash = Hash::default();
    curl.set_rounds(TX_HASH_ROUNDS);

    curl.absorb(tx);
    curl.squeeze(&mut *hash);

    curl.reset();

    hash
}
