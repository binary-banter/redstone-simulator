use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn rep_in_glass() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("rep_in_glass"));
    world.step_with_trigger();
    assert!(!world.get_probe("rep_in_glass"));
    world.step();
    assert!(!world.get_probe("rep_in_glass"));
}

#[test]
fn rep_out_glass() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("rep_out_glass"));
    world.step_with_trigger();
    assert!(!world.get_probe("rep_out_glass"));
    world.step();
    assert!(!world.get_probe("rep_out_glass"));
}

#[test]
fn over_glass() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("over_glass"));
    world.step_with_trigger();
    assert!(!world.get_probe("over_glass"));
}

#[test]
fn glass_tower() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("glass_tower_1"));
    assert!(!world.get_probe("glass_tower_2"));
    world.step_with_trigger();
    assert!(world.get_probe("glass_tower_1"));
    assert!(world.get_probe("glass_tower_2"));
}

#[test]
fn glass_down() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("glass_down"));
    world.step_with_trigger();
    assert!(!world.get_probe("glass_down"));
}

#[test]
fn glass_through() {
    let file = File::open("./schematics/glass.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("glass_through"));
    world.step_with_trigger();
    assert!(world.get_probe("glass_through"));
}

#[test]
fn redstone_split() {
    let file = File::open("./schematics/redstone_split.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("down"));
    assert!(!world.get_probe("up"));
    world.step_with_trigger();
    assert!(world.get_probe("down"));
    assert!(world.get_probe("up"));
}

