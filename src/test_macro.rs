pub const F: bool = false;
pub const T: bool = true;

#[macro_export]
macro_rules! test {
    ($file:literal, $name:ident, $triggers: expr; $($b:expr),*) => {
        #[test]
        fn $name() {
            use std::fs::File;
            use redstone_simulator::world::World;

            let file = File::open(format!("./schematics/{}.schem", $file)).unwrap();
            let mut world = World::from(file);

            let mut triggers = $triggers;

            $(
            assert_eq!(world.get_probe(stringify!($name)).unwrap(),$b);
            if triggers > 0 {
                world.step_with_trigger();
                triggers -= 1;
            } else {
                world.step();
            }
            )*
        }
    };
    ($file:literal, $name:ident; $($b:expr),*) => {
        test!{$file, $name, 1; $($b),*}
    };
}
