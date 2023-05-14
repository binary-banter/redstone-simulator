use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn rep_in_glass() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("rep_in_glass").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("rep_in_glass").unwrap());
    world.step();
    assert!(!world.get_probe("rep_in_glass").unwrap());
}

#[test]
fn rep_out_glass() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("rep_out_glass").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("rep_out_glass").unwrap());
    world.step();
    assert!(!world.get_probe("rep_out_glass").unwrap());
}

#[test]
fn over_glass() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("over_glass").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("over_glass").unwrap());
}

#[test]
fn glass_tower() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("glass_tower_1").unwrap());
    assert!(!world.get_probe("glass_tower_2").unwrap());
    world.step_with_trigger();
    assert!(world.get_probe("glass_tower_1").unwrap());
    assert!(world.get_probe("glass_tower_2").unwrap());
}

#[test]
fn glass_down() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("glass_down").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("glass_down").unwrap());
}

#[test]
fn glass_through() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("glass_through").unwrap());
    world.step_with_trigger();
    assert!(world.get_probe("glass_through").unwrap());
}

#[test]
fn redstone_split() {
    let file = File::open("./schematics/redstone_split.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("down").unwrap());
    assert!(!world.get_probe("up").unwrap());
    world.step_with_trigger();
    assert!(world.get_probe("down").unwrap());
    assert!(world.get_probe("up").unwrap());
}
