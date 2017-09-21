use trytes::*;
use ::inner::*;
use super::inner::*;

use super::get::*;
use super::NonceView;
use super::types::*;
use super::constants::*;
#[derive(Clone, Eq, PartialEq)]
pub struct TransactionView<'a>(&'a [Trit]);

impl<'a> TransactionView<'a> {
    pub fn from_trits(base: &'a [Trit]) -> Result<Self, TransactionParseError> {
        if base.len() != TRANSACTION_LEN_TRITS {
            return Err(TransactionParseError::InvalidLength);
        }
        Ok(TransactionView(base))
    }

    pub unsafe fn from_trits_raw(base: &'a [Trit]) -> Self {
        TransactionView(base)
    }
}

impl<'a> ::core::ops::Deref for TransactionView<'a> {
    type Target = [Trit];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'b> Transaction<'a> for &'b TransactionView<'a> {
    fn signature_or_message(&self) -> &[Trit] {
        &self.0[0..ADDRESS_OFFSET]
    }

    fn address(&self) -> HashView<'a> {
        unsafe { HashView::from_trits_raw(tx_address(self.0)) }
    }
    fn obsolete_tag(&self) -> TagView<'a> {
        unsafe { TagView::from_trits_raw(tx_obsolete_tag(self.0)) }
    }
    fn bundle(&self) -> HashView<'a> {
        unsafe { HashView::from_trits_raw(tx_bundle(self.0)) }
    }
    fn trunk(&self) -> HashView<'a> {
        unsafe { HashView::from_trits_raw(tx_trunk(self.0)) }
    }
    fn branch(&self) -> HashView<'a> {
        unsafe { HashView::from_trits_raw(tx_branch(self.0)) }
    }
    fn nonce(&self) -> NonceView<'a> {
        unsafe { NonceView::from_trits_raw(tx_nonce(self.0)) }
    }

    fn tag(&self) -> TagView<'a> {
        unsafe { TagView::from_trits_raw(tx_tag(self.0)) }
    }

    fn attachment_timestamp(&self) -> usize {
        tx_attachment_timestamp(self.0)
    }

    fn attachment_timestamp_upper(&self) -> usize {
        tx_attachment_timestamp_upper(self.0)
    }

    fn attachment_timestamp_lower(&self) -> usize {
        tx_attachment_timestamp_lower(self.0)
    }

    fn value(&self) -> isize {
        tx_value(self.0)
    }

    fn timestamp(&self) -> usize {
        tx_timestamp(self.0)
    }

    fn current_index(&self) -> usize {
        tx_current_index(self.0)
    }

    fn last_index(&self) -> usize {
        tx_last_index(self.0)
    }

    fn essence(&self) -> &[Trit] {
        tx_essence(self.0)
    }
}

