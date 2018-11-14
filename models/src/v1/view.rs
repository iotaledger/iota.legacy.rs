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
    fn tag(&self) -> TagView<'a> {
        unsafe { TagView::from_trits_raw(tx_tag(self.0)) }
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

    fn value(&self) -> i64 {
        tx_value(self.0)
    }

    fn timestamp(&self) -> u64 {
        tx_timestamp(self.0)
    }

    fn current_index(&self) -> u64 {
        tx_current_index(self.0)
    }

    fn last_index(&self) -> u64 {
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
    fn tag(&self) -> TagView<'a> {
        unsafe { TagView::from_trits_raw(tx_tag(self.0)) }
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

    fn value(&self) -> i64 {
        tx_value(self.0)
    }

    fn timestamp(&self) -> u64 {
        tx_timestamp(self.0)
    }

    fn current_index(&self) -> u64 {
        tx_current_index(self.0)
    }

    fn last_index(&self) -> u64 {
        tx_last_index(self.0)
    }

    fn essence(&self) -> &[Trit] {
        tx_essence(self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use alloc::vec::Vec;

    #[test]
    fn tx_view_and_build() {

        const TX1: &'static str = "999999999999999999999999999999999999999999999999999999999999\
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
                                   999999999999999999999999999MNDRSJHBDIGXMBEVZMPCGPVONHDMBXCEN\
                                   NRNUZKTBIEYTACSRBOCUBFLMUDKTTOHYMXNAHZUARBKDWNVXNSXIGBA99999\
                                   999999999999999999999999999999999999999999STWPWXD99999999999\
                                   D99999999MI9HQGQFJWMCBKXAVBVMWWRWFNWLRXHHQGXHAOZDBXXAXKRVXPP\
                                   PIJNIBLOVRRTFDGWZLNCXPTABADHEDHZYEDHZMRJHT9YUMLBHBQUZWFS9AAW\
                                   UAZSKKZVYSZQOAVCLEOAHXLTWKCCCJBLJUJJOVHCOFVVHO99999AHTIYDFRJ\
                                   AOZJTOGIRIXZEYJSHMXEIGSVYPDQBCLTVQWHYAZMWEZOORGZTXWZMSZQVFQG\
                                   FPGSWGK99999ASUKDG9EANYHNGWBYHVISVZTUQJAQEANXJQAJ9CQYSYOPXKQ\
                                   JBPLCQAJDIBRYYQPOQ9WMZVUNMSSHULRU";

        let tx1: Vec<Trit> = TX1.chars().flat_map(char_to_trits).cloned().collect();
        let txv = ::v1::TransactionView::from_trits(tx1.as_slice()).unwrap();

        assert!(txv.signature_or_message().iter().all(|&t| t == 0));

        let address_ex = "MNDRSJHBDIGXMBEVZMPCGPVONHDMBXCENNRNUZKTBIEYTACSRBOCUBFLMUDKTTOHYMXNAHZUARBKDWNVX";
        let value_ex = 420013121;
        let timestamp_ex = 1504289845;
        let current_index_ex = 0;
        let last_index_ex = 4;
        let bundle_ex = "MI9HQGQFJWMCBKXAVBVMWWRWFNWLRXHHQGXHAOZDBXXAXKRVXPPPIJNIBLOVRRTFDGWZLNCXPTABADHED";
        let trunk_ex = "HZYEDHZMRJHT9YUMLBHBQUZWFS9AAWUAZSKKZVYSZQOAVCLEOAHXLTWKCCCJBLJUJJOVHCOFVVHO99999";
        let branch_ex = "AHTIYDFRJAOZJTOGIRIXZEYJSHMXEIGSVYPDQBCLTVQWHYAZMWEZOORGZTXWZMSZQVFQGFPGSWGK99999";
        let nonce_ex = "ASUKDG9EANYHNGWBYHVISVZTUQJAQEANXJQAJ9CQYSYOPXKQJBPLCQAJDIBRYYQPOQ9WMZVUNMSSHULRU";

        assert_eq!(trits_to_string(&txv.address()).unwrap(), address_ex);
        assert_eq!(txv.value(), value_ex);
        assert!(txv.tag().iter().all(|&t| t == 0));
        assert_eq!(txv.timestamp(), timestamp_ex);
        assert_eq!(txv.current_index(), current_index_ex);
        assert_eq!(txv.last_index(), last_index_ex);
        assert_eq!(trits_to_string(&txv.bundle()).unwrap(), bundle_ex);
        assert_eq!(trits_to_string(&txv.trunk()).unwrap(), trunk_ex);
        assert_eq!(trits_to_string(&txv.branch()).unwrap(), branch_ex);
        assert_eq!(trits_to_string(&txv.nonce()).unwrap(), nonce_ex);

        // test builder.
        let mut tx = ::v1::tx_alloc_stack();
        {
            let mut builder = ::v1::TransactionViewMut::from_trits(&mut tx).unwrap();
            builder.set_address(&txv.address());
            builder.set_value(txv.value());
            builder.set_timestamp(txv.timestamp());
            builder.set_current_index(current_index_ex);
            builder.set_last_index(last_index_ex);
            builder.set_bundle(&txv.bundle());
            builder.set_trunk(&txv.trunk());
            builder.set_branch(&txv.branch());
            builder.set_nonce(&txv.nonce());
        }

        let trytes = trits_to_string(&tx).unwrap();
        assert_eq!(trytes, TX1);

        // test tx hash
        use curl_cpu::*;
        let mut curl = CpuCurl::<Trit>::default();
        let tx_hash_ex = "UZXJIEA9RWL9QOTSMFQTWLHWJSCILUYDJWXBB9BBZYMQMMSZXZK9UVKQHXK9UFMZWWVSTIIYJTOO99999";

        let tx_hash = ::v1::tx_hash(&tx, &mut curl);
        assert_eq!(trits_to_string(&*tx_hash).unwrap(), tx_hash_ex);
    }
}
