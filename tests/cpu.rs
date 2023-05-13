use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn cpu_test() {
    let mut world = World::from(File::open("./schematics/cpu_fib.schem").unwrap());

    world.step_with_trigger();
    world.step_with_trigger();
    for _ in 0..50 {
        world.step();
    }
}
