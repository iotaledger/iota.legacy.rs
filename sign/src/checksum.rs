use alloc::vec::Vec;

use trytes::*;
use curl::*;
use curl_cpu::*;

pub const CHECKSUM_LEN: usize = 9;
const CHECKSUM_TRITS: usize = CHECKSUM_LEN * TRITS_PER_TRYTE;

pub fn trits_checksum<'a, T>(t: &'a [T]) -> Vec<T>
where
    T: Clone + Copy + Sized,
    CpuCurl<T>: Curl<T>,
{
    let mut curl = CpuCurl::<T>::default();
    curl.absorb(t);
    curl.squeeze(CHECKSUM_TRITS)
}

pub fn trits_with_checksum<'a, T>(t: &'a [T]) -> Vec<T>
where
    T: Clone + Copy + Sized,
    CpuCurl<T>: Curl<T>,
{
    let mut tc = t.to_vec();
    tc.append(&mut trits_checksum(t));
    tc
}

pub fn trits_without_checksum<'a, T>(t: &'a [T]) -> &'a [T] {
    &t[0..t.len() - CHECKSUM_LEN]
}

pub fn trits_validate_checksum<'a, T>(t: &'a [T]) -> Option<ChecksumValidationError>
where
    T: Clone + Copy + Sized,
    CpuCurl<T>: Curl<T>,
    Vec<T>: PartialEq,
{
    use ChecksumValidationError::*;

    if t.len() <= CHECKSUM_LEN {
        return Some(InvalidLength);
    }

    let (body, rest) = t.split_at(t.len() - CHECKSUM_LEN);

    if !(trits_checksum(body) == rest.to_vec()) {
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


    #[test]
    fn checksum_test_1() {
        let c: Vec<Trit> = "FOXM9MUBX"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let t: Vec<Trit> = "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let combined: Vec<Trit> = "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVAFOXM9MUBX"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        assert_eq!(trits_checksum(t.as_slice()), c);
        assert_eq!(trits_with_checksum(t.as_slice()), combined);
    }

    #[test]
    fn checksum_test_2() {
        let c: Vec<Trit> = "9QTIWOWTY"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let t: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let combined: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTY"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        assert_eq!(trits_checksum(t.as_slice()), c);
        assert_eq!(trits_with_checksum(t.as_slice()), combined);
    }

    #[test]
    fn checksum_invalid() {
        let combined: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTX"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        assert_eq!(
            trits_validate_checksum(combined.as_slice()),
            Some(ChecksumValidationError::InvalidChecksum)
        );
    }

    #[test]
    fn checksum_invalid_length() {
        let combined: Vec<Trit> = "KTX".chars().flat_map(char_to_trits).cloned().collect();

        assert_eq!(
            trits_validate_checksum(combined.as_slice()),
            Some(ChecksumValidationError::InvalidLength)
        );
    }
}
