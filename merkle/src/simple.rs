use trytes::*;
use tmath::*;
use curl::*;
use sign::iss;
use core::mem;
use alloc::*;
use alloc::boxed::Box;

pub enum MerkleTree {
    Leaf([Trit; HASH_LENGTH]),
    Nil,
    Node(Box<MerkleTree>, [Trit; HASH_LENGTH], Box<MerkleTree>),
}

pub enum MerkleBranch {
    Nil,
    Sibling([Trit; HASH_LENGTH], Box<MerkleBranch>),
}

const NULL_HASH: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];

pub fn create<C: Curl<Trit>>(
    seed: &[Trit],
    index: isize,
    count: usize,
    security: usize,
    c1: &mut C,
    c2: &mut C,
    c3: &mut C,
) -> MerkleTree {
    match count {
        0 => MerkleTree::Nil,
        1 => leaf(seed, index, security, c1, c2, c3),
        _ => {
            let ct = count.next_power_of_two();
            node(seed, index, count, ct, security, c1, c2, c3)
        }
    }
}

fn node<C: Curl<Trit>>(
    seed: &[Trit],
    index: isize,
    remaining_count: usize,
    remaining_width: usize,
    security: usize,
    c1: &mut C,
    c2: &mut C,
    c3: &mut C,
) -> MerkleTree {
    match remaining_count {
        0 => MerkleTree::Nil,
        _ => {
            match remaining_width {
                0 => MerkleTree::Nil,
                1 => {
                    match remaining_count {
                        0 => MerkleTree::Nil,
                        _ => leaf(seed, index, security, c1, c2, c3),
                    }
                }
                _ => {
                    combine(
                        seed,
                        index,
                        remaining_count,
                        remaining_width,
                        security,
                        c1,
                        c2,
                        c3,
                    )
                }
            }
        }
    }
}

fn combine<C: Curl<Trit>>(
    seed: &[Trit],
    index: isize,
    count: usize,
    remaining_width: usize,
    security: usize,
    c1: &mut C,
    c2: &mut C,
    c3: &mut C,
) -> MerkleTree {
    let right_count = count.next_power_of_two() >> 1;
    let left_count = count - right_count;
    let left = node(
        seed,
        index,
        left_count,
        remaining_width >> 1,
        security,
        c1,
        c2,
        c3,
    );
    let right = node(
        seed,
        index + left_count as isize,
        right_count,
        remaining_width >> 1,
        security,
        c1,
        c2,
        c3,
    );
    match left {
        MerkleTree::Leaf(hash) => c1.absorb(&hash),
        MerkleTree::Node(_, hash, _) => c1.absorb(&hash),
        _ => c1.absorb(&NULL_HASH),
    };
    match right {
        MerkleTree::Leaf(hash) => c1.absorb(&hash),
        MerkleTree::Node(_, hash, _) => c1.absorb(&hash),
        _ => c1.absorb(&NULL_HASH),
    };
    let mut hash: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];
    c1.squeeze(&mut hash);
    c1.reset();
    MerkleTree::Node(Box::new(left), hash, Box::new(right))
}

fn leaf<C: Curl<Trit>>(
    seed: &[Trit],
    index: isize,
    security: usize,
    c1: &mut C,
    c2: &mut C,
    c3: &mut C,
) -> MerkleTree {
    let mut subseed: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];
    let mut hash: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];
    iss::subseed(&seed, index, &mut subseed, c1);
    c1.reset();
    iss::subseed_to_digest(&subseed, security, &mut hash, c1, c2, c3);
    c1.reset();
    c2.reset();
    c3.reset();
    iss::address(&mut hash, c1);
    c1.reset();
    MerkleTree::Leaf(hash)
}

pub fn size(node: &MerkleTree) -> usize {
    match *node {
        MerkleTree::Nil => 0,
        MerkleTree::Leaf(_) => 1,
        MerkleTree::Node(ref left, _, ref right) => 1 + size(left) + size(right),
    }
}

