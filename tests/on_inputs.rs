use std::fs::File;
use redstone_simulator::world::World;

#[test]
fn comparator_strengths() {
    let file = File::open("./schematics/comparator_strengths.schem").unwrap();
    let mut world = World::from(file);

    world.step_with_trigger();
    world.step();
    world.step();
    assert!(world.get_probe("0").unwrap());
    assert!(world.get_probe("1").unwrap());
    assert!(world.get_probe("2").unwrap());
    assert!(world.get_probe("3").unwrap());
    assert!(world.get_probe("4").unwrap());
    assert!(world.get_probe("5").unwrap());
    assert!(world.get_probe("6").unwrap());
    assert!(world.get_probe("7").unwrap());
    assert!(world.get_probe("8").unwrap());
    assert!(world.get_probe("9").unwrap());
    assert!(world.get_probe("10").unwrap());
    assert!(world.get_probe("11").unwrap());
    assert!(world.get_probe("12").unwrap());
    assert!(world.get_probe("13").unwrap());
    assert!(world.get_probe("14").unwrap());
    assert!(!world.get_probe("15").unwrap());
    world.step();
    assert!(!world.get_probe("0").unwrap());
    assert!(!world.get_probe("1").unwrap());
    assert!(!world.get_probe("2").unwrap());
    assert!(!world.get_probe("3").unwrap());
    assert!(!world.get_probe("4").unwrap());
    assert!(!world.get_probe("5").unwrap());
    assert!(!world.get_probe("6").unwrap());
    assert!(!world.get_probe("7").unwrap());
    assert!(!world.get_probe("8").unwrap());
    assert!(!world.get_probe("9").unwrap());
    assert!(!world.get_probe("10").unwrap());
    assert!(!world.get_probe("11").unwrap());
    assert!(!world.get_probe("12").unwrap());
    assert!(!world.get_probe("13").unwrap());
    assert!(!world.get_probe("14").unwrap());
    assert!(!world.get_probe("15").unwrap());
}

#[test]
fn comparator_strengths2() {
    let file = File::open("./schematics/comparator_strengths2.schem").unwrap();
    let mut world = World::from(file);

    assert!(world.get_probe("0").unwrap());
    assert!(world.get_probe("1").unwrap());
    assert!(world.get_probe("2").unwrap());
    assert!(world.get_probe("3").unwrap());
    assert!(world.get_probe("4").unwrap());
    assert!(world.get_probe("5").unwrap());
    assert!(world.get_probe("6").unwrap());
    assert!(world.get_probe("7").unwrap());
    assert!(world.get_probe("8").unwrap());
    assert!(world.get_probe("9").unwrap());
    assert!(world.get_probe("10").unwrap());
    assert!(world.get_probe("11").unwrap());
    assert!(!world.get_probe("12").unwrap());
    assert!(!world.get_probe("13").unwrap());
    assert!(!world.get_probe("14").unwrap());
    assert!(!world.get_probe("15").unwrap());
    world.step_with_trigger();
    world.step();
    world.step();
    assert!(world.get_probe("0").unwrap());
    assert!(world.get_probe("1").unwrap());
    assert!(world.get_probe("2").unwrap());
    assert!(world.get_probe("3").unwrap());
    assert!(!world.get_probe("4").unwrap());
    assert!(!world.get_probe("5").unwrap());
    assert!(!world.get_probe("6").unwrap());
    assert!(!world.get_probe("7").unwrap());
    assert!(!world.get_probe("8").unwrap());
    assert!(!world.get_probe("9").unwrap());
    assert!(!world.get_probe("10").unwrap());
    assert!(!world.get_probe("11").unwrap());
    assert!(!world.get_probe("12").unwrap());
    assert!(!world.get_probe("13").unwrap());
    assert!(!world.get_probe("14").unwrap());
    assert!(!world.get_probe("15").unwrap());
    world.step();
    assert!(world.get_probe("0").unwrap());
    assert!(world.get_probe("1").unwrap());
    assert!(world.get_probe("2").unwrap());
    assert!(world.get_probe("3").unwrap());
    assert!(world.get_probe("4").unwrap());
    assert!(world.get_probe("5").unwrap());
    assert!(world.get_probe("6").unwrap());
    assert!(world.get_probe("7").unwrap());
    assert!(world.get_probe("8").unwrap());
    assert!(world.get_probe("9").unwrap());
    assert!(world.get_probe("10").unwrap());
    assert!(world.get_probe("11").unwrap());
    assert!(!world.get_probe("12").unwrap());
    assert!(!world.get_probe("13").unwrap());
    assert!(!world.get_probe("14").unwrap());
    assert!(!world.get_probe("15").unwrap());
}

