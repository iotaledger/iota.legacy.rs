use globals::*;
use trit_bytes::*;
use trits_trytes::*;
use tryte_trits::*;
use bytes_trits::*;
use mappings::*;

pub struct Trinary {
    trytes: Option<String>,
    trits: Option<Vec<Trit>>,
    bytes: Option<Vec<i8>>,
}

impl Trinary {
    pub fn from_trytes(trytes: &str) -> Trinary {
        Trinary {
            trytes: Some(trytes.to_string()),
            trits: None,
            bytes: None,
        }
    }
    pub fn from_trits(trits: Vec<Trit>) -> Trinary {
        Trinary {
            trytes: None,
            trits: Some(trits),
            bytes: None,
        }
    }
    pub fn from_bytes(bytes: Vec<i8>) -> Trinary {
        Trinary {
            trytes: None,
            trits: None,
            bytes: Some(bytes),
        }
    }

    pub fn bytes(&self) -> Option<Vec<i8>> {
        match self.bytes {
            Some(ref x) => Some(x.to_vec()),
            None => {
                match self.trits {
                    Some(ref x) => Some(trits_to_bytes(x.as_slice())),
                    None => {
                        match self.trytes {
                            Some(ref x) => Some(trits_to_bytes(trytes_to_trits(x).as_slice())),
                            None => None,
                        }
                    }
                }
            }
        }
    }

    pub fn trytes(&self) -> Option<String> {
        match self.trytes {
            Some(ref x) => Some(x.to_string()),
            None => {
                match self.trits {
                    Some(ref x) => Some(trits_to_trytes(x.as_slice())),
                    None => {
                        match self.bytes {
                            Some(ref x) => {
                                Some(trits_to_trytes(bytes_to_trits(x.as_slice()).as_slice()))
                            }
                            None => None,
                        }
                    }
                }
            }
        }
    }
}
