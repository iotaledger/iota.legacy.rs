use trytes::*;

pub enum PoWError {
    /// Input trinary is not of `TRANSACTION_LENGTH`
    InvalidInputSize,
    /// Min weight magnitude exceeds `HASH_LENGT``
    InvalidMinWeightMagnitude,
    /// Custom implementation error
    CustomError(String)
}

/// All implementations of the IOTA proof of work algorithm must
/// implement this trait and conform to the test suite.
///
/// # Arguments
///
/// * `transaction` - The transaction encoded as a `Trinary`.
///   TODO Note that this will be changed to use the proper `Transaction` model class in the future.
/// * `mwm` - minimum weight magnitude
pub trait PoW {
    fn search_nonce(transaction: Trinary, mwm: u8) -> Result<Trinary, PoWError>;
}

pub fn offset(&mut curl: DefaultCurl) {
    ///... offset, to be based on usize
    curl.state[0].0 = 0b1101101101101101101101101101101101101101101101101101101101101101L;
    curl.state[0].1 = 0b1011011011011011011011011011011011011011011011011011011011011011L;
    curl.state[1].0 = 0b1111000111111000111111000111111000111111000111111000111111000111L;
    curl.state[1].1 = 0b1000111111000111111000111111000111111000111111000111111000111111L;
    curl.state[2].0 = 0b0111111111111111111000000000111111111111111111000000000111111111L;
    curl.state[2].1 = 0b1111111111000000000111111111111111111000000000111111111111111111L;
    curl.state[3].0 = 0b1111111111000000000000000000000000000111111111111111111111111111L;
    curl.state[3].1 = 0b0000000000111111111111111111111111111111111111111111111111111111L;
}

pub fn increment(&mut state: &[BCTrit]) {
    for i in 0..state.len() {
        let (ref low, ref hi) = state[i];
        state.0 = hi ^ low;
        state.1 = low;
        if hi & !low { break; }
    }
}

pub fn search_state(&mut curl: DefaultCurl, group: usize, length: u32, check: |&[BCTrit]| -> i32) -> Trinary {
    offset(curl);
    for _ in 0..group {
        increment(&curl.state[HASH_LENGTH/3..HASH_LENGTH*2/3]);
    }
    let mut index;
    loop {
        increment(&curl.state[HASH_LENGTH*2/3..HASH_LENGTH]);
        let curl_copy = curl.clone();
        curl_copy.transform();
        index = check(&curl_copy.state[0..length]);
        if index != 0 {
            break;
        }
    }
    TrinaryMultiplexer::from(curl.squeeze(HASH_LENGTH))[index - 1]
}

