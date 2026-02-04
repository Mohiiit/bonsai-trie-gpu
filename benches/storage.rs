use std::hint::black_box;

use bonsai_trie::{
    databases::HashMapDb,
    id::{BasicId, BasicIdBuilder},
    BitVec, BonsaiHasher, BonsaiStorage, BonsaiStorageConfig,
};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{prelude::*, thread_rng};
use starknet_types_core::{
    felt::Felt,
    hash::{Poseidon, StarkHash},
};

mod flamegraph;

#[cfg(feature = "pedersen-gpu")]
use bonsai_trie::PedersenGpu;
#[cfg(not(feature = "pedersen-gpu"))]
use starknet_types_core::hash::Pedersen;

#[cfg(feature = "pedersen-gpu")]
type PedersenBench = PedersenGpu;
#[cfg(not(feature = "pedersen-gpu"))]
type PedersenBench = Pedersen;

fn drop_storage(c: &mut Criterion) {
    c.bench_function("drop storage", move |b| {
        b.iter_batched(
            || {
                let mut bonsai_storage: BonsaiStorage<BasicId, _, PedersenBench> =
                    BonsaiStorage::new(
                    HashMapDb::<BasicId>::default(),
                    BonsaiStorageConfig::default(),
                    251,
                );

                let mut rng = SmallRng::seed_from_u64(42);
                let felt = Felt::from_hex("0x66342762FDD54D033c195fec3ce2568b62052e").unwrap();
                for _ in 0..4000 {
                    let bitvec = BitVec::from_vec(vec![
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                    ]);
                    bonsai_storage.insert(&[], &bitvec, &felt).unwrap();
                }

                let mut id_builder = BasicIdBuilder::new();
                let id1 = id_builder.new_id();
                bonsai_storage.commit(id1).unwrap();

                bonsai_storage
            },
            std::mem::drop,
            BatchSize::LargeInput,
        );
    });
}

fn storage_with_insert(c: &mut Criterion) {
    c.bench_function("storage commit with insert", move |b| {
        let mut rng = thread_rng();
        b.iter_batched_ref(
            || {
                let bonsai_storage: BonsaiStorage<BasicId, _, PedersenBench> =
                    BonsaiStorage::new(
                    HashMapDb::<BasicId>::default(),
                    BonsaiStorageConfig::default(),
                    251,
                );
                bonsai_storage
            },
            |bonsai_storage| {
                let felt = Felt::from_hex("0x66342762FDD54D033c195fec3ce2568b62052e").unwrap();
                for _ in 0..40000 {
                    let bitvec = BitVec::from_vec(vec![
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                    ]);
                    bonsai_storage.insert(&[], &bitvec, &felt).unwrap();
                }
                // let mut id_builder = BasicIdBuilder::new();
                // bonsai_storage.commit(id_builder.new_id()).unwrap();
            },
            BatchSize::LargeInput,
        );
    });
}

fn storage(c: &mut Criterion) {
    c.bench_function("storage commit", move |b| {
        let mut bonsai_storage: BonsaiStorage<BasicId, _, PedersenBench> = BonsaiStorage::new(
            HashMapDb::<BasicId>::default(),
            BonsaiStorageConfig::default(),
            251,
        );
        let mut rng = SmallRng::seed_from_u64(42);

        let felt = Felt::from_hex("0x66342762FDD54D033c195fec3ce2568b62052e").unwrap();
        for _ in 0..1000 {
            let bitvec = BitVec::from_vec(vec![
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
            ]);
            bonsai_storage.insert(&[], &bitvec, &felt).unwrap();
        }

        let mut id_builder = BasicIdBuilder::new();
        b.iter_batched_ref(
            || bonsai_storage.clone(),
            |bonsai_storage| {
                bonsai_storage.commit(id_builder.new_id()).unwrap();
            },
            criterion::BatchSize::LargeInput,
        );
    });
}

fn one_update(c: &mut Criterion) {
    c.bench_function("one update", move |b| {
        let mut bonsai_storage: BonsaiStorage<BasicId, _, PedersenBench> = BonsaiStorage::new(
            HashMapDb::<BasicId>::default(),
            BonsaiStorageConfig::default(),
            251,
        );
        let mut rng = SmallRng::seed_from_u64(42);

        let felt = Felt::from_hex("0x66342762FDD54D033c195fec3ce2568b62052e").unwrap();
        for _ in 0..1000 {
            let bitvec = BitVec::from_vec(vec![
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
            ]);
            bonsai_storage.insert(&[], &bitvec, &felt).unwrap();
        }

        let mut id_builder = BasicIdBuilder::new();
        bonsai_storage.commit(id_builder.new_id()).unwrap();

        b.iter_batched_ref(
            || bonsai_storage.clone(),
            |bonsai_storage| {
                let bitvec = BitVec::from_vec(vec![0, 1, 2, 3, 4, 5]);
                bonsai_storage.insert(&[], &bitvec, &felt).unwrap();
                bonsai_storage.commit(id_builder.new_id()).unwrap();
            },
            criterion::BatchSize::LargeInput,
        );
    });
}

