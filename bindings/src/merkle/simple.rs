use alloc::vec::Vec;
use alloc::boxed::Box;
use iota_trytes::*;
use iota_curl_cpu::*;
use iota_curl::Curl;
use iota_merkle;

use shared::*;

#[no_mangle]
pub fn iota_merkle_create(
    seed: &CTrits,
    index: isize,
    count: usize,
    security: u8,
) -> *mut iota_merkle::MerkleTree {
    let mut c1 = CpuCurl::<Trit>::default();
    let mut c2 = CpuCurl::<Trit>::default();
    let mut c3 = CpuCurl::<Trit>::default();

    let out = if seed.encoding == TritEncoding::TRIT {
        Box::new(iota_merkle::create(
            ctrits_slice_trits(seed),
            index,
            count,
            security as usize,
            &mut c1,
            &mut c2,
            &mut c3,
        ))
    } else {
        let seed_vec = ctrits_to_trits(seed);
        Box::new(iota_merkle::create(
            &seed_vec,
            index,
            count,
            security as usize,
            &mut c1,
            &mut c2,
            &mut c3,
        ))
    };

    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_merkle_drop(merkle: *mut iota_merkle::MerkleTree) {
    unsafe { Box::from_raw(merkle) };
}

#[no_mangle]
pub fn iota_merkle_size(tree: &iota_merkle::MerkleTree) -> usize {
    iota_merkle::size(tree)
}

#[no_mangle]
pub fn iota_merkle_depth(tree: &iota_merkle::MerkleTree) -> usize {
    iota_merkle::depth(tree)
}

#[no_mangle]
pub fn iota_merkle_count(tree: &iota_merkle::MerkleTree) -> usize {
    iota_merkle::count(tree)
}

#[no_mangle]
pub fn iota_merkle_slice(tree: &iota_merkle::MerkleTree) -> *const CTrits {
    let mut out_trits: Vec<Trit> = vec![0; HASH_LENGTH];
    iota_merkle::slice(tree, &mut out_trits);

    let out = Box::new(ctrits_from_trits(out_trits));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_merkle_branch(
    node: &iota_merkle::MerkleTree,
    index: usize,
) -> *const iota_merkle::MerkleBranch {
    let out = Box::new(iota_merkle::branch(node, index));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_merkle_branch_drop(branch: *mut iota_merkle::MerkleBranch) {
    unsafe { Box::from_raw(branch) };
}

#[no_mangle]
pub fn iota_merkle_branch_len(branch: &iota_merkle::MerkleBranch) -> usize {
    iota_merkle::len(branch)
}

#[no_mangle]
pub fn iota_merkle_siblings(branch: &iota_merkle::MerkleBranch) -> *const CTrits {
    let len = iota_merkle::len(branch) * HASH_LENGTH;
    let mut out_trits: Vec<Trit> = vec![0; len];
    iota_merkle::write_branch(&branch, len - HASH_LENGTH, &mut out_trits);

    let out = Box::new(ctrits_from_trits(out_trits));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_merkle_root(addr: &CTrits, siblings: &CTrits, index: usize) -> *const CTrits {
    let mut curl = CpuCurl::<Trit>::default();

    iota_merkle::root(
        ctrits_slice_trits(addr),
        ctrits_slice_trits(siblings),
        index,
        &mut curl,
    );

    let out = Box::new(ctrits_from_trits(curl.rate().to_vec()));
    Box::into_raw(out)
}
