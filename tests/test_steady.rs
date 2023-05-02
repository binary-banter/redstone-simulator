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
