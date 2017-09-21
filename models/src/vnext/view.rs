#[allow(dead_code)]
use trytes::*;

use ::{TagView, HashView};
use super::NonceView;

use super::constants::*;
use super::types::*;
use super::builder::TransactionBuilder;

#[derive(Clone, Eq, PartialEq)]
pub struct TransactionView<'a>(&'a [Trit]);

impl<'a> TransactionView<'a> {
    pub fn from_trits(trits: &'a [Trit]) -> Result<Self, TransactionParseError> {
        if trits.len() != TRANSACTION_LEN_TRITS {
            return Err(TransactionParseError::InvalidLength);
        }

        Ok(TransactionView::<'a>(&trits))
    }

    pub fn to_builder(&self) -> TransactionBuilder {
        TransactionBuilder::from_trits(self).unwrap()
    }
}

impl<'a> ::core::ops::Deref for TransactionView<'a> {
    type Target = [Trit];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<'a> Transaction for TransactionView<'a> {
    fn signature_or_message(&self) -> &[Trit] {
        &self.0[0..MESSAGE_TRITS]
    }

    fn extra_data_digest(&self) -> HashView {
        HashView::from_trits(&self.0[EXTRA_DATA_OFFSET..ADDRESS_OFFSET]).unwrap()
    }

    fn address(&self) -> HashView {
        HashView::from_trits(&self.0[ADDRESS_OFFSET..VALUE_OFFSET]).unwrap()
    }

    fn value(&self) -> isize {
        num::trits2int(&self.0[VALUE_OFFSET..ISSUED_AT_OFFSET])
    }

    fn issued_at(&self) -> usize {
        num::trits2int(&self.0[ISSUED_AT_OFFSET..ISSUED_AT_LB_OFFSET]) as usize
    }

    fn issued_at_lb(&self) -> usize {
        num::trits2int(&self.0[ISSUED_AT_LB_OFFSET..ISSUED_AT_UB_OFFSET]) as usize
    }

    fn issued_at_ub(&self) -> usize {
        num::trits2int(&self.0[ISSUED_AT_UB_OFFSET..BUNDLE_NONCE_OFFSET]) as usize
    }

    fn bundle_nonce(&self) -> NonceView {
        NonceView::from_trits(&self.0[BUNDLE_NONCE_OFFSET..TRUNK_OFFSET]).unwrap()
    }

    fn trunk(&self) -> HashView {
        HashView::from_trits(&self.0[TRUNK_OFFSET..BRANCH_OFFSET]).unwrap()
    }

    fn branch(&self) -> HashView {
        HashView::from_trits(&self.0[BRANCH_OFFSET..TAG_OFFSET]).unwrap()
    }

    fn tag(&self) -> TagView {
        TagView::from_trits(&self.0[TAG_OFFSET..ATTACHED_AT_OFFSET]).unwrap()
    }

    fn attached_at(&self) -> usize {
        num::trits2int(&self.0[ATTACHED_AT_OFFSET..ATTACHED_AT_LB_OFFSET]) as usize
    }

    fn attached_at_lb(&self) -> usize {
        num::trits2int(&self.0[ATTACHED_AT_LB_OFFSET..ATTACHED_AT_UB_OFFSET]) as usize
    }

    fn attached_at_ub(&self) -> usize {
        num::trits2int(&self.0[ATTACHED_AT_UB_OFFSET..BUNDLE_NONCE_OFFSET]) as usize
    }

    fn nonce(&self) -> NonceView {
        NonceView::from_trits(&self.0[NONCE_OFFSET..TRANSACTION_LEN_TRITS]).unwrap()
    }
}
