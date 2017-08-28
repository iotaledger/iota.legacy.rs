use alloc::boxed::Box;
use shared::*;
use cty::c_void;

use iota_curl::*;
use iota_trytes::*;
use iota_kerl::*;

#[no_mangle]
pub fn iota_kerl_trit_new() -> *const Kerl {
    let kerl = Box::new(Kerl::default());
    Box::into_raw(kerl)
}

#[no_mangle]
pub fn iota_kerl_trit_delete(c_kerl: *mut Kerl) {
    // Deallocate c_kerl
    unsafe { Box::from_raw(c_kerl) };
}

#[no_mangle]
pub fn iota_kerl_trit_absorb(c_kerl: &mut Kerl, trinary: &CTrits) {
    if trinary.encoding == TritEncoding::TRIT {
        c_kerl.absorb(ctrits_slice_trits(trinary));
    } else {
        c_kerl.absorb(&ctrits_to_trits(trinary));
    }
}

#[no_mangle]
pub fn iota_kerl_trit_reset(c_kerl: &mut Kerl) {
    c_kerl.reset();
}

#[no_mangle]
pub fn iota_kerl_trit_squeeze(c_kerl: &mut Kerl, trit_count: usize) -> *const CTrits {
    let mut trits = vec![0 as Trit; trit_count];
    c_kerl.squeeze(&mut trits);

    let ctrits = Box::new(ctrits_from_trits(trits));
    Box::into_raw(ctrits)
}

#[no_mangle]
pub fn iota_kerl_trit_state(c_kerl: &Kerl) -> *const CTrits {
    let ptr = c_kerl.byte_state().as_ptr() as *mut c_void;
    let len = c_kerl.byte_state().len();

    Box::into_raw(Box::new(CTrits {
        encoding: TritEncoding::BYTE,
        length: len,
        data: ptr,
        byte_length: len
    }))
}
