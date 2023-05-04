use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn test_steady() {
    let file = File::open("./schematics/steady_state.schem").unwrap();
    let mut world = World::from_file(&file);

    assert!(!world.get_probe("probe_1"));
    world.step_with_trigger();
    assert!(world.get_probe("probe_1"));
    world.step();
    assert!(!world.get_probe("probe_1"));
}

#[test]
fn test_connectivity() {
    let file = File::open("./schematics/redstone_connectivity.schem").unwrap();
    let mut world = World::from_file(&file);

    world.step_with_trigger();
    assert!(world.get_probe("straight"));
    assert!(world.get_probe("up"));
    assert!(world.get_probe("down"));
    assert!(!world.get_probe("up_blocked"));
    assert!(!world.get_probe("down_blocked"));
    assert!(!world.get_probe("side_bent"));
    assert!(world.get_probe("side_bent_over"));
    assert!(world.get_probe("side_bent_rep"));
    assert!(world.get_probe("side_bent_under"));
}
