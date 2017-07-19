use core::array::FixedSizeArray;
use core::ops::{Deref, DerefMut};

use trytes::*;

use ::{Hash, Tag, HASH_LEN_TRITS};
use super::Nonce;

use super::view::TransactionView;
use super::constants::*;
use super::types::*;


pub struct TransactionBuilder([Trit; TRANSACTION_LEN_TRITS]);

impl Clone for TransactionBuilder {
    fn clone(&self) -> TransactionBuilder {
        TransactionBuilder(self.0)
    }
}

impl PartialEq for TransactionBuilder {
    fn eq(&self, other: &TransactionBuilder) -> bool {
        self.0.as_slice() == other.0.as_slice()
    }

    fn ne(&self, other: &TransactionBuilder) -> bool {
        self.0.as_slice() != other.0.as_slice()
    }
}

impl Eq for TransactionBuilder {}

impl Default for TransactionBuilder {
    fn default() -> Self {
        TransactionBuilder([0; TRANSACTION_LEN_TRITS])
    }
}

impl Deref for TransactionBuilder {
    type Target = [Trit];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl DerefMut for TransactionBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl TransactionBuilder {
    pub fn from_trits(base: &[Trit]) -> Result<Self, TransactionParseError> {
        if base.len() != HASH_LEN_TRITS {
            return Err(TransactionParseError::InvalidLength);
        }

        let mut builder = TransactionBuilder([0; TRANSACTION_LEN_TRITS]);
        builder.0.clone_from_slice(base);
        Ok(builder)
    }

    pub fn set_signature_or_message(&mut self, t: &[Trit]) -> Option<&mut Self> {
        if t.len() != MESSAGE_TRITS {
            None
        } else {
            self.0[0..ADDRESS_OFFSET].clone_from_slice(t);
            Some(self)
        }
    }

    pub fn set_address(&mut self, h: &Hash) -> &mut Self {
        self.0[ADDRESS_OFFSET..VALUE_OFFSET].clone_from_slice(h);
        self
    }

    pub fn set_value(&mut self, v: isize) -> &mut Self {
        num::int2trits(v, &mut self.0[VALUE_OFFSET..TAG_OFFSET]);
        self
    }

    pub fn set_tag(&mut self, t: &Tag) -> &mut Self {
        &self.0[TAG_OFFSET..TIMESTAMP_OFFSET].clone_from_slice(t);
        self
    }

    pub fn set_timestamp(&mut self, t: usize) -> &mut Self {
        num::int2trits(
            t as isize,
            &mut self.0[TIMESTAMP_OFFSET..CURRENT_INDEX_OFFSET],
        );
        self
    }

    pub fn set_current_index(&mut self, idx: usize) -> &mut Self {
        num::int2trits(
            idx as isize,
            &mut self.0[CURRENT_INDEX_OFFSET..LAST_INDEX_OFFSET],
        );
        self
    }

    pub fn set_last_index(&mut self, idx: usize) -> &mut Self {
        num::int2trits(idx as isize, &mut self.0[LAST_INDEX_OFFSET..BUNDLE_OFFSET]);
        self
    }

    pub fn set_bundle(&mut self, h: &Hash) -> &mut Self {
        self.0[BUNDLE_OFFSET..TRUNK_OFFSET].clone_from_slice(h);
        self
    }

    pub fn set_trunk(&mut self, h: &Hash) -> &mut Self {
        self.0[TRUNK_OFFSET..BRANCH_OFFSET].clone_from_slice(h);
        self
    }

    pub fn set_branch(&mut self, h: &Hash) -> &mut Self {
        self.0[BRANCH_OFFSET..NONCE_OFFSET].clone_from_slice(h);
        self
    }

    pub fn set_nonce(&mut self, h: &Nonce) -> &mut Self {
        self.0[NONCE_OFFSET..TRANSACTION_LEN_TRITS].clone_from_slice(h);
        self
    }

    pub fn view(&self) -> TransactionView {
        TransactionView::from_trits(self.0.as_slice()).unwrap()
    }
}
