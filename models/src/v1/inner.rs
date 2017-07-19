use core::fmt;
use core::ops::Deref;
use trytes::*;

typed_view!(Nonce, NonceParseError, NONCE_LEN_TRITS, 243);
