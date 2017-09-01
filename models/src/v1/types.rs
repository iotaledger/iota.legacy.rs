use trytes::*;
use ::inner::*;

use super::NonceView;
pub trait Transaction<'a> {
    fn signature_or_message(&self) -> &[Trit];
    fn address(&self) -> HashView<'a>;
    fn value(&self) -> isize;
    fn tag(&self) -> TagView<'a>;
    fn timestamp(&self) -> usize;
    fn current_index(&self) -> usize;
    fn last_index(&self) -> usize;
    fn bundle(&self) -> HashView<'a>;
    fn trunk(&self) -> HashView<'a>;
    fn branch(&self) -> HashView<'a>;
    fn nonce(&self) -> NonceView<'a>;
    fn essence(&self) -> &[Trit];
}

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionParseError {
    InvalidLength,
}

