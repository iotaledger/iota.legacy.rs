pub mod bundle;
pub mod get;
pub mod set;
pub mod helpers;

pub use self::bundle::*;
pub use self::get::*;
pub use self::set::*;
pub use self::helpers::*;

use iota_models::v1;
#[no_mangle]
pub static IOTA_MODELS_V2_TX_LEN : usize = v1::constants::TRANSACTION_LEN_TRITS;
#[no_mangle]
pub static IOTA_MODELS_V2_MESSAGE_LEN : usize = v1::constants::MESSAGE_TRITS;

