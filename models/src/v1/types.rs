use trytes::*;
use ::inner::*;

use super::Nonce;
pub trait Transaction<'a> {
    fn signature_or_message(&self) -> &[Trit];
    fn address(&self) -> Hash<'a>;
    fn value(&self) -> isize;
    fn tag(&self) -> Tag<'a>;
    fn timestamp(&self) -> usize;
    fn current_index(&self) -> usize;
    fn last_index(&self) -> usize;
    fn bundle(&self) -> Hash<'a>;
    fn trunk(&self) -> Hash<'a>;
    fn branch(&self) -> Hash<'a>;
    fn nonce(&self) -> Nonce<'a>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionParseError {
    InvalidLength,
}

