#[allow(dead_code)]
use alloc::Vec;
use core::fmt;

use trytes::*;
use hash::*;
use tag::*;
use super::types::*;

pub const TRANSACTION_LEN_TRITS: usize = 2673 * TRITS_PER_TRYTE;

const MESSAGE_TRITS: usize = 6561;
const TIMESTAMP_TRITS: usize = 27;
const CURRENT_INDEX_TRITS: usize = 27;
const LAST_INDEX_TRITS: usize = 27;
const VALUE_TRITS: usize = 81;
const TAG_TRITS: usize = 81;

const ADDRESS_OFFSET: usize = MESSAGE_TRITS;
const VALUE_OFFSET: usize = ADDRESS_OFFSET + HASH_LEN_TRITS;
const TAG_OFFSET: usize = VALUE_OFFSET + VALUE_TRITS;
const TIMESTAMP_OFFSET: usize = TAG_OFFSET + TAG_TRITS;
const CURRENT_INDEX_OFFSET: usize = TIMESTAMP_OFFSET + TIMESTAMP_TRITS;
const LAST_INDEX_OFFSET: usize = CURRENT_INDEX_OFFSET + CURRENT_INDEX_TRITS;
const BUNDLE_OFFSET: usize = LAST_INDEX_OFFSET + LAST_INDEX_TRITS;
const TRUNK_OFFSET: usize = BUNDLE_OFFSET + HASH_LEN_TRITS;
const BRANCH_OFFSET: usize = TRUNK_OFFSET + HASH_LEN_TRITS;
const NONCE_OFFSET: usize = BRANCH_OFFSET + HASH_LEN_TRITS;

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionParseError {
    InvalidLength,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TransactionBuilder {
    signature_or_message: Vec<Trit>,
    address: Hash,
    value: isize,
    tag: Tag,
    timestamp: usize,
    current_index: usize,
    last_index: usize,
    bundle: Hash,
    trunk: Hash,
    branch: Hash,
    nonce: Hash,
}

#[derive(Clone, Eq, PartialEq)]
pub struct TransactionView<'a>(&'a [Trit]);

impl<'a> TransactionView<'a> {
    pub fn from_trits(trits: &'a [Trit]) -> Result<Self, TransactionParseError> {
        if trits.len() != TRANSACTION_LEN_TRITS {
            return Err(TransactionParseError::InvalidLength);
        }

        Ok(TransactionView::<'a>(&trits))
    }

    pub fn to_builder(&self) -> TransactionBuilder {
        TransactionBuilder {
            signature_or_message: self.signature_or_message().to_vec(),
            address: self.address().to_hash(),
            value: self.value(),
            tag: self.tag().to_tag(),
            timestamp: self.timestamp(),
            current_index: self.current_index(),
            last_index: self.last_index(),
            bundle: self.bundle().to_hash(),
            trunk: self.trunk().to_hash(),
            branch: self.branch().to_hash(),
            nonce: self.nonce().to_hash(),
        }
    }
}

impl<'a> Transaction for TransactionView<'a> {
    fn signature_or_message(&self) -> &[Trit] {
        &self.0[0..ADDRESS_OFFSET]
    }

    fn address(&self) -> HashView {
        HashView::from_trits(&self.0[ADDRESS_OFFSET..VALUE_OFFSET]).unwrap()
    }

    fn value(&self) -> isize {
        num::trits2int(&self.0[VALUE_OFFSET..TAG_OFFSET])
    }

    fn tag(&self) -> TagView {
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

    fn bundle(&self) -> HashView {
        HashView::from_trits(&self.0[BUNDLE_OFFSET..TRUNK_OFFSET]).unwrap()
    }

    fn trunk(&self) -> HashView {
        HashView::from_trits(&self.0[TRUNK_OFFSET..BRANCH_OFFSET]).unwrap()
    }

    fn branch(&self) -> HashView {
        HashView::from_trits(&self.0[BRANCH_OFFSET..NONCE_OFFSET]).unwrap()
    }

    fn nonce(&self) -> HashView {
        HashView::from_trits(&self.0[NONCE_OFFSET..TRANSACTION_LEN_TRITS]).unwrap()
    }
}

impl Transaction for TransactionBuilder {
    fn signature_or_message(&self) -> &[Trit] {
        self.signature_or_message.as_slice()
    }

    fn address(&self) -> HashView {
        self.address.view()
    }

    fn value(&self) -> isize {
        self.value
    }

    fn tag(&self) -> TagView {
        self.tag.view()
    }

    fn timestamp(&self) -> usize {
        self.timestamp
    }

    fn current_index(&self) -> usize {
        self.current_index
    }

    fn last_index(&self) -> usize {
        self.last_index
    }

    fn bundle(&self) -> HashView {
        self.bundle.view()
    }

    fn trunk(&self) -> HashView {
        self.trunk.view()
    }

    fn branch(&self) -> HashView {
        self.branch.view()
    }

    fn nonce(&self) -> HashView {
        self.nonce.view()
    }
}

impl<'a> IntoTrits<Trit> for TransactionView<'a> {
    fn len_trits(&self) -> usize {
        TRANSACTION_LEN_TRITS
    }