fn five_updates(c: &mut Criterion) {
    c.bench_function("five updates", move |b| {
        let mut bonsai_storage: BonsaiStorage<BasicId, _, PedersenBench> = BonsaiStorage::new(
            HashMapDb::<BasicId>::default(),
            BonsaiStorageConfig::default(),
            251,
        );
        let mut rng = SmallRng::seed_from_u64(42);

        let felt = Felt::from_hex("0x66342762FDD54D033c195fec3ce2568b62052e").unwrap();
        for _ in 0..1000 {
            let bitvec = BitVec::from_vec(vec![
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
            ]);
            bonsai_storage.insert(&[], &bitvec, &felt).unwrap();
        }

        let mut id_builder = BasicIdBuilder::new();
        bonsai_storage.commit(id_builder.new_id()).unwrap();

        b.iter_batched_ref(
            || bonsai_storage.clone(),
            |bonsai_storage| {
                bonsai_storage
                    .insert(&[], &BitVec::from_vec(vec![0, 1, 2, 3, 4, 5]), &felt)
                    .unwrap();
                bonsai_storage
                    .insert(&[], &BitVec::from_vec(vec![0, 2, 2, 5, 4, 5]), &felt)
                    .unwrap();
                bonsai_storage
                    .insert(&[], &BitVec::from_vec(vec![0, 1, 2, 3, 3, 5]), &felt)
                    .unwrap();
                bonsai_storage
                    .insert(&[], &BitVec::from_vec(vec![0, 1, 1, 3, 99, 3]), &felt)
                    .unwrap();
                bonsai_storage
                    .insert(&[], &BitVec::from_vec(vec![0, 1, 2, 3, 4, 6]), &felt)
                    .unwrap();
                bonsai_storage.commit(id_builder.new_id()).unwrap();
            },
            criterion::BatchSize::LargeInput,
        );
    });
}

fn multiple_contracts(c: &mut Criterion) {
    c.bench_function("multiple contracts", move |b| {
        let mut bonsai_storage: BonsaiStorage<BasicId, _, PedersenBench> = BonsaiStorage::new(
            HashMapDb::<BasicId>::default(),
            BonsaiStorageConfig::default(),
            251,
        );
        let mut rng = thread_rng();

        let felt = Felt::from_hex("0x66342762FDD54D033c195fec3ce2568b62052e").unwrap();
        for _ in 0..1000 {
            let bitvec = BitVec::from_vec(vec![rng.gen(), rng.gen(), rng.gen(), rng.gen()]);
            bonsai_storage
                .insert(
                    &[
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                    ],
                    &bitvec,
                    &felt,
                )
                .unwrap();
        }

        let mut id_builder = BasicIdBuilder::new();

        b.iter_batched_ref(
            || bonsai_storage.clone(),
            |bonsai_storage| {
                bonsai_storage.commit(id_builder.new_id()).unwrap();
            },
            criterion::BatchSize::LargeInput,
        );
    });
}

fn pedersen_hash(c: &mut Criterion) {
    c.bench_function("pedersen hash", move |b| {
        let felt0 =
            Felt::from_hex("0x100bd6fbfced88ded1b34bd1a55b747ce3a9fde9a914bca75571e4496b56443")
                .unwrap();
        let felt1 =
            Felt::from_hex("0x00a038cda302fedbc4f6117648c6d3faca3cda924cb9c517b46232c6316b152f")
                .unwrap();
        b.iter(|| {
            black_box(PedersenBench::hash(&felt0, &felt1));
        })
    });
}

fn pedersen_hash_batch(c: &mut Criterion) {
    let pairs: Vec<(Felt, Felt)> = (0..4096)
        .map(|i| (Felt::from(i as u64), Felt::from((i + 1) as u64)))
        .collect();
    let mut group = c.benchmark_group("pedersen hash batch");
    group.sample_size(10);
    group.bench_function("pairs_4096", move |b| {
        b.iter(|| {
            black_box(PedersenBench::hash_pairs(black_box(&pairs)));
        })
    });
    group.finish();
}

fn poseidon_hash(c: &mut Criterion) {
    c.bench_function("poseidon hash", move |b| {
        let felt0 =
            Felt::from_hex("0x100bd6fbfced88ded1b34bd1a55b747ce3a9fde9a914bca75571e4496b56443")
                .unwrap();
        let felt1 =
            Felt::from_hex("0x00a038cda302fedbc4f6117648c6d3faca3cda924cb9c517b46232c6316b152f")
                .unwrap();
        b.iter(|| {
            black_box(Poseidon::hash(&felt0, &felt1));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default(); // .with_profiler(flamegraph::FlamegraphProfiler::new(100));
    targets = storage, one_update, five_updates, pedersen_hash, pedersen_hash_batch, poseidon_hash, drop_storage, storage_with_insert, multiple_contracts
}
criterion_main!(benches);
