use trytes::*;
use errors::*;

/// All implementations of the IOTA proof of work algorithm must
/// implement this trait and conform to the test suite.
///
/// # Arguments
///
/// * `transaction` - The transaction encoded as a `Trinary`.
///   TODO Note that this will be changed to use the proper `Transaction` model class in the future.
/// * `mwm` - minimum weight magnitude
pub trait PoW {
    fn search_nonce(transaction: Trinary, mwm: u8) -> Result<Trinary>;
}
