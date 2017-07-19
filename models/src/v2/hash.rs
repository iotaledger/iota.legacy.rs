use trytes::*;
use alloc::Vec;

pub const HASH_LEN_TRITS: usize = 243;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hash(Vec<Trit>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HashView<'a>(&'a [Trit]);

#[derive(Debug, Eq, PartialEq)]
pub enum HashParseError {
    InvalidLength,
}

impl IntoTrits<Trit> for Hash {
    fn len_trits(&self) -> usize {
        self.0.len()
    }
    fn trits(&self) -> Vec<Trit> {
        self.0.clone()
    }
}

impl FromTrits<Trit> for Hash {
    type Err = HashParseError;
    fn from_trits(base: &[Trit]) -> Result<Self, Self::Err> {
        HashView::from_trits(base).map(|v| v.to_hash())
    }
}

impl Hash {
    pub fn view(&self) -> HashView {
       HashView(self.0.as_slice()) 
    }
}

impl Default for Hash {
    fn default() -> Self {
        use core::iter;
        Hash(iter::repeat(0).take(HASH_LEN_TRITS).collect())
    }
}

impl<'a> HashView<'a> {
    pub fn from_trits(base: &'a [Trit]) -> Result<Self, HashParseError> {
        if base.len() != HASH_LEN_TRITS {
            return Err(HashParseError::InvalidLength);
        }
        Ok(HashView(base))
    }

    pub fn to_hash(&self) -> Hash {
        Hash(self.0.to_vec())
    }
}
