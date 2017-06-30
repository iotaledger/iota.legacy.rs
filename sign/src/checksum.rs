use core::array::FixedSizeArray;
use core::result::Result;

use trytes::*;

pub const CHECKSUM_LEN: usize = 6;

trait Checksum {
    fn checksum(&self) -> Trinary;
    fn with_checksum(&self) -> Trinary;
}

impl Checksum for IntoTrinary {
    fn checksum(&self) -> Trinary {
        "999999".chars().collect()
    }

    fn with_checksum(&self) -> Trinary {
        [self.trinary(), self.checksum()].as_slice().trinary()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ChecksumValidationError {
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
        // XXX - verify checksum.
        Err(ChecksumValidationError::InvalidChecksum)
        //Self::from_trinary(t)
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
        fn from_trinary(t: &Trinary) -> Result<Self, Self::Err> {
            Err(FromTrinaryError::SomeError)
        }
    }

    #[test]
    fn my_model_from() {
        let t: Trinary = "ABC".chars().collect();
        let res = MyModel::from_trinary(&t);
        let res_checksum = MyModel::from_trinary_with_checksum(&t);

        assert_eq!(res.unwrap_err(), FromTrinaryError::SomeError);
        assert_eq!(res_checksum.unwrap_err(), ChecksumValidationError::InvalidChecksum);
    }
}
