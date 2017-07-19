use trytes::*;
use super::hash::HashView;
use super::nonce::NonceView;

pub trait Transaction {
    fn signature_or_message(&self) -> &[Trit];
    fn extra_data_digest(&self) -> HashView;
    fn address(&self) -> HashView;
    fn value(&self) -> isize;
    fn issued_at(&self) -> usize;
    fn issued_at_lb(&self) -> usize;
    fn issued_at_ub(&self) -> usize;
    fn bundle_nonce(&self) -> NonceView;
    fn trunk(&self) -> HashView;
    fn branch(&self) -> HashView;
    fn tag(&self) -> NonceView;
    fn attached_at(&self) -> usize;
    fn attached_at_lb(&self) -> usize;
    fn attached_at_ub(&self) -> usize;
    fn nonce(&self) -> NonceView;
}
