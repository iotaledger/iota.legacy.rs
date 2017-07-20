#![allow(dead_code)]
/// Type alias for `Trit`
pub type Trit = i8;
/// Binary coded trit, see http://homepage.divms.uiowa.edu/~jones/ternary/bct.shtml
pub type BCTrit = (usize, usize);
/// Default hash length in trits (81*3)
pub const HASH_LENGTH: usize = 243;
pub const RADIX: Trit = 3;
pub const MIN_TRYTE_VALUE: i8 = -13;
pub const MAX_TRYTE_VALUE: i8 = 13;
pub const MAX_TRIT_VALUE: Trit = (RADIX - 1) / 2;
pub const MIN_TRIT_VALUE: Trit = -MAX_TRIT_VALUE;
pub const TRINARY_LENGTH: usize = HASH_LENGTH / (RADIX as usize);
pub const TRITS_PER_BYTE: usize = 5;
pub const TRITS_PER_TRYTE: usize = 3;
/// Valid tryte alphabet
pub const TRYTE_SPACE: usize = 27;
pub const TRYTE_ALPHABET: [char; TRYTE_SPACE] = ['9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
