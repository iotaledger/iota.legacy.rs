use cty::*;
use core::mem;
use alloc::*;
use alloc::string::String;
use iota_trytes::*;
use iota_curl_cpu::*;
use iota_sign::iss;
use shared::util::c_str_to_static_slice;
use iota_merkle;

#[no_mangle]
pub fn merkle_create(
    c_seed: *const c_char,
    index: isize,
    count: usize,
    security: u8,
) -> iota_merkle::MerkleTree {
    let seed: Vec<Trit> = {
        let seed_str = unsafe { c_str_to_static_slice(c_seed) };
        seed_str.chars().flat_map(char_to_trits).cloned().collect()
    };

    let mut c1 = CpuCurl::<Trit>::default();
    let mut c2 = CpuCurl::<Trit>::default();
    let mut c3 = CpuCurl::<Trit>::default();
    let mut key_space = [0 as Trit; iss::KEY_LENGTH];
    iota_merkle::create(
        &seed,
        index,
        count,
        security as usize,
        &mut c1,
        &mut c2,
        &mut c3,
    )
}

#[no_mangle]
pub fn merkle_size(tree: &iota_merkle::MerkleTree) -> usize {
    iota_merkle::size(tree)
}

#[no_mangle]
pub fn merkle_depth(tree: &iota_merkle::MerkleTree) -> usize {
    iota_merkle::depth(tree)
}

#[no_mangle]
pub fn merkle_count(tree: &iota_merkle::MerkleTree) -> usize {
    iota_merkle::count(tree)
}

#[no_mangle]
pub fn merkle_slice(tree: &iota_merkle::MerkleTree) -> *const u8 {
    let mut out_trits: Vec<Trit> = vec![0; HASH_LENGTH];
    iota_merkle::slice(tree, &mut out_trits);
    let slice_str = trits_to_string(&out_trits).unwrap();
    let ptr = slice_str.as_ptr();
    mem::forget(slice_str);

    ptr
}

#[no_mangle]
pub fn merkle_branch(node: &iota_merkle::MerkleTree, index: usize) -> iota_merkle::MerkleBranch {
    iota_merkle::branch(node, index)
}

#[no_mangle]
pub fn merkle_branch_len(branch: &iota_merkle::MerkleBranch) -> usize {
    iota_merkle::len(branch)
}

#[no_mangle]
pub fn merkle_siblings(branch: &iota_merkle::MerkleBranch) -> *const u8 {
    let len = iota_merkle::len(branch) * HASH_LENGTH;
    let mut out_trits: Vec<Trit> = vec![0; len];
    iota_merkle::write_branch(&branch, len - HASH_LENGTH, &mut out_trits);

    let slice_str = trits_to_string(&out_trits).unwrap();
    let ptr = slice_str.as_ptr();
    mem::forget(slice_str);

    ptr
}

#[no_mangle]
pub fn merkle_root(c_addr: *const c_char, c_siblings: *const c_char, index: usize) -> *const u8 {
    let addr_str = unsafe { c_str_to_static_slice(c_addr) };
    let addr: Vec<Trit> = addr_str.chars().flat_map(char_to_trits).cloned().collect();

    let siblings_str = unsafe { c_str_to_static_slice(c_siblings) };
    let siblings: Vec<Trit> = siblings_str
        .split("\n")
        .flat_map(|a| {
            a.chars()
                .flat_map(char_to_trits)
                .cloned()
                .collect::<Vec<Trit>>()
        })
        .collect();

    let mut curl = CpuCurl::<Trit>::default();
    let num_before_end = iota_merkle::root(&addr, &siblings, index, &mut curl);

    let out_str = trits_to_string(&curl.state[..HASH_LENGTH]).unwrap() + "\0";
    let ptr = out_str.as_ptr();
    mem::forget(out_str);

    ptr
}
