use trytes::*;
use inner::*;

use super::NonceView;
pub trait Transaction<'a> {
    fn signature_or_message(&self) -> &[Trit];
    fn address(&self) -> HashView<'a>;
    fn value(&self) -> i64;
    fn tag(&self) -> TagView<'a>;
    fn timestamp(&self) -> u64;
    fn current_index(&self) -> u64;
    fn last_index(&self) -> u64;
    fn bundle(&self) -> HashView<'a>;
    fn trunk(&self) -> HashView<'a>;
    fn branch(&self) -> HashView<'a>;
    fn nonce(&self) -> NonceView<'a>;
    fn essence(&self) -> &[Trit];
}

pub trait TransactionMut<'a> {
    fn set_signature_or_message(&mut self, t: &[Trit]);
    fn set_address(&mut self, h: &HashView);
    fn set_value(&mut self, v: i64);
    fn set_tag(&mut self, t: &TagView);
    fn set_timestamp(&mut self, t: u64);
    fn set_current_index(&mut self, idx: u64);
    fn set_last_index(&mut self, idx: u64);
    fn set_bundle(&mut self, h: &HashView);
    fn set_trunk(&mut self, h: &HashView);
    fn set_branch(&mut self, h: &HashView);
    fn set_nonce(&mut self, h: &NonceView);
}

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionParseError {
    InvalidLength,
}
