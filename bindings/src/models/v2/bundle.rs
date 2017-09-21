use alloc::Vec;
use alloc::boxed::Box;
use core::slice;
use shared::*;

use iota_kerl::Kerl;
use iota_models::v2;


#[no_mangle]
pub fn iota_models_v2_bundle_hash(
    ctrits_ptr: *const CTrits,
    ctrits_len: usize,
    kerl: &mut Kerl,
) -> *const CTrits {
    let ctrits: &[CTrits] = unsafe { slice::from_raw_parts(ctrits_ptr, ctrits_len) };

    assert_eq!(
        0,
        ctrits
            .iter()
            .filter(|&t| t.encoding != TritEncoding::TRIT)
            .count()
    );

    let txviews: Vec<v2::TransactionView> = ctrits
        .iter()
        .map(|ct| {
            v2::TransactionView::from_trits(ctrits_slice_trits(ct)).unwrap()
        })
        .collect();


    let hash = v2::bundle_hash(&txviews, kerl);


    let out = Box::new(ctrits_from_trits(hash.to_vec()));
    Box::into_raw(out)
}
