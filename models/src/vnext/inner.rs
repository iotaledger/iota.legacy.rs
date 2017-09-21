use trytes::*;

pub const NONCE_LEN_TRITS : usize = 81;
typed_view!(NonceView, NONCE_LEN_TRITS);
typed_own!(Nonce, NonceView, NONCE_LEN_TRITS);
