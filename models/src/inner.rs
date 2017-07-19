use core::fmt;
use core::ops::Deref;
use trytes::*;

typed_view!(Tag, TagParseError, TAG_LEN_TRITS, 81);
typed_view!(Hash, HashParseError, HASH_LEN_TRITS, 243);
