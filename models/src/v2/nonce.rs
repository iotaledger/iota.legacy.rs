use trytes::*;
use alloc::Vec;
use core::fmt;

pub const NONCE_LEN_TRITS: usize = 243;

#[derive(Clone, Eq, PartialEq)]
pub struct Nonce(Vec<Trit>);

#[derive(Clone, Eq, PartialEq)]
pub struct NonceView<'a>(&'a [Trit]);

#[derive(Debug, Eq, PartialEq)]
pub enum NonceParseError {
    InvalidLength,
}

impl IntoTrits<Trit> for Nonce {
    fn len_trits(&self) -> usize {
        self.0.len()
    }
    fn trits(&self) -> Vec<Trit> {
        self.0.clone()
    }
}

impl FromTrits<Trit> for Nonce {
    type Err = NonceParseError;
    fn from_trits(base: &[Trit]) -> Result<Self, Self::Err> {
        NonceView::from_trits(base).map(|v| v.to_nonce())
    }
}

impl Nonce {
    pub fn view(&self) -> NonceView {
        NonceView(self.0.as_slice())
    }
}

impl Default for Nonce {
    fn default() -> Self {
        use core::iter;
        Nonce(iter::repeat(0).take(NONCE_LEN_TRITS).collect())
    }
}
impl<'a> fmt::Display for NonceView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = trits_to_string(self.0).unwrap();
        f.write_str(&s)
    }
}

impl fmt::Display for Nonce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.view().fmt(f)
    }
}

impl fmt::Debug for Nonce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Nonce(")
            .and_then(|_| fmt::Display::fmt(self, f))
            .and_then(|_| f.write_str(")"))
    }
}

impl<'a> fmt::Debug for NonceView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NonceView(")
            .and_then(|_| fmt::Display::fmt(self, f))
            .and_then(|_| f.write_str(")"))
    }
}


impl<'a> NonceView<'a> {
    pub fn from_trits(base: &'a [Trit]) -> Result<Self, NonceParseError> {
        if base.len() != NONCE_LEN_TRITS {
            return Err(NonceParseError::InvalidLength);
        }
        Ok(NonceView(base))
    }

    pub fn to_nonce(&self) -> Nonce {
        Nonce(self.0.to_vec())
    }
}
