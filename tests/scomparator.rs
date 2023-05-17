use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn scomparator() {
    let file = File::open("./schematics/scomparator.schem").unwrap();
    let mut world = World::from(file);

}
