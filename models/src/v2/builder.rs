#[allow(dead_code)]

use core::ops::{Deref, DerefMut};
use core::array::FixedSizeArray;

use trytes::*;
use trytes::num;

use ::inner::*;

use super::inner::*;
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
            self.0[0..EXTRA_DATA_OFFSET].clone_from_slice(t);
            Some(self)
        }
    }

    pub fn set_extra_data_digest(&mut self, h: &Hash) -> &mut Self {
        self.0[EXTRA_DATA_OFFSET..ADDRESS_OFFSET].clone_from_slice(h);
        self
    }

    pub fn set_address(&mut self, h: &Hash) -> &mut Self {
        self.0[ADDRESS_OFFSET..VALUE_OFFSET].clone_from_slice(h);
        self
    }

    pub fn set_value(&mut self, v: isize) -> &mut Self {
        num::int2trits(v, &mut self.0[VALUE_OFFSET..ISSUED_AT_OFFSET]);
        self
    }

    pub fn set_issued_at(&mut self, v: usize) -> &mut Self {
        num::int2trits(
            v as isize,
            &mut self.0[ISSUED_AT_OFFSET..ISSUED_AT_LB_OFFSET],
        );
        self
    }

    pub fn set_issued_at_lb(&mut self, v: usize) -> &mut Self {
        num::int2trits(
            v as isize,
            &mut self.0[ISSUED_AT_LB_OFFSET..ISSUED_AT_UB_OFFSET],
        );
        self
    }

    pub fn set_issued_at_ub(&mut self, v: usize) -> &mut Self {
        num::int2trits(
            v as isize,
            &mut self.0[ISSUED_AT_UB_OFFSET..BUNDLE_NONCE_OFFSET],
        );
        self
    }

    pub fn set_bundle_nonce(&mut self, n: &Nonce) -> &mut Self {
        self.0[BUNDLE_NONCE_OFFSET..TRUNK_OFFSET].clone_from_slice(n);
        self
    }

    pub fn set_trunk(&mut self, h: &Hash) -> &mut Self {
        self.0[TRUNK_OFFSET..BRANCH_OFFSET].clone_from_slice(h);
        self
    }

    pub fn set_branch(&mut self, h: &Hash) -> &mut Self {
        self.0[BRANCH_OFFSET..TAG_OFFSET].clone_from_slice(h);
        self
    }

    pub fn set_tag(&mut self, t: &Tag) -> &mut Self {
        self.0[TAG_OFFSET..ATTACHED_AT_OFFSET].clone_from_slice(t);
        self
    }

    pub fn set_attached_at(&mut self, v: usize) -> &mut Self {
        num::int2trits(
            v as isize,
            &mut self.0[ATTACHED_AT_OFFSET..ATTACHED_AT_LB_OFFSET],
        );
        self
    }

    pub fn set_attached_at_lb(&mut self, v: usize) -> &mut Self {
        num::int2trits(
            v as isize,
            &mut self.0[ATTACHED_AT_LB_OFFSET..ATTACHED_AT_UB_OFFSET],
        );
        self
    }

    pub fn set_attached_at_ub(&mut self, v: usize) -> &mut Self {
        num::int2trits(v as isize, &mut self.0[ATTACHED_AT_UB_OFFSET..NONCE_OFFSET]);
        self
    }

    pub fn set_nonce(&mut self, n: &Nonce) -> &mut Self {
        self.0[NONCE_OFFSET..TRANSACTION_LEN_TRITS].clone_from_slice(n);
        self
    }

    pub fn view(&self) -> TransactionView {
        TransactionView::from_trits(self.0.as_slice()).unwrap()
    }
}
