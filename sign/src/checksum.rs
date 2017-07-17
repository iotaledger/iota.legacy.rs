use alloc::vec::Vec;
use core::result::Result;

use trytes::*;
use curl::*;
use curl_cpu::*;

pub const CHECKSUM_LEN: usize = 9;
const CHECKSUM_TRITS: usize = CHECKSUM_LEN * TRITS_PER_TRYTE;

pub trait Checksum {
    fn checksum(&self) -> Vec<Trit>;
    fn with_checksum(&self) -> Vec<Trit>;
}

impl<T> Checksum for T
where
    T: IntoTrits<Trit>,
{
    fn checksum(&self) -> Vec<Trit> {
        let mut curl = CpuCurl::<Trit>::default();
        let trits: Vec<Trit> = self.trits();
        curl.absorb(&trits);
        curl.squeeze(CHECKSUM_TRITS)
    }

    fn with_checksum(&self) -> Vec<Trit> {
        let mut this = self.trits();
        this.append(&mut self.checksum());

        this
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
    T: FromTrits<Trit>,
{
    /// Verifies a Checksum. If it's valid it'll call `T::from_trits` with the input trinary without the final checksum.
    fn from_trits_with_checksum(
        trinary: &IntoTrits<Trit>,
    ) -> Result<Result<T, T::Err>, ChecksumValidationError>;
}

impl<T> FromTrinaryWithChecksum<T> for T
where
    T: FromTrits<Trit>,
{
    fn from_trits_with_checksum(
        trinary: &IntoTrits<Trit>,
    ) -> Result<Result<T, T::Err>, ChecksumValidationError> {
        if trinary.len_trits() <= CHECKSUM_LEN {
            return Err(ChecksumValidationError::InvalidLength);
        }

        let trits: Vec<Trit> = trinary.trits();
        let (base, provided_checksum) = trits.split_at(trits.len() - CHECKSUM_TRITS);

        // Validate that input checksum matches computed checksum
        let base_checksum: Vec<Trit> = base.checksum();
        if base_checksum != provided_checksum {
            return Err(ChecksumValidationError::InvalidChecksum);
        }

        // Checksums are valid.
        Ok(Self::from_trits(&base))
    }
}

trait ToTrinaryWithChecksum<T>
where
    T: IntoTrits<Trit>,
{
    fn to_trinary_with_checksum(&self) -> Vec<Trit>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct MyModel(Vec<Trit>);

    #[derive(Debug, PartialEq, Eq)]
    enum FromTrinaryError {
        SomeError,
    }

    impl FromTrits<Trit> for MyModel {
        type Err = FromTrinaryError;
        fn from_trits(_: &[Trit]) -> Result<Self, Self::Err> {
            Err(FromTrinaryError::SomeError)
        }
    }

    #[test]
    fn my_model_from() {
        let t: Vec<Trit> = "ABC".trits();
        let res = MyModel::from_trits(&t);
        let res_checksum = MyModel::from_trits_with_checksum(&t);

        assert_eq!(res.unwrap_err(), FromTrinaryError::SomeError);
        assert_eq!(
            res_checksum.unwrap_err(),
            ChecksumValidationError::InvalidLength
        );
    }

    #[test]
    fn checksum_test_1() {
        let c: Vec<Trit> = "FOXM9MUBX".trits();
        let t: Vec<Trit> = "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA"
            .trits();
        let combined: Vec<Trit> = "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVAFOXM9MUBX"
            .trits();

        assert_eq!(t.checksum(), c);
        assert_eq!(t.with_checksum(), combined);
    }

    #[test]
    fn checksum_test_2() {
        let c: Vec<Trit> = "9QTIWOWTY".trits();
        let t: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL"
            .trits();
        let combined: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTY"
            .trits();

        assert_eq!(t.checksum(), c);
        assert_eq!(t.with_checksum(), combined);
    }

    #[test]
    fn from_trits_with_checksum_valid() {
        let combined: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTY"
            .trits();
        let ex: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL"
            .trits();

        let result = Vec::<Trit>::from_trits_with_checksum(&combined);
        let inner = result.unwrap();

        assert_eq!(ex, inner.unwrap());
    }

    #[test]
    fn from_trits_with_checksum_invalid() {
        let combined: Vec<Trit> = "KTXFP9XOVMVWIXEWMOISJHMQEXMYMZCUGEQNKGUNVRPUDPRX9IR9LBASIARWNFXXESPITSLYAQMLCLVTL9QTIWOWTX"
            .trits();

        let result = Vec::<Trit>::from_trits_with_checksum(&combined);

        assert_eq!(result, Err(ChecksumValidationError::InvalidChecksum));
    }

    #[test]
    fn from_trits_with_checksum_length() {
        let combined: Vec<Trit> = "KTX".trits();

        let result = Vec::<Trit>::from_trits_with_checksum(&combined);

        assert_eq!(result, Err(ChecksumValidationError::InvalidLength));
    }
}
