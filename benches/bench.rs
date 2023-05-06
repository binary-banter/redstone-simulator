use std::fs::File;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use redstone_simulator::world::World;

fn redstone_stress(c: &mut Criterion) {
    let file = File::open("./schematics/stress.schem").unwrap();
    let mut world = World::from_file(&file);

    c.bench_function("redstone_stress", |b| b.iter(|| {
        black_box(&mut world).step_with_trigger();
        black_box(&mut world).step();
    }));
}

criterion_group!(benches, redstone_stress);
criterion_main!(benches);