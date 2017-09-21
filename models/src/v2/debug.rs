use core::fmt;

use super::Transaction;
use super::TransactionView;
use super::TransactionViewMut;

fn fmt_tx<'a, T: Transaction<'a>>(tx: &T, f: &mut fmt::Formatter) -> fmt::Result {

    let r1 = f.write_str("(");
    let r2;

    #[cfg(feature = "alloc")]
    {
        use trytes::*;
        r2 = r1.and_then(|_| {
            f.write_str(&trits_to_string(tx.signature_or_message()).unwrap())
        });
    }

    #[cfg(not(feature = "alloc"))]
    {
        r2 = r1.and_then(|_| fmt::Debug::fmt(&tx.signature_or_message(), f));
    }

    r2.and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.address(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.value(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.obsolete_tag(), f))
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
        .and_then(|_| fmt::Debug::fmt(&tx.tag(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.attachment_timestamp_lower(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.attachment_timestamp_upper(), f))
        .and_then(|_| f.write_str(", "))
        .and_then(|_| fmt::Debug::fmt(&tx.nonce(), f))
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
