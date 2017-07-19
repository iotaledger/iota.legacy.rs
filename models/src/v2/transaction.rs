#[allow(dead_code)]
use alloc::Vec;

use trytes::*;
use hash::*;
use tag::*;
use super::nonce::*;
use super::types::*;

const TRANSACTION_LEN_TRITS: usize = 2673 * TRITS_PER_TRYTE; // = 8019

const MESSAGE_TRITS: usize = 6561;
const EXTRA_DATA_DIGEST_TRITS: usize = 243;
const ADDRESS_TRITS: usize = HASH_LEN_TRITS;
const VALUE_TRITS: usize = 81;
const ISSUED_AT_TRITS: usize = 27;
const ISSUED_AT_LB_TRITS: usize = 27;
const ISSUED_AT_UB_TRITS: usize = 27;
const BUNDLE_NONCE_TRITS: usize = NONCE_LEN_TRITS;
const TRUNK_TRITS: usize = HASH_LEN_TRITS;
const BRANCH_TRITS: usize = HASH_LEN_TRITS;
const TAG_TRITS: usize = TAG_LEN_TRITS;
const ATTACHED_AT_TRITS: usize = 27;
const ATTACHED_AT_LB_TRITS: usize = 27;
const ATTACHED_AT_UB_TRITS: usize = 27;

