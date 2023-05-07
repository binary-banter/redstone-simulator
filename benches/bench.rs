use criterion::{black_box, criterion_group, criterion_main, Criterion};
use redstone_simulator::world::World;
use std::fs::File;

fn redstone_stress(c: &mut Criterion) {
    let file = File::open("./schematics/stress.schem").unwrap();
    let mut world = World::from_file(&file);

    c.bench_function("redstone_stress", |b| {
        b.iter(|| {
            black_box(&mut world).step_with_trigger();
            black_box(&mut world).step();
        })
    });
}

fn cpu(c: &mut Criterion) {
    let file = File::open("./schematics/8bit_cpu_1.1.schem").unwrap();
    let mut world = World::from_file(&file);

    c.bench_function("cpu", |b| {
        b.iter(|| {
            black_box(&mut world).step_with_trigger();
            for _ in 0..40 {
                black_box(&mut world).step();
            }
        })
    });
}

criterion_group!(benches, redstone_stress, cpu);
criterion_main!(benches);
