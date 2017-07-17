use cty::*;
use alloc::boxed::Box;
use alloc::Vec;

use iota_trytes::*;
use iota_sign::iss;

use util::c_str_to_static_slice;

#[no_mangle]
pub fn subseed(c_seed: *const c_char, index: usize) -> *const u8 {
    let seed_str = unsafe { c_str_to_static_slice(c_seed) };
    let seed: Vec<Trit> = seed_str.trits();

    let subseed = iss::subseed(&seed, index);

    let out_str = Box::new(trits_to_string(subseed.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn key(c_subseed: *const c_char) -> *const u8 {
    let subseed_str = unsafe { c_str_to_static_slice(c_subseed) };
    let subseed: Vec<Trit> = subseed_str.trits();

    let key = iss::key(&subseed);

    let out_str = Box::new(trits_to_string(key.trits().as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn digest_key(c_key: *const c_char) -> *const u8 {
    let key_str = unsafe { c_str_to_static_slice(c_key) };
    let key: Vec<Trit> = key_str.trits();

    let digest = iss::digest_key(&key);

    let out_str = Box::new(trits_to_string(digest.trits().as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn address(c_digest: *const c_char) -> *const u8 {
    let digest_str = unsafe { c_str_to_static_slice(c_digest) };
    let digest: Vec<Trit> = digest_str.trits();

    let address = iss::address(&digest);

    let out_str = Box::new(trits_to_string(address.trits().as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn signature(c_bundle: *const c_char, c_key: *const c_char) -> *const u8 {
    let key_str = unsafe { c_str_to_static_slice(c_key) };
    let key: Vec<Trit> = key_str.trits();

    let bundle_str = unsafe { c_str_to_static_slice(c_bundle) };
    let bundle: Vec<Trit> = bundle_str.trits();

    let signature = iss::signature(&bundle, &key);

    let out_str = Box::new(trits_to_string(signature.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn digest_bundle_signature(c_bundle: *const c_char, c_signature: *const c_char) -> *const u8 {
    let signature_str = unsafe { c_str_to_static_slice(c_signature) };
    let signature: Vec<Trit> = signature_str.trits();

    let bundle_str = unsafe { c_str_to_static_slice(c_bundle) };
    let bundle: Vec<Trit> = bundle_str.trits();

    let digest = iss::digest_bundle_signature(&bundle, &signature);

    let out_str = Box::new(trits_to_string(digest.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}
