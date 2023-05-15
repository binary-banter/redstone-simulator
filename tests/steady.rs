use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn steady() {
    let file = File::open("./schematics/steady_state.schem").unwrap();
    let mut world = World::from(file);

    println!("{:?}", world.blocks);

    assert!(!world.get_probe("probe_1").unwrap());
    world.step_with_trigger();
    assert!(world.get_probe("probe_1").unwrap());
    world.step();
    assert!(!world.get_probe("probe_1").unwrap());
}

#[test]
fn connectivity() {
    let file = File::open("./schematics/redstone_connectivity.schem").unwrap();
    let mut world = World::from(file);

    world.step_with_trigger();

    assert!(world.get_probe("straight").unwrap());
    assert!(world.get_probe("up").unwrap());
    assert!(world.get_probe("down").unwrap());
    assert!(!world.get_probe("up_blocked").unwrap());
    assert!(!world.get_probe("down_blocked").unwrap());
    assert!(!world.get_probe("side_bent").unwrap());
    assert!(world.get_probe("side_bent_over").unwrap());
    assert!(!world.get_probe("side_bent_rep").unwrap());
    assert!(world.get_probe("side_bent_under").unwrap());

    world.step();

    assert!(world.get_probe("side_bent_rep").unwrap());
}

#[test]
fn indirect() {
    let file = File::open("./schematics/indirect.schem").unwrap();
    let mut world = World::from(file);

    world.step_with_trigger();

    assert!(world.get_probe("indirect_down").unwrap());
    assert!(world.get_probe("indirect_up").unwrap());
}

#[test]
fn power_block_with_1_strength() {
    let file = File::open("./schematics/power_block_with_1_strength.schem").unwrap();
    let mut world = World::from(file);

    world.step_with_trigger();

    assert!(world.get_probe("probe_1").unwrap());
    assert!(world.get_probe("probe_2").unwrap());
}
