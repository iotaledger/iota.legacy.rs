use trytes::*;

pub const TAG_LEN_TRITS: usize = 81;
typed_view!(TagView, TAG_LEN_TRITS);
typed_own!(Tag, TagView, TAG_LEN_TRITS);

pub const HASH_LEN_TRITS: usize = 243;
typed_view!(HashView, HASH_LEN_TRITS);
typed_own!(Hash, HashView, HASH_LEN_TRITS);
