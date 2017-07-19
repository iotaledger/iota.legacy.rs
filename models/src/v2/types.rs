use core::fmt;
use alloc::string::ToString;

use trytes::*;

use hash::HashView;
use tag::TagView;
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
    fn tag(&self) -> TagView;
    fn attached_at(&self) -> usize;
    fn attached_at_lb(&self) -> usize;
    fn attached_at_ub(&self) -> usize;
    fn nonce(&self) -> NonceView;

    fn fmt_tx(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::fmt::Debug;

        f.write_str("(")
            .and_then(|_| {
                f.write_str(&trits_to_string(self.signature_or_message()).unwrap())
            })
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.extra_data_digest().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.value().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.issued_at().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.issued_at_lb().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.issued_at_ub().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.bundle_nonce().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.trunk().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.branch().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.tag().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.attached_at().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.attached_at_lb().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.attached_at_ub().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.nonce().fmt(f))
            .and_then(|_| f.write_str(", "))
    }
}
