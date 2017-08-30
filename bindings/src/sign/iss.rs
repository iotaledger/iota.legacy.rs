use alloc::boxed::Box;

use shared::*;

use iota_curl::*;
use iota_trytes::*;
use iota_sign::iss;
use iota_curl_cpu::*;

#[no_mangle]
pub static IOTA_SIGN_ISS_KEY_LENGTH: usize = iss::KEY_LENGTH;
#[no_mangle]
pub static IOTA_SIGN_ISS_DIGEST_LENGTH: usize = iss::DIGEST_LENGTH;
#[no_mangle]
pub static IOTA_SIGN_ISS_ADDRESS_LENGTH: usize = iss::ADDRESS_LENGTH;
#[no_mangle]
pub static IOTA_SIGN_ISS_SIGNATURE_LENGTH: usize = iss::SIGNATURE_LENGTH;

#[no_mangle]
pub  fn iota_sign_iss_subseed(seed: &CTrits, index: isize, curl: &mut CpuCurl<Trit>) -> *const CTrits {
    let mut subseed = vec![0; HASH_LENGTH];

    if seed.encoding == TritEncoding::TRIT {
        iss::subseed(ctrits_slice_trits(seed), index, &mut subseed, curl);
    } else {
        iss::subseed(&ctrits_to_trits(seed), index, &mut subseed, curl);
    }

    curl.reset();

    let ctrits = Box::new(ctrits_from_trits(subseed));
    Box::into_raw(ctrits)
}

#[no_mangle]
pub  fn iota_sign_iss_key(subseed: &CTrits, security: usize, curl: &mut CpuCurl<Trit>) -> *const CTrits {
    let mut key = vec![0; security * iss::KEY_LENGTH];
    assert_eq!(subseed.length, HASH_LENGTH);

    if subseed.encoding == TritEncoding::TRIT {
        key[..HASH_LENGTH].clone_from_slice(ctrits_slice_trits(subseed));
    } else {
        key[..HASH_LENGTH].clone_from_slice(&ctrits_to_trits(subseed));
    }

    iss::key(&mut key, security, curl);
    curl.reset();

    let ctrits = Box::new(ctrits_from_trits(key));
    Box::into_raw(ctrits)
}

#[no_mangle]
pub  fn iota_sign_iss_digest_key(key: &CTrits, curl: &mut CpuCurl<Trit>, curl2: &mut CpuCurl<Trit>) -> *const CTrits {
    let mut digest = vec![0; iss::DIGEST_LENGTH];

    if key.encoding == TritEncoding::TRIT {
        iss::digest_key::<Trit, CpuCurl<Trit>>(ctrits_slice_trits(key), &mut digest, curl, curl2);
    } else {
        iss::digest_key::<Trit, CpuCurl<Trit>>(&ctrits_to_trits(key), &mut digest, curl, curl2);
    }

    curl.reset();
    curl2.reset();

    let ctrits = Box::new(ctrits_from_trits(digest));
    Box::into_raw(ctrits)
}

#[no_mangle]
pub  fn iota_sign_iss_address(digest: &CTrits, curl: &mut CpuCurl<Trit>) -> *const CTrits {
    let mut digest_vec = ctrits_to_trits(digest);
    iss::address::<Trit, CpuCurl<Trit>>(&mut digest_vec, curl);
    curl.reset();

    let address = digest_vec.split_off(HASH_LENGTH);

    let ctrits = Box::new(ctrits_from_trits(address));
    Box::into_raw(ctrits)
}

#[no_mangle]
pub  fn iota_sign_iss_signature(bundle: &CTrits, key: &CTrits, curl: &mut CpuCurl<Trit>) -> *const CTrits {
    let mut signature = ctrits_to_trits(key);

    if bundle.encoding == TritEncoding::TRIT {
        iss::signature::<CpuCurl<Trit>>(ctrits_slice_trits(bundle), &mut signature, curl);
    } else {
        iss::signature::<CpuCurl<Trit>>(&ctrits_to_trits(bundle), &mut signature, curl);
    }

    curl.reset();

    let ctrits = Box::new(ctrits_from_trits(signature));
    Box::into_raw(ctrits)
}

#[no_mangle]
pub  fn iota_sign_iss_subseed_to_signature(
    hash: &CTrits,
    subkey: &CTrits,
    security: usize,
    curl1: &mut CpuCurl<Trit>,
    curl2: &mut CpuCurl<Trit>,
) -> *const CTrits {
    let mut signature = vec![0 as Trit; security * iss::KEY_LENGTH];
    use shared::TritEncoding::*;

    match (&hash.encoding, &subkey.encoding) {
        (&TRIT, &TRIT) => {
            iss::subseed_to_signature(
                ctrits_slice_trits(hash),
                ctrits_slice_trits(subkey),
                &mut signature,
                security,
                curl1,
                curl2,
            );
        }
        (&TRIT, _) => {
            iss::subseed_to_signature(
                ctrits_slice_trits(hash),
                &ctrits_to_trits(subkey),
                &mut signature,
                security,
                curl1,
                curl2,
            );
        }
        (_, &TRIT) => {
            iss::subseed_to_signature(
                &ctrits_to_trits(hash),
                ctrits_slice_trits(subkey),
                &mut signature,
                security,
                curl1,
                curl2,
            );
        }
        (_, _) => {
            iss::subseed_to_signature(
                &ctrits_to_trits(hash),
                &ctrits_to_trits(subkey),
                &mut signature,
                security,
                curl1,
                curl2,
            );
        }
    };

    curl1.reset();
    curl2.reset();

    let ctrits = Box::new(ctrits_from_trits(signature));
    Box::into_raw(ctrits)
}

#[no_mangle]
pub  fn iota_sign_iss_digest_bundle_signature(
    bundle: &CTrits,
    signature: &CTrits,
    curl: &mut CpuCurl<Trit>,
) -> *const CTrits {
    let mut signature = ctrits_to_trits(signature);

    if bundle.encoding == TritEncoding::TRIT {
        iss::digest_bundle_signature::<CpuCurl<Trit>>(ctrits_slice_trits(bundle), &mut signature, curl);
    } else {
        iss::digest_bundle_signature::<CpuCurl<Trit>>(&ctrits_to_trits(bundle), &mut signature, curl);
    }

    curl.reset();

    let digest = signature.split_off(HASH_LENGTH);
    let ctrits = Box::new(ctrits_from_trits(digest));
    Box::into_raw(ctrits)
}
