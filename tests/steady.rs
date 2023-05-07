use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn steady() {
    let file = File::open("./schematics/steady_state.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("probe_1"));
    world.step_with_trigger();
    assert!(world.get_probe("probe_1"));
    world.step();
    assert!(!world.get_probe("probe_1"));
}

#[test]
fn connectivity() {
    let file = File::open("./schematics/redstone_connectivity.schem").unwrap();
    let mut world = World::from(file);

    world.step_with_trigger();

    assert!(world.get_probe("straight"));
    assert!(world.get_probe("up"));
    assert!(world.get_probe("down"));
    assert!(!world.get_probe("up_blocked"));
    assert!(!world.get_probe("down_blocked"));
    assert!(!world.get_probe("side_bent"));
    assert!(world.get_probe("side_bent_over"));
    assert!(!world.get_probe("side_bent_rep"));
    assert!(world.get_probe("side_bent_under"));

    world.step();

    assert!(world.get_probe("side_bent_rep"));
}

#[test]
fn indirect() {
    let file = File::open("./schematics/indirect.schem").unwrap();
    let mut world = World::from(file);

    world.step_with_trigger();

    assert!(world.get_probe("indirect_down"));
    assert!(world.get_probe("indirect_up"));
}

#[test]
fn power_block_with_1_strength() {
    let file = File::open("./schematics/power_block_with_1_strength.schem").unwrap();
    let mut world = World::from(file);

    world.step_with_trigger();

    assert!(world.get_probe("probe_1"));
    assert!(world.get_probe("probe_2"));
}
