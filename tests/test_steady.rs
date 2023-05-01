use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn test_steady() {
    let file = File::open("./schematics/steady_state.schem").unwrap();
    let mut world = World::from_file(&file);

    assert_eq!(world.get_probes(), vec![false]);
    world.step_with_trigger();
    assert_eq!(world.get_probes(), vec![true]);
    world.step();
    assert_eq!(world.get_probes(), vec![false]);
}
