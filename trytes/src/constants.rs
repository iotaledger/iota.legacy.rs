/// Type alias for `Trit`
pub type Trit = i8;
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
pub const TRYTE_ALPHABET: &str = "9ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const TRYTE_SPACE: usize = 27;
