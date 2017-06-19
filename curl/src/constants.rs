use trytes::HASH_LENGTH;
use trytes::Trit;

pub const STATE_LENGTH: usize = HASH_LENGTH * 3;
pub const TRUTH_TABLE: [Trit; 9] = [1, 0, -1, 1, -1, 0, -1, 1, 0];
pub const NUMBER_OF_ROUNDS: usize = 27;
