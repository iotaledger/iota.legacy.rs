use alloc::vec::Vec;
use core::array::FixedSizeArray;
use core::result::Result;

use trytes::*;
use curl::*;
use curl_cpu::*;

pub const CHECKSUM_LEN: usize = 9;
const CHECKSUM_TRITS: usize = CHECKSUM_LEN * TRITS_PER_TRYTE;

pub trait Checksum {
    fn checksum(&self) -> Trinary;
    fn with_checksum(&self) -> Trinary;
}

impl<T> Checksum for T
where
    T: IntoTrinary,
{
    fn checksum(&self) -> Trinary {
        let mut curl = CpuCurl::<Trit>::default();
        let trits: Vec<Trit> = self.trinary().trits();
        curl.absorb(&trits);
        let hashed: Trinary = curl.squeeze(CHECKSUM_TRITS).into_iter().collect();
        hashed
    }

    fn with_checksum(&self) -> Trinary {
        [self.trinary(), self.checksum()].as_slice().trinary()
    }
}

impl Checksum for [Trit] {
    fn checksum(&self) -> Trinary {
        let mut curl = CpuCurl::<Trit>::default();
        curl.absorb(&self);
        let hashed: Trinary = curl.squeeze(CHECKSUM_TRITS).into_iter().collect();
        hashed
    }

    fn with_checksum(&self) -> Trinary {
        [self.iter().cloned().collect(), self.checksum()]
            .as_slice()
            .trinary()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ChecksumValidationError {
    /// Given Trinary is not of `> 9` trytes.
    InvalidLength,
    /// Checksum did not match input.
    InvalidChecksum,
}

trait FromTrinaryWithChecksum<T>
where
    T: FromTrinary,
{
    /// Verifies a Checksum. If it's valid it'll call `T::from_trinary` with the input trinary without the final checksum.
    fn from_trinary_with_checksum(
        trinary: &Trinary,
    ) -> Result<Result<T, T::Err>, ChecksumValidationError>;
}

impl<T> FromTrinaryWithChecksum<T> for T
where
    T: FromTrinary,
{
    fn from_trinary_with_checksum(
        trinary: &Trinary,
    ) -> Result<Result<T, T::Err>, ChecksumValidationError> {
        if trinary.len_trytes() <= CHECKSUM_LEN {
            return Err(ChecksumValidationError::InvalidLength);
        }

        let trits: Vec<Trit> = trinary.trits();
        let (base, provided_checksum) = trits.split_at(trits.len() - CHECKSUM_TRITS);

        // Validate that input checksum matches computed checksum
        let base_checksum: Vec<Trit> = base.checksum().trits();
        if base_checksum != provided_checksum {
            return Err(ChecksumValidationError::InvalidChecksum);
        }

        // Checksums are valid.
        Ok(Self::from_trinary(&base.iter().cloned().collect()))
    }
}

trait ToTrinaryWithChecksum<T>
where
    T: IntoTrinary,
{
    fn to_trinary_with_checksum(&self) -> Trinary;
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct MyModel(Trinary);

    #[derive(Debug, PartialEq, Eq)]
    enum FromTrinaryError {
        SomeError,
    }

    impl FromTrinary for MyModel {
        type Err = FromTrinaryError;
        fn from_trinary(_: &Trinary) -> Result<Self, Self::Err> {
            Err(FromTrinaryError::SomeError)
        }
    }

    #[test]
    fn my_model_from() {
        let t: Trinary = "ABC".chars().collect();
        let res = MyModel::from_trinary(&t);
        let res_checksum = MyModel::from_trinary_with_checksum(&t);

        assert_eq!(res.unwrap_err(), FromTrinaryError::SomeError);
        assert_eq!(
            res_checksum.unwrap_err(),
            ChecksumValidationError::InvalidLength
        );
    }

    #[test]
    fn checksum_test_1() {
        let c: Trinary = "FOXM9MUBX".chars().collect();
        let t: Trinary = "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA"
            .chars()
            .collect();
        let combined = "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVAFOXM9MUBX"
            .chars()
            .collect();

        assert_eq!(t.checksum(), c);
        assert_eq!(t.with_checksum(), combined);
    }

    #[test]
    fn checksum_test_2() {
        let c: Trinary = "9QTIWOWTY".chars().collect();
        let t: Trinary = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL"
            .chars()
            .collect();
        let combined = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTY"
            .chars()
            .collect();

        assert_eq!(t.checksum(), c);
        assert_eq!(t.with_checksum(), combined);
    }

    #[test]
    fn from_trinary_with_checksum_valid() {
        let combined = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTY"
            .chars()
            .collect();
        let ex: Trinary = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL"
            .chars()
            .collect();

        let result = Trinary::from_trinary_with_checksum(&combined);
        let inner = result.unwrap();

        assert_eq!(ex, inner.unwrap());
    }

    #[test]
    fn from_trinary_with_checksum_invalid() {
        let combined = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTX"
            .chars()
            .collect();

        let result = Trinary::from_trinary_with_checksum(&combined);

        assert_eq!(result, Err(ChecksumValidationError::InvalidChecksum));
    }

    #[test]
    fn from_trinary_with_checksum_length() {
        let combined = "KTX".chars().collect();

        let result = Trinary::from_trinary_with_checksum(&combined);

        assert_eq!(result, Err(ChecksumValidationError::InvalidLength));
    }
}
