use criterion::{black_box, criterion_group, criterion_main, Criterion};
use redstone_simulator::world::World;
use std::fs::File;

fn stress_srepeater(c: &mut Criterion) {
    let file = File::open("./schematics/srepeater_stress.schem").unwrap();
    let mut world = World::from(file);
    world.step_with_trigger();
    c.bench_function("stress_srepeater", |b| {
        b.iter(|| {
            black_box(&mut world).step();
        })
    });
}

fn stress_repeater(c: &mut Criterion) {
    let file = File::open("./schematics/repeater_stress.schem").unwrap();
    let mut world = World::from(file);
    world.step_with_trigger();
    c.bench_function("stress_repeater", |b| {
        b.iter(|| {
            black_box(&mut world).step();
        })
    });
}

fn stress_comparator(c: &mut Criterion) {
    let file = File::open("./schematics/comparator_stress.schem").unwrap();
    let mut world = World::from(file);
    world.step_with_trigger();
    c.bench_function("stress_comparator", |b| {
        b.iter(|| {
            black_box(&mut world).step();
        })
    });
}

criterion_group!(stress, stress_srepeater, stress_repeater, stress_comparator);
criterion_main!(stress);
