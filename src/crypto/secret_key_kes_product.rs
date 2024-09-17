use std::hash::{Hash};

use strata_node_rs::co::topl::consensus::models::SignatureKesSum;

#[derive(Hash)]
struct KesBinaryTree {
    node_type_prefix: u8,
    leaf_type_prefix: u8,
    empty_type_prefix: u8,
}

#[derive(Hash)]
struct MerkleNode {
    seed: Vec<u8>,
    witness_left: Vec<u8>,
    witness_right: Vec<u8>,
    left: KesBinaryTree,
    right: KesBinaryTree,
}

#[derive(Hash)]
struct SigningLeaf {}

struct SecretKeyKesProduct {
    super_tree: KesBinaryTree,
    sub_tree: KesBinaryTree,
    next_sub_seed: Vec<u8>,
    sub_signature: SignatureKesSum,
    offset: u32,
}
