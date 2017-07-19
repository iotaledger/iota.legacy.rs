use trytes::*;
use Hash;
use Tag;
use super::inner::Nonce;

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionParseError {
    InvalidLength,
}

pub trait Transaction {
    fn signature_or_message(&self) -> &[Trit];
    fn extra_data_digest(&self) -> Hash;
    fn address(&self) -> Hash;
    fn value(&self) -> isize;
    fn issued_at(&self) -> usize;
    fn issued_at_lb(&self) -> usize;
    fn issued_at_ub(&self) -> usize;
    fn bundle_nonce(&self) -> Nonce;
    fn trunk(&self) -> Hash;
    fn branch(&self) -> Hash;
    fn tag(&self) -> Tag;
    fn attached_at(&self) -> usize;
    fn attached_at_lb(&self) -> usize;
    fn attached_at_ub(&self) -> usize;
    fn nonce(&self) -> Nonce;
}
