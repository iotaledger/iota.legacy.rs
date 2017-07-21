use curl::*;
use trytes::TRITS_PER_TRYTE;

pub const CHECKSUM_TRYTES : usize = 9;
pub const CHECKSUM_LEN: usize = CHECKSUM_TRYTES * TRITS_PER_TRYTE;

pub fn trits_checksum<'a, T, C>(t: &'a [T], out: &mut [T], curl: &mut C)
where
    T: Clone + Copy + Sized,
    C: Curl<T>,
{
    assert_eq!(out.len(), CHECKSUM_LEN);
    curl.absorb(t);
    curl.squeeze(out)
}

pub fn trits_without_checksum<'a, T>(t: &'a [T]) -> &'a [T] {
    &t[0..t.len() - CHECKSUM_LEN]
}

pub fn trits_validate_checksum<'a, T, C>(t: &'a [T], curl: &mut C) -> Option<ChecksumValidationError>
where
    T: Clone + Copy + Sized + PartialEq,
    C: Curl<T>
{
    use ChecksumValidationError::*;

    if t.len() <= CHECKSUM_LEN {
        return Some(InvalidLength);
    }


    let (body, rest) = t.split_at(t.len() - CHECKSUM_LEN);

    let mut checksum = [t[0]; CHECKSUM_LEN];
    trits_checksum(body, &mut checksum, curl);

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
    use curl_cpu::*;
    use trytes::*;
    use alloc::vec::Vec;

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

        let mut checksum = [0 as Trit; CHECKSUM_LEN];
        let mut curl = CpuCurl::<Trit>::default();
        trits_checksum(t.as_slice(), &mut checksum, &mut curl);
        assert_eq!(c.as_slice(), checksum);
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

        let mut checksum = [0 as Trit; CHECKSUM_LEN];
        let mut curl = CpuCurl::<Trit>::default();
        trits_checksum(t.as_slice(), &mut checksum, &mut curl);
        assert_eq!(c.as_slice(), checksum);
    }

    #[test]
    fn checksum_invalid() {
        let combined: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTX"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let mut curl = CpuCurl::<Trit>::default();
        assert_eq!(
            trits_validate_checksum(combined.as_slice(), &mut curl),
            Some(ChecksumValidationError::InvalidChecksum)
        );
    }

    #[test]
    fn checksum_invalid_length() {
        let combined: Vec<Trit> = "KTX".chars().flat_map(char_to_trits).cloned().collect();
        let mut curl = CpuCurl::<Trit>::default();

        assert_eq!(
            trits_validate_checksum(combined.as_slice(), &mut curl),
            Some(ChecksumValidationError::InvalidLength)
        );
    }
}
