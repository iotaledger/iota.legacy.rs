use cty::*;
use alloc::boxed::Box;
use alloc::Vec;

use iota_trytes::*;
use iota_sign::iss;
use iota_curl_cpu::*;

use util::c_str_to_static_slice;

#[no_mangle]
pub fn subseed(c_seed: *const c_char, index: isize) -> *const u8 {
    let seed_str = unsafe { c_str_to_static_slice(c_seed) };
    let seed: Vec<Trit> = seed_str.chars().flat_map(char_to_trits).cloned().collect();


    let mut subseed = vec![0; HASH_LENGTH];
    let mut curl = CpuCurl::<Trit>::default();
    iss::subseed(&seed, index, &mut subseed, &mut curl);

    let out_str = Box::new(trits_to_string(subseed.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn key(c_subseed: *const c_char, security: u8) -> *const u8 {
    let subseed_str = unsafe { c_str_to_static_slice(c_subseed) };
    let subseed: Vec<Trit> = subseed_str
        .chars()
        .flat_map(char_to_trits)
        .cloned()
        .collect();

    let mut key = vec![0; iss::KEY_LENGTH];
    key[..HASH_LENGTH].clone_from_slice(&subseed);
    let mut curl = CpuCurl::<Trit>::default();
    iss::key(&mut key, security, &mut curl);

    let out_str = Box::new(trits_to_string(&key).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn digest_key(c_key: *const c_char) -> *const u8 {
    let key_str = unsafe { c_str_to_static_slice(c_key) };
    let mut key: Vec<Trit> = key_str.chars().flat_map(char_to_trits).cloned().collect();


    let mut digest = vec![0; iss::DIGEST_LENGTH];
    let mut curl = CpuCurl::<Trit>::default();
    let mut curl2 = CpuCurl::<Trit>::default();
    iss::digest_key::<Trit, CpuCurl<Trit>>(&key, &mut digest, &mut curl, &mut curl2);

    let out_str = Box::new(trits_to_string(&key[..HASH_LENGTH]).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn address(c_digest: *const c_char) -> *const u8 {
    let digest_str = unsafe { c_str_to_static_slice(c_digest) };
    let mut digest: Vec<Trit> = digest_str
        .chars()
        .flat_map(char_to_trits)
        .cloned()
        .collect();

    let mut curl = CpuCurl::<Trit>::default();
    iss::address::<Trit, CpuCurl<Trit>>(&mut digest, &mut curl);

    let out_str = Box::new(trits_to_string(&digest[..HASH_LENGTH]).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn signature(c_bundle: *const c_char, c_key: *const c_char) -> *const u8 {
    let key_str = unsafe { c_str_to_static_slice(c_key) };
    let mut key: Vec<Trit> = key_str.chars().flat_map(char_to_trits).cloned().collect();

    let bundle_str = unsafe { c_str_to_static_slice(c_bundle) };
    let bundle: Vec<Trit> = bundle_str
        .chars()
        .flat_map(char_to_trits)
        .cloned()
        .collect();

    let mut signature = vec![0; key.len()];
    let mut curl = CpuCurl::<Trit>::default();
    iss::signature::<CpuCurl<Trit>>(&bundle, &mut key, &mut curl);

    let out_str = Box::new(trits_to_string(key.as_slice()).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}

#[no_mangle]
pub fn digest_bundle_signature(c_bundle: *const c_char, c_signature: *const c_char) -> *const u8 {
    let signature_str = unsafe { c_str_to_static_slice(c_signature) };
    let mut signature: Vec<Trit> = signature_str
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

    let mut curl = CpuCurl::<Trit>::default();
    let mut curl2 = CpuCurl::<Trit>::default();
    iss::digest_bundle_signature::<CpuCurl<Trit>>(&bundle, &mut signature, &mut curl);

    let out_str = Box::new(trits_to_string(&curl.state[..HASH_LENGTH]).unwrap() + "\0");
    &out_str.as_bytes()[0] as *const u8
}