    fn trits(&self) -> Vec<Trit> {
        self.0.to_vec()
    }
}

impl IntoTrits<Trit> for TransactionBuilder {
    fn len_trits(&self) -> usize {
        TRANSACTION_LEN_TRITS
    }

    fn trits(&self) -> Vec<Trit> {
        let mut trits = self.signature_or_message.clone();
        trits.reserve(self.len_trits() - self.signature_or_message.len());
        trits.append(&mut self.address.trits());
        trits.append(&mut num::int2trits(self.value, VALUE_TRITS as u8));
        trits.append(&mut self.tag.trits());
        trits.append(&mut num::int2trits(
            self.timestamp as isize,
            TIMESTAMP_TRITS as u8,
        ));
        trits.append(&mut num::int2trits(
            self.current_index as isize,
            CURRENT_INDEX_TRITS as u8,
        ));
        trits.append(&mut num::int2trits(
            self.last_index as isize,
            LAST_INDEX_TRITS as u8,
        ));
        trits.append(&mut self.bundle.trits());
        trits.append(&mut self.trunk.trits());
        trits.append(&mut self.branch.trits());
        trits.append(&mut self.nonce.trits());

        trits
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        TransactionBuilder {
            signature_or_message: Vec::new(),
            address: Hash::default(),
            value: 0,
            tag: Tag::default(),
            timestamp: 0,
            current_index: 0,
            last_index: 0,
            bundle: Hash::default(),
            trunk: Hash::default(),
            branch: Hash::default(),
            nonce: Hash::default(),
        }
    }
}

impl<'a> fmt::Debug for TransactionView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TransactionView").and_then(|_| self.fmt_tx(f))
    }
}

impl fmt::Debug for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Transaction").and_then(|_| self.fmt_tx(f))
    }
}

impl TransactionBuilder {
    pub fn set_signature_or_message(&mut self, value: &Vec<Trit>) -> Option<&mut Self> {
        if value.len() != MESSAGE_TRITS {
            None
        } else {
            self.signature_or_message = value.clone();
            Some(self)
        }
    }

    pub fn set_address(&mut self, hash: &Hash) -> &mut Self {
        self.address = hash.clone();
        self
    }

    pub fn set_value(&mut self, value: isize) -> &mut Self {
        self.value = value;
        self
    }

    pub fn set_tag(&mut self, hash: &Tag) -> &mut Self {
        self.tag = hash.clone();
        self
    }

    pub fn set_timestamp(&mut self, value: usize) -> &mut Self {
        self.timestamp = value;
        self
    }

    pub fn set_current_index(&mut self, value: usize) -> &mut Self {
        self.current_index = value;
        self
    }

    pub fn set_last_index(&mut self, value: usize) -> &mut Self {
        self.last_index = value;
        self
    }

    pub fn set_bundle(&mut self, hash: &Hash) -> &mut Self {
        self.bundle = hash.clone();
        self
    }

    pub fn set_trunk(&mut self, hash: &Hash) -> &mut Self {
        self.trunk = hash.clone();
        self
    }

    pub fn set_branch(&mut self, hash: &Hash) -> &mut Self {
        self.branch = hash.clone();
        self
    }

