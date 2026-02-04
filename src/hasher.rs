use crate::Vec;
use starknet_types_core::{felt::Felt, hash::StarkHash};

/// Hashing trait used by Bonsai that supports optional batched hashing.
pub trait BonsaiHasher: StarkHash + Send + Sync {
    /// Hash a batch of (left, right) pairs.
    fn hash_pairs(pairs: &[(Felt, Felt)]) -> Vec<Felt> {
        pairs.iter().map(|(a, b)| Self::hash(a, b)).collect()
    }

    /// Whether the MerkleTree should prefer batched hashing for commits.
    fn prefers_batched() -> bool {
        false
    }
}

impl BonsaiHasher for starknet_types_core::hash::Pedersen {}
impl BonsaiHasher for starknet_types_core::hash::Poseidon {}

#[cfg(feature = "pedersen-gpu")]
#[derive(Clone, Copy, Debug)]
pub struct PedersenGpu;

#[cfg(feature = "pedersen-gpu")]
impl StarkHash for PedersenGpu {
    fn hash(felt_0: &Felt, felt_1: &Felt) -> Felt {
        pedersen_hash_gpu::pedersen::pedersen_hash(felt_0, felt_1)
    }

    fn hash_array(felts: &[Felt]) -> Felt {
        pedersen_hash_gpu::pedersen::pedersen_hash_array(felts)
    }

    fn hash_single(felt: &Felt) -> Felt {
        pedersen_hash_gpu::pedersen::pedersen_hash(felt, &Felt::ZERO)
    }
}

#[cfg(feature = "pedersen-gpu")]
impl BonsaiHasher for PedersenGpu {
    fn hash_pairs(pairs: &[(Felt, Felt)]) -> Vec<Felt> {
        pedersen_hash_gpu::pedersen::pedersen_hash_batch(pairs)
    }

    fn prefers_batched() -> bool {
        true
    }
}