pub fn depth(node: &MerkleTree) -> usize {
    match *node {
        MerkleTree::Nil => 0,
        MerkleTree::Leaf(_) => 1,
        MerkleTree::Node(ref left, _, _) => 1 + depth(left),
    }
}

pub fn count(node: &MerkleTree) -> usize {
    match *node {
        MerkleTree::Nil => 0,
        MerkleTree::Leaf(_) => 1,
        MerkleTree::Node(ref left, _, ref right) => count(left) + count(right),
    }
}

pub fn slice(node: &MerkleTree, out: &mut [Trit]) {
    match *node {
        MerkleTree::Leaf(h) => out.clone_from_slice(&h),
        MerkleTree::Node(_, h, _) => out.clone_from_slice(&h),
        _ => panic!("Error, dude!"),
    }
}

pub fn branch(node: &MerkleTree, index: usize) -> MerkleBranch {
    match *node {
        MerkleTree::Leaf(_) => MerkleBranch::Nil,
        MerkleTree::Node(ref left, _, ref right) => {
            let mut hash: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];
            let left_count = count(left);
            let go_left = index < left_count;
            let (sibling, child) = if go_left {
                (right, left)
            } else {
                (left, right)
            };
            slice(sibling, &mut hash);
            MerkleBranch::Sibling(
                hash,
                Box::new(branch(
                    child,
                    if go_left { index } else { index - left_count },
                )),
            )
        }
        _ => panic!("Error, dude!"),
    }
}

pub fn len(node: &MerkleBranch) -> usize {
    match *node {
        MerkleBranch::Nil => 0,
        MerkleBranch::Sibling(_, ref next) => 1 + len(next),
    }
}

pub fn write_branch(node: &MerkleBranch, index: usize, out: &mut [Trit]) {
    match *node {
        MerkleBranch::Sibling(hash, ref next) => {
            out[index..index + HASH_LENGTH].clone_from_slice(&hash);
            if index != 0 {
                write_branch(next, index.saturating_sub(HASH_LENGTH), out);
            }
        }
        _ => {}
    }
}

pub fn root<C: Curl<Trit>>(address: &[Trit], hashes: &[Trit], index: usize, curl: &mut C) -> usize {
    let mut i = 1;
    let mut num_before_end: usize = 0;
    let mut out = address.to_vec();
    let mut helper = |out: &mut [Trit], hash: &[Trit]| -> usize {
        curl.reset();
        let end = if i & index == 0 {
            curl.absorb(&out);
            curl.absorb(&hash);
            1
        } else {
            curl.absorb(&hash);
            curl.absorb(&out);
            0
        };
        i <<= 1;

        out.clone_from_slice(curl.rate());
        end
    };

    for hash in hashes.chunks(HASH_LENGTH) {
        num_before_end += helper(&mut out, hash);
    }
    num_before_end
}

#[cfg(test)]
mod tests {
    use super::*;
    use sign::iss;
    use curl_cpu::*;

    #[test]
    fn it_does_not_panic() {
        let seed: Vec<Trit> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ9"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let mut c1 = CpuCurl::<Trit>::default();
        let mut c2 = CpuCurl::<Trit>::default();
        let mut c3 = CpuCurl::<Trit>::default();

        let start = 1;
        let index = 5;
        let tree_depth = 5;
        let leaf_count = 9;
        let security = 1;
        let mut digest = vec![0; iss::DIGEST_LENGTH];
        let mut root_node = create(
            &seed,
            start,
            leaf_count,
            security,
            &mut c1,
            &mut c2,
            &mut c3,
        );
        let mut some_branch = branch(&root_node, index);
        assert_eq!(20, size(&root_node));
        assert_eq!(tree_depth, depth(&root_node));
        assert_eq!(leaf_count, count(&root_node));
        let branch_length = len(&some_branch);
        assert_eq!(tree_depth - 1, branch_length);
    }
}
