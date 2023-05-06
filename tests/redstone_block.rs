use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn side() {
    let file = File::open("./schematics/redstone_block.schem").unwrap();
    let mut world = World::from_file(&file);

    assert!(world.get_probe("side"));
    world.step();
    assert!(world.get_probe("side"));
}

#[test]
fn top() {
    let file = File::open("./schematics/redstone_block.schem").unwrap();
    let mut world = World::from_file(&file);

    assert!(world.get_probe("top"));
    world.step();
    assert!(world.get_probe("top"));
}

#[test]
fn hug() {
    let file = File::open("./schematics/redstone_block.schem").unwrap();
    let mut world = World::from_file(&file);

    assert!(!world.get_probe("hug"));
    world.step();
    assert!(!world.get_probe("hug"));
}

#[test]
fn torch() {
    let file = File::open("./schematics/redstone_block.schem").unwrap();
    let mut world = World::from_file(&file);

    assert!(!world.get_probe("torch"));
    world.step();
    assert!(!world.get_probe("torch"));
}