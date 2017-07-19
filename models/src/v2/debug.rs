use core::fmt;

use super::Transaction;
use super::TransactionView;
use super::TransactionBuilder;

#[cfg(feature = "alloc")]
fn fmt_tx(tx: &TransactionView, f: &mut fmt::Formatter) -> fmt::Result {
    use trytes::trits_to_string;

    f.write_str("(")
        .and_then(|_| {
            f.write_str(&trits_to_string(&tx.signature_or_message()).unwrap())
        })
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.extra_data_digest(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.value(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.issued_at(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.issued_at_lb(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.issued_at_ub(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.bundle_nonce(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.trunk(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.branch(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.tag(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.attached_at(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.attached_at_lb(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.attached_at_ub(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.nonce(), f))
        .and_then(|_| f.write_str(", "))
}

#[cfg(not(feature = "alloc"))]
fn fmt_tx(tx: &TransactionView, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("(")
        .and_then(|_| fmt::Debug::fmt(&tx.signature_or_message(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.extra_data_digest(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.value(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.issued_at(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.issued_at_lb(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.issued_at_ub(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.bundle_nonce(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.trunk(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.branch(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.tag(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.attached_at(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.attached_at_lb(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.attached_at_ub(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.nonce(), f))
        .and_then(|_| f.write_str(", "))
}

impl<'a> fmt::Debug for TransactionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TransactionView").and_then(
            |_| fmt_tx(&self, f),
        )
    }
}

impl fmt::Debug for TransactionBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TransactionBuilder").and_then(
            |_| fmt_tx(&self.view(), f),
        )
    }
}
