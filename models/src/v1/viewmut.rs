use core::ops::{Deref, DerefMut};

use trytes::*;

use {HashView, TagView};
use super::NonceView;

use super::view::TransactionView;
use super::constants::*;
use super::types::*;

use super::set::*;


#[derive(Eq, PartialEq)]
pub struct TransactionViewMut<'a>(&'a mut [Trit]);

impl<'a> Deref for TransactionViewMut<'a> {
    type Target = [Trit];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> DerefMut for TransactionViewMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a> TransactionViewMut<'a> {
    pub fn from_trits(base: &'a mut [Trit]) -> Result<Self, TransactionParseError> {
        if base.len() != TRANSACTION_LEN_TRITS {
            return Err(TransactionParseError::InvalidLength);
        }

        Ok(TransactionViewMut(base))
    }

    pub unsafe fn from_trits_raw(base: &'a mut [Trit]) -> Self {
        TransactionViewMut(base)
    }

    pub fn view(&'a self) -> TransactionView<'a> {
        unsafe { TransactionView::from_trits_raw(&self.0) }
    }
}

impl<'a> TransactionMut<'a> for TransactionViewMut<'a> {
    fn set_signature_or_message(&mut self, t: &[Trit]) {
        tx_set_signature_or_message(self.0, t);
    }

    fn set_address(&mut self, h: &HashView) {
        tx_set_address(self.0, h);
    }

    fn set_value(&mut self, v: isize) {
        tx_set_value(self.0, v);
    }

    fn set_tag(&mut self, t: &TagView) {
        tx_set_tag(self.0, t);
    }

    fn set_timestamp(&mut self, t: usize) {
        tx_set_timestamp(self.0, t);
    }

    fn set_current_index(&mut self, idx: usize) {
        tx_set_current_index(self.0, idx);
    }

    fn set_last_index(&mut self, idx: usize) {
        tx_set_last_index(self.0, idx);
    }

    fn set_bundle(&mut self, h: &HashView) {
        tx_set_bundle(self.0, h);
    }

    fn set_trunk(&mut self, h: &HashView) {
        tx_set_trunk(self.0, h);
    }

    fn set_branch(&mut self, h: &HashView) {
        tx_set_branch(self.0, h);
    }

    fn set_nonce(&mut self, h: &NonceView) {
        tx_set_nonce(self.0, h);
    }
}
