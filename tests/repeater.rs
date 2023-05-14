use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn repeater_1t() {
    let file = File::open("./schematics/repeater_1t.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("probe_1").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 0
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 1
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 2
}

#[test]
fn repeater_2t() {
    let file = File::open("./schematics/repeater_2t.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("probe_1").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 0
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 1
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 2
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 3
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 4
}

#[test]
fn repeater_3t() {
    let file = File::open("./schematics/repeater_3t.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("probe_1").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 0
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 1
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 2
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 3
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 4
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 5
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 6
}

#[test]
fn repeater_4t() {
    let file = File::open("./schematics/repeater_4t.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("probe_1").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 0
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 1
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 2
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 3
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 4
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 5
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 6
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 7
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 8
}

#[test]
fn extender() {
    let file = File::open("./schematics/repeater_extender.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("probe_1").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 0
    world.step();
    assert!(world.get_probe("probe_1").unwrap()); // tick 1
    world.step();
    assert!(!world.get_probe("probe_1").unwrap()); // tick 2
}