const EXTRA_DATA_OFFSET: usize = MESSAGE_TRITS;
const ADDRESS_OFFSET: usize = EXTRA_DATA_OFFSET + EXTRA_DATA_DIGEST_TRITS;
const VALUE_OFFSET: usize = ADDRESS_OFFSET + ADDRESS_TRITS;
const ISSUED_AT_OFFSET: usize = VALUE_OFFSET + VALUE_TRITS;
const ISSUED_AT_LB_OFFSET: usize = ISSUED_AT_OFFSET + ISSUED_AT_TRITS;
const ISSUED_AT_UB_OFFSET: usize = ISSUED_AT_LB_OFFSET + ISSUED_AT_LB_TRITS;
const BUNDLE_NONCE_OFFSET: usize = ISSUED_AT_UB_OFFSET + ISSUED_AT_UB_TRITS;
const TRUNK_OFFSET: usize = BUNDLE_NONCE_OFFSET + BUNDLE_NONCE_TRITS;
const BRANCH_OFFSET: usize = TRUNK_OFFSET + TRUNK_TRITS;
const TAG_OFFSET: usize = BRANCH_OFFSET + BRANCH_TRITS;
const ATTACHED_AT_OFFSET: usize = TAG_OFFSET + TAG_TRITS;
const ATTACHED_AT_LB_OFFSET: usize = ATTACHED_AT_OFFSET + ATTACHED_AT_TRITS;
const ATTACHED_AT_UB_OFFSET: usize = ATTACHED_AT_LB_OFFSET + ATTACHED_AT_LB_TRITS;
const NONCE_OFFSET: usize = ATTACHED_AT_UB_OFFSET + ATTACHED_AT_UB_TRITS;

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionParseError {
    InvalidLength,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TransactionBuilder {
    signature_or_message: Vec<Trit>,
    extra_data_digest: Hash,
    address: Hash,
    value: isize,

    issued_at: usize,
    issued_at_lb: usize,
    issued_at_ub: usize,

    bundle_nonce: Nonce,
    trunk: Hash,
    branch: Hash,
    tag: Tag,

    attached_at: usize,
    attached_at_lb: usize,
    attached_at_ub: usize,

    nonce: Nonce,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TransactionView<'a>(&'a [Trit]);

impl<'a> TransactionView<'a> {
    pub fn from_trits(trits: &'a [Trit]) -> Result<Self, TransactionParseError> {
        if trits.len() != TRANSACTION_LEN_TRITS {
            return Err(TransactionParseError::InvalidLength);
        }

        Ok(TransactionView::<'a>(&trits))
    }

    pub fn to_builder(&self) -> TransactionBuilder {
        TransactionBuilder {
            signature_or_message: self.signature_or_message().to_vec(),
            extra_data_digest: self.extra_data_digest().to_hash(),
            address: self.address().to_hash(),
            value: self.value(),
            issued_at: self.issued_at(),
            issued_at_lb: self.issued_at_lb(),
            issued_at_ub: self.issued_at_ub(),
            bundle_nonce: self.bundle_nonce().to_nonce(),
            trunk: self.trunk().to_hash(),
            branch: self.branch().to_hash(),
            tag: self.tag().to_tag(),
            attached_at: self.attached_at(),
            attached_at_lb: self.attached_at_lb(),
            attached_at_ub: self.attached_at_ub(),
            nonce: self.nonce().to_nonce(),
        }
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

impl Transaction for TransactionBuilder {
    fn signature_or_message(&self) -> &[Trit] {
        self.signature_or_message.as_slice()
    }

    fn extra_data_digest(&self) -> HashView {
        self.extra_data_digest.view()
    }

    fn address(&self) -> HashView {
        self.address.view()
    }

    fn value(&self) -> isize {
        self.value
    }

    fn issued_at(&self) -> usize {
        self.issued_at
    }
    fn issued_at_lb(&self) -> usize {
        self.issued_at_lb
    }
    fn issued_at_ub(&self) -> usize {
        self.issued_at_ub
    }

    fn bundle_nonce(&self) -> NonceView {
        self.bundle_nonce.view()
    }

    fn trunk(&self) -> HashView {
        self.trunk.view()
    }

    fn branch(&self) -> HashView {
        self.branch.view()
    }

    fn tag(&self) -> TagView {
        self.tag.view()
    }

    fn attached_at(&self) -> usize {
        self.attached_at
    }
    fn attached_at_lb(&self) -> usize {
        self.attached_at_lb
    }
    fn attached_at_ub(&self) -> usize {
        self.attached_at_ub
    }

    fn nonce(&self) -> NonceView {
        self.nonce.view()
    }
}

impl<'a> IntoTrits<Trit> for TransactionView<'a> {
    fn len_trits(&self) -> usize {
        TRANSACTION_LEN_TRITS
    }

    fn trits(&self) -> Vec<Trit> {
        self.0.to_vec()
    }
}

impl IntoTrits<Trit> for TransactionBuilder {
    fn len_trits(&self) -> usize {
        TRANSACTION_LEN_TRITS
    }

    fn trits(&self) -> Vec<Trit> {
        let mut trits = self.signature_or_message.clone();
        trits.reserve(self.len_trits() - self.signature_or_message.len());

        trits.append(&mut self.extra_data_digest.trits());
        trits.append(&mut self.address.trits());
        trits.append(&mut num::int2trits(self.value, VALUE_TRITS as u8));

        trits.append(&mut num::int2trits(
            self.issued_at as isize,
            ISSUED_AT_TRITS as u8,
        ));
        trits.append(&mut num::int2trits(
            self.issued_at_lb as isize,
            ISSUED_AT_LB_TRITS as u8,
        ));
        trits.append(&mut num::int2trits(
            self.issued_at_ub as isize,
            ISSUED_AT_UB_TRITS as u8,
        ));

        trits.append(&mut self.bundle_nonce.trits());
        trits.append(&mut self.trunk.trits());
        trits.append(&mut self.branch.trits());
        trits.append(&mut self.tag.trits());

        trits.append(&mut num::int2trits(
            self.attached_at as isize,
            ATTACHED_AT_TRITS as u8,
        ));
        trits.append(&mut num::int2trits(
            self.attached_at_lb as isize,
            ATTACHED_AT_LB_TRITS as u8,
        ));
        trits.append(&mut num::int2trits(
            self.attached_at_ub as isize,
            ATTACHED_AT_UB_TRITS as u8,
        ));

        trits.append(&mut self.nonce.trits());
        trits
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        TransactionBuilder {
            signature_or_message: Vec::new(),
            extra_data_digest: Hash::default(),
            address: Hash::default(),
            value: 0,
            issued_at: 0,
            issued_at_lb: 0,
            issued_at_ub: usize::max_value(),
            bundle_nonce: Nonce::default(),
            trunk: Hash::default(),
            branch: Hash::default(),
            tag: Tag::default(),
            attached_at: 0,
            attached_at_lb: 0,
            attached_at_ub: usize::max_value(),
            nonce: Nonce::default(),
        }
    }
}

impl TransactionBuilder {
    pub fn set_signature_or_message(&mut self, value: &Vec<Trit>) -> Option<&mut Self> {
        if value.len() != MESSAGE_TRITS {
            None
        } else {
            self.signature_or_message = value.clone();
            Some(self)
        }
    }

    pub fn set_extra_data_digest(&mut self, hash: &Hash) -> &mut Self {
        self.extra_data_digest = hash.clone();
        self
    }

    pub fn set_address(&mut self, hash: &Hash) -> &mut Self {
        self.address = hash.clone();
        self
    }

    pub fn set_value(&mut self, value: isize) -> &mut Self {
        self.value = value;
        self
    }

    pub fn set_issued_at(&mut self, value: usize) -> &mut Self {
        self.issued_at = value;
        self
    }

    pub fn set_issued_at_lb(&mut self, value: usize) -> &mut Self {
        self.issued_at_lb = value;
        self
    }

    pub fn set_issued_at_ub(&mut self, value: usize) -> &mut Self {
        self.issued_at_ub = value;
        self
    }

    pub fn set_bundle_nonce(&mut self, nonce: &Nonce) -> &mut Self {
        self.bundle_nonce = nonce.clone();
        self
    }

    pub fn set_trunk(&mut self, hash: &Hash) -> &mut Self {
        self.trunk = hash.clone();
        self
    }

    pub fn set_branch(&mut self, hash: &Hash) -> &mut Self {
        self.branch = hash.clone();
        self
    }

    pub fn set_tag(&mut self, tag: &Tag) -> &mut Self {
        self.tag = tag.clone();
        self
    }

    pub fn set_attached_at(&mut self, value: usize) -> &mut Self {
        self.attached_at = value;
        self
    }

    pub fn set_attached_at_lb(&mut self, value: usize) -> &mut Self {
        self.attached_at_lb = value;
        self
    }

    pub fn set_attached_at_ub(&mut self, value: usize) -> &mut Self {
        self.attached_at_ub = value;
        self
    }

    pub fn set_nonce(&mut self, nonce: &Nonce) -> &mut Self {
        self.nonce = nonce.clone();
        self
    }
}