impl<'a> Transaction<'a> for TransactionView<'a> {
    fn signature_or_message(&self) -> &[Trit] {
        &self.0[0..ADDRESS_OFFSET]
    }

    fn address(&self) -> HashView<'a> {
        unsafe { HashView::from_trits_raw(tx_address(self.0)) }
    }
    fn obsolete_tag(&self) -> TagView<'a> {
        unsafe { TagView::from_trits_raw(tx_obsolete_tag(self.0)) }
    }
    fn bundle(&self) -> HashView<'a> {
        unsafe { HashView::from_trits_raw(tx_bundle(self.0)) }
    }
    fn trunk(&self) -> HashView<'a> {
        unsafe { HashView::from_trits_raw(tx_trunk(self.0)) }
    }
    fn branch(&self) -> HashView<'a> {
        unsafe { HashView::from_trits_raw(tx_branch(self.0)) }
    }
    fn nonce(&self) -> NonceView<'a> {
        unsafe { NonceView::from_trits_raw(tx_nonce(self.0)) }
    }

    fn tag(&self) -> TagView<'a> {
        unsafe { TagView::from_trits_raw(tx_tag(self.0)) }
    }

    fn attachment_timestamp(&self) -> usize {
        tx_attachment_timestamp(self.0)
    }

    fn attachment_timestamp_upper(&self) -> usize {
        tx_attachment_timestamp_upper(self.0)
    }

    fn attachment_timestamp_lower(&self) -> usize {
        tx_attachment_timestamp_lower(self.0)
    }

    fn value(&self) -> isize {
        tx_value(self.0)
    }

    fn timestamp(&self) -> usize {
        tx_timestamp(self.0)
    }

    fn current_index(&self) -> usize {
        tx_current_index(self.0)
    }

    fn last_index(&self) -> usize {
        tx_last_index(self.0)
    }

    fn essence(&self) -> &[Trit] {
        tx_essence(self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use alloc::Vec;

    #[test]
    fn tx_view_and_build() {

        const TX1: &'static str = "SPAMSPAMSPAM999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999999999999999999999999999999999999999999999999\
                                   999999999999999TEST9TWELVE9999999999999999RCPWZXD99999999999\
                                   999999999SLHL9DOBKRUUBPLKXFPLTSWEYAHIHLGH9RGLBWNWQLAAGDSAIMZ\
                                   QEEKCFZVGTKJVWKKKDSCFKSWAZAFZCQURTFPHPMVYHVUWDVQJUKEGQQEXNWP\
                                   YKDPEPR9VI9IKVOYU9XUU99DAQJOKACMIMRKRPULATEVXM99999VSELFFFYB\
                                   HSNQBEGDKDADGNDIRKJPCRSGDUQEJQDPEKLITQSGRYELGFNNEGRGABMZEXHS\
                                   CSZPNKMA9999TEST9TWELVE99999999999999999YHXYHZIE999999999YYK\
                                   XHCKEUNRYCUDSTZEEJQORYXWMQATVERBP";

        let tx1: Vec<Trit> = TX1.chars().flat_map(char_to_trits).cloned().collect();
        let txv = TransactionView(tx1.as_slice());

        assert_eq!(
            trits_to_string(&txv.signature_or_message()[..36]).unwrap(),
            "SPAMSPAMSPAM"
        );
        assert!(txv.signature_or_message().iter().skip(36).all(|&t| t == 0));

        let address_ex = "999999999999999999999999999999999999999999999999999999999999999999999999999999999";
        let value_ex = 0;
        let timestamp_ex = 1506017115;
        let current_index_ex = 0;
        let last_index_ex = 0;
        let bundle_ex = "SLHL9DOBKRUUBPLKXFPLTSWEYAHIHLGH9RGLBWNWQLAAGDSAIMZQEEKCFZVGTKJVWKKKDSCFKSWAZAFZC";
        let trunk_ex = "QURTFPHPMVYHVUWDVQJUKEGQQEXNWPYKDPEPR9VI9IKVOYU9XUU99DAQJOKACMIMRKRPULATEVXM99999";
        let branch_ex = "VSELFFFYBHSNQBEGDKDADGNDIRKJPCRSGDUQEJQDPEKLITQSGRYELGFNNEGRGABMZEXHSCSZPNKMA9999";

        let tag_ex = "TEST9TWELVE9999999999999999";
        let obsolete_tag_ex = "TEST9TWELVE9999999999999999";
        let attachment_timestamp_ex = 1506017115846;
        let attachment_timestamp_lower_ex: usize = 0;
        let attachment_timestamp_upper_ex: usize = (-1637966580329_isize) as usize;

        let nonce_ex = "NRYCUDSTZEEJQORYXWMQATVERBP";

        assert_eq!(trits_to_string(&txv.address()).unwrap(), address_ex);
        assert_eq!(txv.value(), value_ex);
        assert_eq!(
            trits_to_string(&txv.obsolete_tag()).unwrap(),
            obsolete_tag_ex
        );
        assert_eq!(txv.timestamp(), timestamp_ex);
        assert_eq!(txv.current_index(), current_index_ex);
        assert_eq!(txv.last_index(), last_index_ex);
        assert_eq!(trits_to_string(&txv.bundle()).unwrap(), bundle_ex);
        assert_eq!(trits_to_string(&txv.trunk()).unwrap(), trunk_ex);
        assert_eq!(trits_to_string(&txv.branch()).unwrap(), branch_ex);
        assert_eq!(trits_to_string(&txv.tag()).unwrap(), tag_ex);
        assert_eq!(txv.attachment_timestamp(), attachment_timestamp_ex);
        assert_eq!(txv.attachment_timestamp_lower(), attachment_timestamp_lower_ex);
        assert_eq!(txv.attachment_timestamp_upper(), attachment_timestamp_upper_ex);
        assert_eq!(trits_to_string(&txv.nonce()).unwrap(), nonce_ex);

        // test builder.
        let mut tx = ::v2::tx_alloc_stack();
        {
            let mut builder = ::v2::TransactionViewMut::from_trits(&mut tx).unwrap();
            builder.set_signature_or_message(txv.signature_or_message());
            builder.set_address(&txv.address());
            builder.set_value(txv.value());
            builder.set_obsolete_tag(&txv.obsolete_tag());
            builder.set_timestamp(txv.timestamp());
            builder.set_current_index(current_index_ex);
            builder.set_last_index(last_index_ex);
            builder.set_bundle(&txv.bundle());
            builder.set_trunk(&txv.trunk());
            builder.set_branch(&txv.branch());
            builder.set_tag(&txv.tag());
            builder.set_attachment_timestamp(attachment_timestamp_ex);
            builder.set_attachment_timestamp_lower(attachment_timestamp_lower_ex);
            builder.set_attachment_timestamp_upper(attachment_timestamp_upper_ex);
            builder.set_nonce(&txv.nonce());
        }

        let trytes = trits_to_string(&tx).unwrap();
        assert_eq!(trytes, TX1);

        // test tx hash
        use curl_cpu::*;
        let mut curl = CpuCurl::<Trit>::default();
        let tx_hash_ex = "YPVWGXZ9NYK9YHKAZROK9AQVVLNRJJQIYVSUHDUTFYCDDHWIFVPBASEGPLWHZOTZBMMLNMLSMWPBD9999";

        let tx_hash = ::v2::tx_hash(&tx, &mut curl);
        assert_eq!(trits_to_string(&*tx_hash).unwrap(), tx_hash_ex);


    }
}
