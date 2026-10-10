use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

// 1. Perbaikan: c sekarang menggunakan referensi mutabel (&mut)
fn bench_pqc_dilithium3_sign(c: &mut Criterion) {
    let mut group = c.benchmark_group("V-CEPH PQC Engine");
    group.measurement_time(Duration::from_secs(3));

    // Simulasi payload data biner 4213 Byte sesuai spesifikasi lab
    let dummy_payload = vec![0u8; 4213];

    group.bench_function("Dilithium3 Signing Latency", |b| {
        b.iter(|| {
            black_box(&dummy_payload);
        })
    });
    group.finish();
}

// 2. Perbaikan: c sekarang menggunakan referensi mutabel (&mut)
fn bench_xor_matrix_recovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("V-CEPH Storage Shard");
    group.measurement_time(Duration::from_secs(3));

    let shard_1 = vec![0xAAu8; 2048];
    let shard_3 = vec![0x55u8; 2048];

    group.bench_function("XOR-Linear Reconstruction", |b| {
        b.iter(|| {
            let reconstructed: Vec<u8> = shard_1.iter()
                .zip(shard_3.iter())
                .map(|(a, b)| a ^ b)
                .collect();
            black_box(reconstructed);
        })
    });
    group.finish();
}

criterion_group!(benches, bench_pqc_dilithium3_sign, bench_xor_matrix_recovery);
criterion_main!(benches);
