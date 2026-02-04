use bonsai_trie::{
    databases::HashMapDb,
    id::BasicIdBuilder,
    BitVec, BonsaiStorage, BonsaiStorageConfig,
};
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use starknet_types_core::{felt::Felt, hash::Pedersen};

fn make_key(rng: &mut impl RngCore) -> BitVec {
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    let mut key = BitVec::from_vec(bytes.to_vec());
    key.truncate(251);
    key
}

fn main() {
    let num_keys: u32 = std::env::var("BONSAI_KEYS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1_000);
    let num_updates: u32 = std::env::var("BONSAI_UPDATES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);

    let mut storage: BonsaiStorage<_, _, Pedersen> = BonsaiStorage::new(
        HashMapDb::default(),
        BonsaiStorageConfig::default(),
        251,
    );
    let mut id_builder = BasicIdBuilder::new();
    let identifier: Vec<u8> = vec![];
    let mut rng = SmallRng::seed_from_u64(42);
    let mut keys: Vec<BitVec> = Vec::with_capacity(num_keys as usize);

    for i in 0..num_keys {
        let key = make_key(&mut rng);
        keys.push(key.clone());
        storage
            .insert(&identifier, &key, &Felt::from((i + 1) as u64))
            .unwrap();
    }
    storage.commit(id_builder.new_id()).unwrap();

    for i in 0..num_updates {
        let key = &keys[(i as usize) % keys.len()];
        storage
            .insert(&identifier, &key, &Felt::from((i + 10) as u64))
            .unwrap();
    }
    storage.commit(id_builder.new_id()).unwrap();
}
