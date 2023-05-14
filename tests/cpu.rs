use redstone_simulator::world::World;
use std::fs::File;

const EXPECTED_OUTPUT: [(&str, &str); 40] = [
    ("00000000", "00000001"),
    ("00000000", "00000010"),
    ("00000001", "00000011"),
    ("00000001", "00000100"),
    ("00000000", "00000101"),
    ("00000001", "00000110"),
    ("00000001", "00000111"),
    ("00000001", "00001000"),
    ("00000001", "00000100"),
    ("00000001", "00000101"),
    ("00000010", "00000110"),
    ("00000010", "00000111"),
    ("00000010", "00001000"),
    ("00000010", "00000100"),
    ("00000001", "00000101"),
    ("00000011", "00000110"),
    ("00000011", "00000111"),
    ("00000011", "00001000"),
    ("00000011", "00000100"),
    ("00000010", "00000101"),
    ("00000101", "00000110"),
    ("00000101", "00000111"),
    ("00000101", "00001000"),
    ("00000101", "00000100"),
    ("00000011", "00000101"),
    ("00001000", "00000110"),
    ("00001000", "00000111"),
    ("00001000", "00001000"),
    ("00001000", "00000100"),
    ("00000101", "00000101"),
    ("00001101", "00000110"),
    ("00001101", "00000111"),
    ("00001101", "00001000"),
    ("00001101", "00000100"),
    ("00001000", "00000101"),
    ("00010101", "00000110"),
    ("00010101", "00000111"),
    ("00010101", "00001000"),
    ("00010101", "00000100"),
    ("00001101", "00000101"),
];

#[test]
fn cpu_test() {
    let mut world = World::from(File::open("./schematics/cpu_fib.schem").unwrap());

    for i in 0..40 {
        world.step_with_trigger();
        world.step_with_trigger();
        for _ in 0..40 {
            world.step();
        }
        let a = format!(
            "{}{}{}{}{}{}{}{}",
            world.get_probe("7").unwrap() as u8,
            world.get_probe("6").unwrap() as u8,
            world.get_probe("5").unwrap() as u8,
            world.get_probe("4").unwrap() as u8,
            world.get_probe("3").unwrap() as u8,
            world.get_probe("2").unwrap() as u8,
            world.get_probe("1").unwrap() as u8,
            world.get_probe("0").unwrap() as u8,
        );
        let pc = format!(
            "{}{}{}{}{}{}{}{}",
            world.get_probe("pc7").unwrap() as u8,
            world.get_probe("pc6").unwrap() as u8,
            world.get_probe("pc5").unwrap() as u8,
            world.get_probe("pc4").unwrap() as u8,
            world.get_probe("pc3").unwrap() as u8,
            world.get_probe("pc2").unwrap() as u8,
            world.get_probe("pc1").unwrap() as u8,
            world.get_probe("pc0").unwrap() as u8,
        );

        assert_eq!(EXPECTED_OUTPUT[i], (a.as_str(), pc.as_str()));
    }
}
