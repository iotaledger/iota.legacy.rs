use trytes::*;
use hash::HashView;

pub trait Transaction {
    fn signature_or_message(&self) -> &[Trit];
    fn address(&self) -> HashView;
    fn value(&self) -> isize;
    fn tag(&self) -> HashView;
    fn timestamp(&self) -> usize;
    fn current_index(&self) -> usize;
    fn last_index(&self) -> usize;
    fn bundle(&self) -> HashView;
    fn trunk(&self) -> HashView;
    fn branch(&self) -> HashView;
    fn nonce(&self) -> HashView;

}
