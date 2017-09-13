use curl::*;
use kerl::Kerl;
use trytes::TRITS_PER_TRYTE;
use trytes::HASH_LENGTH;
use trytes::Trit;

pub const CHECKSUM_TRYTES: usize = 9;
pub const CHECKSUM_LEN: usize = CHECKSUM_TRYTES * TRITS_PER_TRYTE;

pub fn trits_checksum(t: &[Trit], out: &mut [Trit], kerl: &mut Kerl) {
    assert_eq!(out.len(), CHECKSUM_LEN);

    let mut trits = [0 as Trit; HASH_LENGTH];

    kerl.reset();
    kerl.absorb(t);
    kerl.squeeze(&mut trits);

    out.clone_from_slice(&trits[HASH_LENGTH - CHECKSUM_LEN..HASH_LENGTH]);
}

pub fn trits_without_checksum<'a, T>(t: &'a [T]) -> &'a [T] {
    &t[0..t.len() - CHECKSUM_LEN]
}

pub fn trits_validate_checksum(t: &[Trit], kerl: &mut Kerl) -> Option<ChecksumValidationError> {
    use ChecksumValidationError::*;

    if t.len() <= CHECKSUM_LEN {
        return Some(InvalidLength);
    }

    let (body, rest) = t.split_at(t.len() - CHECKSUM_LEN);

    let mut checksum = [0 as Trit; CHECKSUM_LEN];
    trits_checksum(body, &mut checksum, kerl);

    if rest != checksum {
        return Some(InvalidChecksum);
    }

    None
}

#[derive(Debug, Eq, PartialEq)]
pub enum ChecksumValidationError {
    /// Given Trinary is not of `> 9` trytes.
    InvalidLength,
    /// Checksum did not match input.
    InvalidChecksum,
}


#[cfg(test)]
mod test {
    use super::*;
    use kerl::*;
    use trytes::*;
    use alloc::vec::Vec;


    #[test]
    fn checksum_invalid() {
        let combined: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTX"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let mut curl = Kerl::default();
        assert_eq!(
            trits_validate_checksum(combined.as_slice(), &mut curl),
            Some(ChecksumValidationError::InvalidChecksum)
        );
    }

    #[test]
    fn checksum_invalid_length() {
        let combined: Vec<Trit> = "KTX".chars().flat_map(char_to_trits).cloned().collect();
        let mut curl = Kerl::default();

        assert_eq!(
            trits_validate_checksum(combined.as_slice(), &mut curl),
            Some(ChecksumValidationError::InvalidLength)
        );
    }

    #[test]
    fn checksum_kerl_split() {
        let c: Vec<Trit> = "ODCNSCYJD"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let t: Vec<Trit> = "P9UDUZMN9DEXCRQEKLJYSBSBZFCHOBPJSDKMLCCVJDOVOFDWMNBZRIRRZJGINOUMPJBMYYZEGRTIDUABD"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let mut checksum = [0 as Trit; CHECKSUM_LEN];
        let mut curl = Kerl::default();
        trits_checksum(t.as_slice(), &mut checksum, &mut curl);
        assert_eq!(c.as_slice(), checksum);
    }

    #[test]
    fn checksum_kerl() {
        let combined: Vec<Trit> = "EUHMAFIYBYZOXAVQQYRQ9RCNMTYX9KNEZFWXYMQIYPSRZRVDOLXDPUEARYPTWSZCAXJLXRYUUQKSHIJYZICCXCXUHX"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let mut kerl = Kerl::default();
        assert_eq!(trits_validate_checksum(&combined, &mut kerl), None);
    }
}
