use alloc::boxed::Box;

use iota_sign as sign;
use iota_trytes::*;
use iota_kerl::*;
use shared::*;

#[no_mangle]
pub static IOTA_SIGN_CHECKSUM_LEN : usize = sign::CHECKSUM_LEN;

#[no_mangle]
pub fn iota_sign_checksum(ctrits: &CTrits, kerl: &mut Kerl) -> *const CTrits {
    let mut checksum = vec![0 as Trit; sign::CHECKSUM_LEN];

    if ctrits.encoding == TritEncoding::TRIT {
        sign::checksum(ctrits_slice_trits(ctrits), &mut checksum, kerl);
    } else {
        sign::checksum(&ctrits_to_trits(ctrits), &mut checksum, kerl);
    }

    let out = Box::new(ctrits_from_trits(checksum));

    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_sign_checksum_validate(ctrits: &CTrits, kerl: &mut Kerl) -> u32 {
    let result = if ctrits.encoding == TritEncoding::TRIT {
        sign::checksum_validate(ctrits_slice_trits(ctrits), kerl)
    } else {
        sign::checksum_validate(&ctrits_to_trits(ctrits), kerl)
    };

    result as u32
}


