use trytes::*;
use inner::*;

use super::NonceView;
use super::types::*;
use super::constants::*;
use super::builder::*;

#[derive(Clone, Eq, PartialEq)]
pub struct TransactionView<'a>(&'a [Trit]);

impl<'a> TransactionView<'a> {
    pub fn from_trits(base: &'a [Trit]) -> Result<Self, TransactionParseError> {
        if base.len() != TRANSACTION_LEN_TRITS {
            return Err(TransactionParseError::InvalidLength);
        }
        Ok(TransactionView(base))
    }

    pub fn to_builder(&self) -> TransactionBuilder {
        TransactionBuilder::from_trits(self).unwrap()
    }
}

impl<'a> ::core::ops::Deref for TransactionView<'a> {
    type Target = [Trit];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Transaction<'a> for TransactionView<'a> {
    fn signature_or_message(&self) -> &[Trit] {
        &self.0[0..ADDRESS_OFFSET]
    }

    fn address(&self) -> HashView<'a> {
        HashView::from_trits(&self.0[ADDRESS_OFFSET..VALUE_OFFSET]).unwrap()
    }

    fn value(&self) -> isize {
        num::trits2int(&self.0[VALUE_OFFSET..TAG_OFFSET])
    }

    fn tag(&self) -> TagView<'a> {
        TagView::from_trits(&self.0[TAG_OFFSET..TIMESTAMP_OFFSET]).unwrap()
    }

    fn timestamp(&self) -> usize {
        num::trits2int(&self.0[TIMESTAMP_OFFSET..CURRENT_INDEX_OFFSET]) as usize
    }

    fn current_index(&self) -> usize {
        num::trits2int(&self.0[CURRENT_INDEX_OFFSET..LAST_INDEX_OFFSET]) as usize
    }

    fn last_index(&self) -> usize {
        num::trits2int(&self.0[LAST_INDEX_OFFSET..BUNDLE_OFFSET]) as usize
    }

    fn bundle(&self) -> HashView<'a> {
        HashView::from_trits(&self.0[BUNDLE_OFFSET..TRUNK_OFFSET]).unwrap()
    }

    fn trunk(&self) -> HashView<'a> {
        HashView::from_trits(&self.0[TRUNK_OFFSET..BRANCH_OFFSET]).unwrap()
    }

    fn branch(&self) -> HashView<'a> {
        HashView::from_trits(&self.0[BRANCH_OFFSET..NONCE_OFFSET]).unwrap()
    }

    fn nonce(&self) -> NonceView<'a> {
        NonceView::from_trits(&self.0[NONCE_OFFSET..TRANSACTION_LEN_TRITS]).unwrap()
    }

    fn essence(&self) -> &[Trit] {
        &self.0[ESSENCE_OFFSET..][..ESSENCE_TRITS]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::Vec;
    use super::super::builder::*;

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
        let txv = TransactionView(tx1.as_slice());

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
        let mut builder = TransactionBuilder::default();
        builder.set_address(&txv.address())
            .set_value(txv.value())
            .set_timestamp(txv.timestamp())
            .set_current_index(current_index_ex)
            .set_last_index(last_index_ex)
            .set_bundle(&txv.bundle())
            .set_trunk(&txv.trunk())
            .set_branch(&txv.branch())
            .set_nonce(&txv.nonce());

        let trytes = trits_to_string(&builder).unwrap();
        assert_eq!(trytes, TX1);


    }
}
