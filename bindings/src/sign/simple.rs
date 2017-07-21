use cty::*;
use alloc::boxed::Box;
use alloc::Vec;

use iota_trytes::*;
use iota_sign::iss;
use iota_curl_cpu::*;

use util::c_str_to_static_slice;

#[no_mangle]
pub fn subseed(c_seed: *const c_char, index: usize) -> *const u8 {
    let seed_str = unsafe { c_str_to_static_slice(c_seed) };
    let seed: Vec<Trit> = seed_str.chars().flat_map(char_to_trits).cloned().collect();


    let mut subseed = vec![0; seed.len()];
    iss::subseed::<CpuCurl<Trit>>(&seed, index, &mut subseed);

    let out_str = Box::new(trits_to_string(subseed.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn key(c_subseed: *const c_char, security: usize) -> *const u8 {
    let subseed_str = unsafe { c_str_to_static_slice(c_subseed) };
    let subseed: Vec<Trit> = subseed_str
        .chars()
        .flat_map(char_to_trits)
        .cloned()
        .collect();

    let mut security_space = vec![0; security * iss::KEY_LENGTH];
    let mut key = vec![0; iss::KEY_LENGTH];
    iss::key::<Trit, CpuCurl<Trit>>(&subseed, &mut security_space, &mut key);

    let out_str = Box::new(trits_to_string(&key).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn digest_key(c_key: *const c_char) -> *const u8 {
    let key_str = unsafe { c_str_to_static_slice(c_key) };
    let key: Vec<Trit> = key_str.chars().flat_map(char_to_trits).cloned().collect();


    let mut digest = vec![0; iss::DIGEST_LENGTH];
    iss::digest_key::<Trit, CpuCurl<Trit>>(&key, &mut digest);

    let out_str = Box::new(trits_to_string(digest.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn address(c_digest: *const c_char) -> *const u8 {
    let digest_str = unsafe { c_str_to_static_slice(c_digest) };
    let digest: Vec<Trit> = digest_str
        .chars()
        .flat_map(char_to_trits)
        .cloned()
        .collect();

    let mut address = vec![0; iss::ADDRESS_LENGTH];
    iss::address::<Trit, CpuCurl<Trit>>(&digest, &mut address);

    let out_str = Box::new(trits_to_string(address.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn signature(c_bundle: *const c_char, c_key: *const c_char) -> *const u8 {
    let key_str = unsafe { c_str_to_static_slice(c_key) };
    let key: Vec<Trit> = key_str.chars().flat_map(char_to_trits).cloned().collect();

    let bundle_str = unsafe { c_str_to_static_slice(c_bundle) };
    let bundle: Vec<Trit> = bundle_str
        .chars()
        .flat_map(char_to_trits)
        .cloned()
        .collect();

    let mut signature = vec![0; key.len()];
    iss::signature::<CpuCurl<Trit>>(&bundle, &key, &mut signature);

    let out_str = Box::new(trits_to_string(signature.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn digest_bundle_signature(c_bundle: *const c_char, c_signature: *const c_char) -> *const u8 {
    let signature_str = unsafe { c_str_to_static_slice(c_signature) };
    let signature: Vec<Trit> = signature_str
        .chars()
        .flat_map(char_to_trits)
        .cloned()
        .collect();

    let bundle_str = unsafe { c_str_to_static_slice(c_bundle) };
    let bundle: Vec<Trit> = bundle_str
        .chars()
        .flat_map(char_to_trits)
        .cloned()
        .collect();

    let mut digest = vec![0; iss::DIGEST_LENGTH];
    iss::digest_bundle_signature::<CpuCurl<Trit>>(&bundle, &signature, &mut digest);

    let out_str = Box::new(trits_to_string(digest.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}
