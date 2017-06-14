pub type Trit = i32;
#[allow(dead_code)]
pub const HASH_LENGTH: usize = 243;
#[allow(dead_code)]
pub const TRYTE_SPACE: usize = 27;
#[allow(dead_code)]
pub const MIN_TRYTE_VALUE: i8 = -13;
#[allow(dead_code)]
pub const MAX_TRYTE_VALUE: i8 = 13;
pub const RADIX: Trit = 3;
#[allow(dead_code)]
pub const MAX_TRIT_VALUE: Trit = (RADIX - 1) / 2;
#[allow(dead_code)]
pub const MIN_TRIT_VALUE: Trit = -MAX_TRIT_VALUE;
#[allow(dead_code)]
pub const NUMBER_OF_TRITS_IN_A_BYTE: usize = 5;
#[allow(dead_code)]
pub const NUMBER_OF_TRITS_IN_A_TRYTE: usize = 3;
#[allow(dead_code)]
pub const TRYTE_ALPHABET: &'static str = "9ABCDEFGHIJKLMNOPQRSTUVWXYZ";
