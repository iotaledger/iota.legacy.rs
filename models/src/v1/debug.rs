use core::fmt;

use super::Transaction;
use super::TransactionView;
use super::TransactionViewMut;

#[cfg(feature = "alloc")]
fn fmt_tx<'a, T: Transaction<'a>>(tx: &T, f: &mut fmt::Formatter) -> fmt::Result {
    use trytes::*;

    f.write_str("(")
        .and_then(|_| {
            f.write_str(&trits_to_string(tx.signature_or_message()).unwrap())
        })
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.address(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.value(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.tag(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.timestamp(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.current_index(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.last_index(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.bundle(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.trunk(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.branch(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.nonce(), f))
}

#[cfg(not(feature = "alloc"))]
fn fmt_tx<'a, T: Transaction<'a>>(tx: &T, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("(")
        .and_then(|_| fmt::Debug::fmt(&tx.signature_or_message(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.address(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.value(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.tag(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.timestamp(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.current_index(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.last_index(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.bundle(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.trunk(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.branch(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.nonce(), f))
        .and_then(|_| f.write_str(")"))
}

impl<'a> fmt::Debug for TransactionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TransactionView").and_then(
            |_| fmt_tx(&self, f),
        )
    }
}

impl<'a> fmt::Debug for TransactionViewMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TransactionViewMut").and_then(
            |_| fmt_tx(&self.view(), f),
        )
    }
}
