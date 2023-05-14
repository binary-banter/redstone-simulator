use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn cmp_repeat() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_repeat").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_repeat").unwrap());
    world.step();
    assert!(world.get_probe("cmp_repeat").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_repeat").unwrap());
}

#[test]
fn sub_repeat() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_repeat").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("sub_repeat").unwrap());
    world.step();
    assert!(world.get_probe("sub_repeat").unwrap());
    world.step();
    assert!(!world.get_probe("sub_repeat").unwrap());
}

#[test]
fn cmp_off() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_off").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_off").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_off").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_off").unwrap());
}

#[test]
fn cmp_on() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_on").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_on").unwrap());
    world.step();
    assert!(world.get_probe("cmp_on").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_on").unwrap());
}

#[test]
fn cmp_double_off() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_double_off").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_double_off").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_double_off").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_double_off").unwrap());
}

#[test]
fn cmp_double_on() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_double_on").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_double_on").unwrap());
    world.step();
    assert!(world.get_probe("cmp_double_on").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_double_on").unwrap());
}
#[test]
fn sub_off() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_off").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("sub_off").unwrap());
    world.step();
    assert!(!world.get_probe("sub_off").unwrap());
    world.step();
    assert!(!world.get_probe("sub_off").unwrap());
}

#[test]
fn sub_on() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_on").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("sub_on").unwrap());
    world.step();

    assert!(world.get_probe("sub_on").unwrap());
    world.step();
    assert!(!world.get_probe("sub_on").unwrap());
}

#[test]
fn sub_double_off() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_double_off").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("sub_double_off").unwrap());
    world.step();
    assert!(!world.get_probe("sub_double_off").unwrap());
    world.step();
    assert!(!world.get_probe("sub_double_off").unwrap());
}

#[test]
fn sub_double_on() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_double_on").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("sub_double_on").unwrap());
    world.step();
    assert!(world.get_probe("sub_double_on").unwrap());
    world.step();
    assert!(!world.get_probe("sub_double_on").unwrap());
}

#[test]
fn cmp_ss1() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_ss_on").unwrap());
    assert!(!world.get_probe("cmp_ss_off").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_ss_on").unwrap());
    assert!(!world.get_probe("cmp_ss_off").unwrap());
    world.step();
    assert!(world.get_probe("cmp_ss_on").unwrap());
    assert!(!world.get_probe("cmp_ss_off").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_ss_on").unwrap());
    assert!(!world.get_probe("cmp_ss_off").unwrap());
}

#[test]
fn cmp_ss2() {
    let file = File::open("./schematics/comparator.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_ss_on_2").unwrap());
    assert!(!world.get_probe("cmp_ss_off_2").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_ss_on_2").unwrap());
    assert!(!world.get_probe("cmp_ss_off_2").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_ss_on_2").unwrap());
    assert!(!world.get_probe("cmp_ss_off_2").unwrap());
    world.step();
    assert!(world.get_probe("cmp_ss_on_2").unwrap());
    assert!(!world.get_probe("cmp_ss_off_2").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_ss_on_2").unwrap());
    assert!(!world.get_probe("cmp_ss_off_2").unwrap());
}

#[test]
fn inp_redblock() {
    let file = File::open("./schematics/comparator_inputs.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_redblock").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("sub_redblock").unwrap());
    world.step();
    assert!(!world.get_probe("sub_redblock").unwrap());
    world.step();
    assert!(!world.get_probe("sub_redblock").unwrap());
}

#[test]
fn inp_solid() {
    let file = File::open("./schematics/comparator_inputs.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_solid").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("sub_solid").unwrap());
    world.step();
    assert!(world.get_probe("sub_solid").unwrap());
    world.step();
    assert!(!world.get_probe("sub_solid").unwrap());
}

#[test]
fn inp_repeater() {
    let file = File::open("./schematics/comparator_inputs.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_repeater").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("sub_repeater").unwrap());
    world.step();
    assert!(!world.get_probe("sub_repeater").unwrap());
    world.step();
    assert!(!world.get_probe("sub_repeater").unwrap());
}

#[test]
fn inp_torch() {
    let file = File::open("./schematics/comparator_inputs.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("sub_torch").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("sub_torch").unwrap());
    world.step();
    assert!(world.get_probe("sub_torch").unwrap());
    world.step();
    assert!(!world.get_probe("sub_torch").unwrap());
}

#[test]
fn cmp_cycle() {
    let file = File::open("./schematics/cmp_cycle.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_cycle_off").unwrap());
    assert!(world.get_probe("cmp_cycle_on").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_cycle_off").unwrap());
    assert!(world.get_probe("cmp_cycle_on").unwrap());
}

#[test]
fn cmp_entity() {
    let file = File::open("./schematics/cmp_entity.schem").unwrap();
    let mut world = World::from(file);

    world.step_with_trigger();
    assert!(world.get_probe("cmp_entity").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_entity").unwrap());
    world.step();
    assert!(world.get_probe("cmp_entity").unwrap());
}

#[test]
fn cmp_prune() {
    let file = File::open("./schematics/cmp_prune.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("cmp_prune").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("cmp_prune").unwrap());
    world.step();
    assert!(!world.get_probe("cmp_prune").unwrap());
}
