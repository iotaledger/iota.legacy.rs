use core::fmt;
use core::fmt::Debug;
use alloc::string::ToString;

use trytes::*;

use hash::*;
use tag::*;
pub trait Transaction {
    fn signature_or_message(&self) -> &[Trit];
    fn address(&self) -> HashView;
    fn value(&self) -> isize;
    fn tag(&self) -> TagView;
    fn timestamp(&self) -> usize;
    fn current_index(&self) -> usize;
    fn last_index(&self) -> usize;
    fn bundle(&self) -> HashView;
    fn trunk(&self) -> HashView;
    fn branch(&self) -> HashView;
    fn nonce(&self) -> HashView;

    fn fmt_tx(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("(")
            .and_then(|_| {
                f.write_str(&trits_to_string(self.signature_or_message()).unwrap())
            })
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.address().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.value().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.tag().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.timestamp().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.current_index().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| f.write_str(&self.last_index().to_string()))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.bundle().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.trunk().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.branch().fmt(f))
            .and_then(|_| f.write_str(", "))
            .and_then(|_| self.nonce().fmt(f))
            .and_then(|_| f.write_str(")"))
    }
}
