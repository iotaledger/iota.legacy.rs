use cty::*;
use alloc::string::ToString;
use alloc::boxed::Box;

use iota_trytes::*;
use iota_sign::iss;
use iota_curl_cpu::*;

use util::c_str_to_static_slice;

#[no_mangle]
pub fn subseed(c_seed: *const c_char, index: usize) -> *const u8 {
    let seed_str = unsafe { c_str_to_static_slice(c_seed) };
    let seed: Trinary = seed_str.chars().collect();

    let subseed = iss::subseed::<CpuCurl<Trit>>(&seed.trits(), index);

    let out_str = Box::new(subseed.to_string() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn key(c_subseed: *const c_char) -> *const u8 {
    let subseed_str = unsafe { c_str_to_static_slice(c_subseed) };
    let subseed: Trinary = subseed_str.chars().collect();

    let key: Trinary = iss::key::<Trit, CpuCurl<Trit>>(&subseed.trits())
        .into_iter()
        .collect();

    let out_str = Box::new(key.to_string() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn digest_key(c_key: *const c_char) -> *const u8 {
    let key_str = unsafe { c_str_to_static_slice(c_key) };
    let key: Trinary = key_str.chars().collect();

    let digest: Trinary = iss::digest_key::<Trit, CpuCurl<Trit>>(&key.trits())
        .into_iter()
        .collect();

    let out_str = Box::new(digest.to_string() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn address(c_digest: *const c_char) -> *const u8 {
    let digest_str = unsafe { c_str_to_static_slice(c_digest) };
    let digest: Trinary = digest_str.chars().collect();

    let address: Trinary = iss::address::<Trit, CpuCurl<Trit>>(&digest.trits())
        .into_iter()
        .collect();

    let out_str = Box::new(address.to_string() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn signature(c_bundle: *const c_char, c_key: *const c_char) -> *const u8 {
    let key_str = unsafe { c_str_to_static_slice(c_key) };
    let key: Trinary = key_str.chars().collect();

    let bundle_str = unsafe { c_str_to_static_slice(c_bundle) };
    let bundle: Trinary = bundle_str.chars().collect();

    let signature = iss::signature::<CpuCurl<Trit>>(bundle, key);

    let out_str = Box::new(signature.to_string() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn digest_bundle_signature(c_bundle: *const c_char, c_signature: *const c_char) -> *const u8 {
    let signature_str = unsafe { c_str_to_static_slice(c_signature) };
    let signature: Trinary = signature_str.chars().collect();

    let bundle_str = unsafe { c_str_to_static_slice(c_bundle) };
    let bundle: Trinary = bundle_str.chars().collect();

    let digest = iss::digest_bundle_signature::<CpuCurl<Trit>>(bundle, signature);

    let out_str = Box::new(digest.to_string() + "\0");
    &out_str.as_bytes()[0] as *const u8
}
