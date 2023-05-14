use std::fs::File;
use redstone_simulator::world::{RedGraph, World};

#[test]
fn telescope_simple() {
    let file = File::open("./schematics/telescope_simple.schem").unwrap();
    let mut world = World::from(file);

    println!("{:?}", world.blocks);


    assert!(!world.get_probe("tel_rrrrr"));
    world.step_with_trigger();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));

}

#[test]
fn telescope_simple_pulse() {
    let file = File::open("./schematics/telescope_simple.schem").unwrap();
    let mut world = World::from(file);

    println!("{:?}", world.blocks);


    assert!(!world.get_probe("tel_rrrrr"));
    world.step_with_trigger();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step_with_trigger();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step_with_trigger();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(world.get_probe("tel_rrrrr"));
    world.step_with_trigger();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));
    world.step();
    assert!(world.get_probe("tel_rrrrr"));
    world.step();
    assert!(!world.get_probe("tel_rrrrr"));

}