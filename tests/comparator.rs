use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn cmp_repeat() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_repeat"));
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_repeat"));
    world.step();
    assert!(world.get_probe("cmp_repeat"));
    world.step();
    assert!(!world.get_probe("cmp_repeat"));
}

#[test]
fn sub_repeat() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_repeat"));
    world.step_with_trigger();
    assert!(!world.get_probe("sub_repeat"));
    world.step();
    assert!(world.get_probe("sub_repeat"));
    world.step();
    assert!(!world.get_probe("sub_repeat"));
}

#[test]
fn cmp_off() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_off"));
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_off"));
    world.step();
    assert!(!world.get_probe("cmp_off"));
    world.step();
    assert!(!world.get_probe("cmp_off"));
}

#[test]
fn cmp_on() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_on"));
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_on"));
    world.step();
    assert!(world.get_probe("cmp_on"));
    world.step();
    assert!(!world.get_probe("cmp_on"));
}

#[test]
fn cmp_double_off() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_double_off"));
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_double_off"));
    world.step();
    assert!(!world.get_probe("cmp_double_off"));
    world.step();
    assert!(!world.get_probe("cmp_double_off"));
}

#[test]
fn cmp_double_on() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_double_on"));
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_double_on"));
    world.step();
    assert!(world.get_probe("cmp_double_on"));
    world.step();
    assert!(!world.get_probe("cmp_double_on"));
}
#[test]
fn sub_off() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_off"));
    world.step_with_trigger();
    assert!(!world.get_probe("sub_off"));
    world.step();
    assert!(!world.get_probe("sub_off"));
    world.step();
    assert!(!world.get_probe("sub_off"));
}

#[test]
fn sub_on() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_on"));
    world.step_with_trigger();
    assert!(!world.get_probe("sub_on"));
    world.step();

    assert!(world.get_probe("sub_on"));
    world.step();
    assert!(!world.get_probe("sub_on"));
}

#[test]
fn sub_double_off() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_double_off"));
    world.step_with_trigger();
    assert!(!world.get_probe("sub_double_off"));
    world.step();
    assert!(!world.get_probe("sub_double_off"));
    world.step();
    assert!(!world.get_probe("sub_double_off"));
}

#[test]
fn sub_double_on() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_double_on"));
    world.step_with_trigger();
    assert!(!world.get_probe("sub_double_on"));
    world.step();
    assert!(world.get_probe("sub_double_on"));
    world.step();
    assert!(!world.get_probe("sub_double_on"));
}

#[test]
fn cmp_ss1() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_ss_on"));
    assert!(!world.get_probe("cmp_ss_off"));
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_ss_on"));
    assert!(!world.get_probe("cmp_ss_off"));
    world.step();
    assert!(world.get_probe("cmp_ss_on"));
    assert!(!world.get_probe("cmp_ss_off"));
    world.step();
    assert!(!world.get_probe("cmp_ss_on"));
    assert!(!world.get_probe("cmp_ss_off"));
}

#[test]
fn cmp_ss2() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_ss_on_2"));
    assert!(!world.get_probe("cmp_ss_off_2"));
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_ss_on_2"));
    assert!(!world.get_probe("cmp_ss_off_2"));
    world.step();
    assert!(!world.get_probe("cmp_ss_on_2"));
    assert!(!world.get_probe("cmp_ss_off_2"));
    world.step();
    assert!(world.get_probe("cmp_ss_on_2"));
    assert!(!world.get_probe("cmp_ss_off_2"));
    world.step();
    assert!(!world.get_probe("cmp_ss_on_2"));
    assert!(!world.get_probe("cmp_ss_off_2"));
}

#[test]
fn inp_redblock() {
    let file = File::open("./schematics/comparator_inputs.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_redblock"));
    world.step_with_trigger();
    assert!(!world.get_probe("sub_redblock"));
    world.step();
    assert!(!world.get_probe("sub_redblock"));
    world.step();
    assert!(!world.get_probe("sub_redblock"));
}

#[test]
fn inp_solid() {
    let file = File::open("./schematics/comparator_inputs.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_solid"));
    world.step_with_trigger();
    assert!(!world.get_probe("sub_solid"));
    world.step();
    assert!(world.get_probe("sub_solid"));
    world.step();
    assert!(!world.get_probe("sub_solid"));
}

#[test]
fn inp_repeater() {
    let file = File::open("./schematics/comparator_inputs.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_repeater"));
    world.step_with_trigger();
    assert!(!world.get_probe("sub_repeater"));
    world.step();
    assert!(!world.get_probe("sub_repeater"));
    world.step();
    assert!(!world.get_probe("sub_repeater"));
}

#[test]
fn inp_torch() {
    let file = File::open("./schematics/comparator_inputs.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_torch"));
    world.step_with_trigger();
    assert!(!world.get_probe("sub_torch"));
    world.step();
    assert!(world.get_probe("sub_torch"));
    world.step();
    assert!(!world.get_probe("sub_torch"));
}

#[test]
fn cmp_cycle() {
    let file = File::open("./schematics/cmp_cycle.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_cycle_off"));
    assert!(world.get_probe("cmp_cycle_on"));
    world.step();
    assert!(!world.get_probe("cmp_cycle_off"));
    assert!(world.get_probe("cmp_cycle_on"));
}