    pub fn set_nonce(&mut self, hash: &Hash) -> &mut Self {
        self.nonce = hash.clone();
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tx1() {

        const TX1: &'static str = "KGYTQSLYKIFRXCADXJFRC9PQSFDGVPX9HTIYZMMT9LDALXZK9CUPBRWLCIQC\
DRKOQPHDTDCOJZYYVCVQHOG9TVMGNRGHYZVO9FFMXKMEOAMJIJDEFJXFYPPE\
NPRHGTWYRDJBSGSHRURJRSTHHHVMQJVXJTCNEKJXEXUFDAJBNEIDMVCPHQBJ\
AOQLPDOBDCLBVYPLPPVOYXYLRJMRVVPYXGGYCZMEHVJDSSMPRYKYOYG9TQXH\
RUTPQMWJMMMSZ9ABUVJATBJ9ENCTPRTUDEDGKHKCCAGSFVYNJKSKAZDESJXC\
BXTLHKNSY9EDAOJAJCDMHASBNKXEOZSJDZRFEZUBWXXJTMKZADOAVLLBDFPW\
QGAAWNZTC9KPCWUJUOQLPD9NVQPHMVRWU9PXHR9VBPGXTLJENDCLLOSEHKXD\
AXRJGUFAHILMUNYLWSOARQEOUWSTQKLLDMWQ9BJAKQVRCCFFQVBUCWBJWGEN\
MTOX9WJDCWYTTPX9MZTABTBNQQKXHDQVSDUIQR9MRZYUHSKFSDFDCUJVDAFR\
BZTLITOLNGFAUNGNAOYT9XKVYUWYQIZTWSVJIOESIGHQVATXSMUTXPMDQATE\
HAXVXVCADIGINNZTGRWKAJHILGJFQZNQSHK9R9QLTSZTVNMBWIVVVLWBK9RY\
ZPZVRCDPSWVCGTFJCIGYHKOZKGJOOAYXRFCKKBQQBVYVHWMYNYFKYRNWQIGM\
QVIKAUOIKR9X9FMCZRZXZAK9RRIIYHQBV9YIPGLOHIYJWLHXHNUKZQVQFLGU\
SJNHKUJPZWYJCCJEXSWVGWTXRPMMTCQJRPJKFSZBBHJEFRGCUBDEQJIQGWTF\
ZZSKRFOFACOHGEXDYT9TIWZDGOWHHPT9PFAZVBPJSPRXKKEVGKPJOCRKXEGK\
TGYKFAYYXWLRMKLP9PQUNSXOLEGAYIJBGFPXLFRTCEDIOHURJOJTKGWQXZRO\
FWMIPSKAQUXPTAJAKMBPSMYPJBNIFEUIGGGNEHVSSO9PFNFORZLVVXBPDLAL\
WXXGTDEHJRBROVTCKFNR9JNFQKYBTJQSNQZRYZYMWLILMYDXCHFMVAXBVP9C\
ZZRNINNNRGFMRHSCUCZNOHPO9QJZAMJINJBIK9ICKOGZJZGLJLZLDV9IRQOV\
KGSPEGDLUKZXTHVZNJXWEJMSQZOCMWBVSKBGHBXJTLZQJDGEZBVMOCAIIFBT\
EV9IOHRO9HRYIW9OJFYECIQPISYFYOCLZDAHCWAKEZINHMGDIPOGANIPCAFC\
UCUQH9EABGYVIAYZD9TKMEZRQIECADWMVPZC9OZZU9FJZIPFAHLCMIUVWUCC\
XHNX9VAEGFSNYJKBZKPCYLXPWBDOR9EZATYHYZJVZSQFYHCCW9VDXJNXTDBO\
LNGLZANVYRXMBNBBXEUPCGZMYMEQDXHAWERKQSTXKXJZWGFMWIIVDAPBOMLM\
JSISPMJQWSCRZLBV9RUCJQQGVLQCUQCDNVAIGMSJGHM9NKHPZGUHTX9XJAKV\
GVYKXJK9HVFBSYSUIZI9WW9QEINDYBNWOUHAS99PVUGUKSHL9UYSUBTUTBNI\
CWCHTLTXWCOXUTYHWKSHZKJEGEBVYWYUDVXEXWVJOMJNNYCVCYLHKJZBYPXU\
QHESDKIOUGNCLNMTYQYXUFLOYP9KXRYXKWYDUEQXQ9XZFAYCFQCTDYMYWJLV\
EACJFBAUCUAXGCFMTXHTHMLTFPVZGONMCDMNBWPUTXDAFDNRPQWL9EMKEKLA\
NGBEUMKIJZBJVS9ABDHRPCJXVZWQUJFFHJHDKF9XFHMMLCKNABZEVPENTMNA\
YMRGYKMUTL9GDHWAILJTRCVKGPTBBVIIEBUKDAEBRUEQVLPDYGVUNUD9XMUE\
MXRAYTHSDJDLLJDMHSSGTIO9GRZPNYCBL9IZZOKXKB9DYTBYJLDAIP9SKVNT\
PADXQMAEHIDLHUQFESEXXUEANJTTFLOKGMXSPDASEO9FVOXPCRLDFVEHRKTO\
EWERZQESKQZKHJQQTIAFYNV9JDLTZUFGYAQWMURMATW9HFTIDTPBUJUF9ZNP\
CILKJCYRVIFDKCDHTPOSDOWRZQSQVJNURHWDDYQRSFEJFGZWZFRYASR9LZUK\
AXNR9JWSDTPCJXBJJJINFANYYMNTOMLEHWNKAQ9G9ZIH9YXKZURNQQYNLDRW\
KIFWQJZMEJGPXFCXGQMKBAEVZFBNEXHK9CRPMMIBGTEKUDHLKVGCZTOTCDZQ\
SXNXNSWIUWJKWQMXDLREAZBNXUCSDXYJBPGIWLQDJSZXSP9UHOZNDOW99999\
999999999999999999999999999999999999999999OVPNLWD99A99999999\
C99999999TPBVJTRLQQKZLASZWKTBZXS9JJVKLXQIRDRVPQEZTQBELCOBYIV\
WJZWXR9O9JHLQ9OUBQJXGKPTPEMYGVHKPDUEMBXJEUDPYJJAEQRBD9RRSIKB\
PS9LLPOUMWCTOUBKIEIANOZBDIXWWIMLFLZXPGKQWTLWVS99999YMJFFS9UB\
ZUM9FGQNBGTNBWNBTXQPMWCHNFNIJFSUIKCUFLF9BAWAWZZDYDSUC9MFVTWI\
SQGEUKP99999LPSI9VRPPHM9DRNWWZRVZWJGNIIRKYGKRHQNBNXNDIXPKDBX\
NVIFBFTPMCQZWTPDHUCGPC9FOXRVRYPCH";

        let tx1 = TX1.trits();
        let txview = TransactionView(tx1.as_slice());
    }
}
