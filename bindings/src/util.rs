use core::mem;
use alloc::slice::from_raw_parts;
use cty::*;
/// Form a slice from a C string. Unsafe because the caller must ensure the
/// C string has the static lifetime, or else the return value may be
/// invalidated later.
///
/// From: https://github.com/japaric/dstr.rs/blob/master/src/raw.rs
pub unsafe fn c_str_to_static_slice(s: *const c_char) -> &'static str {
    let mut curr = s;
    let mut len: usize = 0;
    while *curr != 0 {
        len += 1;
        curr = s.offset(len as isize);
    }
    let v = from_raw_parts(s, len);
    mem::transmute(v)
}
