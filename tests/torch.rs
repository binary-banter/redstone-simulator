use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn torch() {
    let file = File::open("./schematics/torch.schem").unwrap();
    let mut world = World::from(file);

    assert!(world.get_probe("probe_1").unwrap());
    world.step_with_trigger();
    assert!(world.get_probe("probe_1").unwrap());
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 1
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 2
}

#[test]
fn wall_torch() {
    let file = File::open("./schematics/wall_torch.schem").unwrap();
    let mut world = World::from(file);

    assert!(world.get_probe("probe_1").unwrap());
    world.step_with_trigger();
    assert!(world.get_probe("probe_1").unwrap());
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 1
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 2
}

#[test]
fn torch_strong() {
    let file = File::open("./schematics/torch_strong.schem").unwrap();
    let mut world = World::from(file);

    assert!(world.get_probe("probe_1").unwrap());
    world.step_with_trigger();
    assert!(world.get_probe("probe_1").unwrap());
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 1
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 2
}
