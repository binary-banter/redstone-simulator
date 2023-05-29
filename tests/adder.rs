use std::fs::File;
use redstone_simulator::world::World;

#[test]
fn adder() {
    let file = File::open("./schematics/adder.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("0").unwrap());
    assert!(!world.get_probe("1").unwrap());
    assert!(!world.get_probe("2").unwrap());
    assert!(!world.get_probe("3").unwrap());
    assert!(!world.get_probe("4").unwrap());
    assert!(!world.get_probe("5").unwrap());
    assert!(!world.get_probe("6").unwrap());
    assert!(!world.get_probe("7").unwrap());
    world.step();
    assert!(!world.get_probe("0").unwrap());
    assert!(!world.get_probe("1").unwrap());
    assert!(!world.get_probe("2").unwrap());
    assert!(!world.get_probe("3").unwrap());
    assert!(!world.get_probe("4").unwrap());
    assert!(!world.get_probe("5").unwrap());
    assert!(!world.get_probe("6").unwrap());
    assert!(!world.get_probe("7").unwrap());
}
