use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn side() {
    let file = File::open("./schematics/redstone_block.schem").unwrap();
    let mut world = World::from(file);

    assert!(world.get_probe("side").unwrap());
    world.step();
    assert!(world.get_probe("side").unwrap());
}

#[test]
fn top() {
    let file = File::open("./schematics/redstone_block.schem").unwrap();
    let mut world = World::from(file);

    assert!(world.get_probe("top").unwrap());
    world.step();
    assert!(world.get_probe("top").unwrap());
}

#[test]
fn hug() {
    let file = File::open("./schematics/redstone_block.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("hug").unwrap());
    world.step();
    assert!(!world.get_probe("hug").unwrap());
}

#[test]
fn torch() {
    let file = File::open("./schematics/redstone_block.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("torch").unwrap());
    world.step();
    assert!(!world.get_probe("torch").unwrap());
}
