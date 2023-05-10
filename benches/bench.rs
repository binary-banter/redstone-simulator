use criterion::{black_box, criterion_group, criterion_main, Criterion};
use redstone_simulator::world::World;
use std::fs::File;

fn cpu_dec(c: &mut Criterion) {
    let file = File::open("./schematics/8bit_cpu_1.1.schem").unwrap();
    let mut world = World::from(file);

    c.bench_function("cpu_dec", |b| {
        b.iter(|| {
            black_box(&mut world).step_with_trigger();
            black_box(&mut world).step_with_trigger();
            for _ in 0..40 {
                black_box(&mut world).step();
            }
        })
    });
}

fn cpu_fib(c: &mut Criterion) {
    let file = File::open("./schematics/cpu_fib.schem").unwrap();
    let mut world = World::from(file);

    c.bench_function("cpu_fib", |b| {
        b.iter(|| {
            black_box(&mut world).step_with_trigger();
            black_box(&mut world).step_with_trigger();
            for _ in 0..40 {
                black_box(&mut world).step();
            }
        })
    });
}

criterion_group!(benches, cpu_dec, cpu_fib);
criterion_main!(benches);
