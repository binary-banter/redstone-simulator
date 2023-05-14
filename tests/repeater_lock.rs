use redstone_simulator::world::World;
use std::fs::File;

macro_rules! test {
    ($name:ident, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr) => {
        #[test]
        fn $name() {
            let file = File::open("./schematics/locking.schem").unwrap();
            let mut world = World::from(file);

            assert_eq!(
                world.get_probe(stringify!($name)).unwrap(),
                $b1,
                "T0 expected {} got {}",
                $b1,
                world.get_probe(stringify!($name)).unwrap()
            );
            world.step_with_trigger();
            assert_eq!(
                world.get_probe(stringify!($name)).unwrap(),
                $b2,
                "T1 expected {} got {}",
                $b2,
                world.get_probe(stringify!($name)).unwrap()
            );
            world.step();
            assert_eq!(
                world.get_probe(stringify!($name)).unwrap(),
                $b3,
                "T2 expected {} got {}",
                $b3,
                world.get_probe(stringify!($name)).unwrap()
            );
            world.step();
            assert_eq!(
                world.get_probe(stringify!($name)).unwrap(),
                $b4,
                "T3 expected {} got {}",
                $b4,
                world.get_probe(stringify!($name)).unwrap()
            );
            world.step();
            assert_eq!(
                world.get_probe(stringify!($name)).unwrap(),
                $b5,
                "T4 expected {} got {}",
                $b5,
                world.get_probe(stringify!($name)).unwrap()
            );
            world.step();
            assert_eq!(
                world.get_probe(stringify!($name)).unwrap(),
                $b6,
                "T5 expected {} got {}",
                $b6,
                world.get_probe(stringify!($name)).unwrap()
            );
            world.step();
            assert_eq!(
                world.get_probe(stringify!($name)).unwrap(),
                $b7,
                "T6 expected {} got {}",
                $b7,
                world.get_probe(stringify!($name)).unwrap()
            );
        }
    };
}

test!(lock_rstone, false, false, true, false, false, false, false);
test!(
    lock_rep_off,
    false,
    false,
    false,
    false,
    false,
    false,
    false
);
test!(
    lock_cmp_off,
    false,
    false,
    false,
    false,
    false,
    false,
    false
);
test!(lock_rep_on, true, true, true, true, true, true, true);
test!(lock_rep_1t, false, false, false, false, false, false, false);
test!(lock_rep_2t, false, false, true, true, true, true, false);

#[test]
fn register() {
    let file = File::open("./schematics/register.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("out").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("out").unwrap());
    world.step_with_trigger();
    assert!(!world.get_probe("out").unwrap());
    world.step();
    assert!(!world.get_probe("out").unwrap());
    world.step();
    assert!(!world.get_probe("out").unwrap());
    world.step();
    assert!(world.get_probe("out").unwrap());
    world.step();
    assert!(world.get_probe("out").unwrap());
    world.step();
    assert!(world.get_probe("out").unwrap());
    world.step();
    assert!(world.get_probe("out").unwrap());
    world.step();
}

// #[test]
// fn lock_rep_2t_test() {
//     let file = File::open("./schematics/lock_rep_2t.schem").unwrap();
//     let mut world = World::from(file);
//
//     println!("{:?}", world.blocks);
//
//     assert!(!world.get_probe("lock_rep_2t"));
//     world.step_with_trigger();
//     assert!(!world.get_probe("lock_rep_2t"));
//     world.step();
//     assert!(world.get_probe("lock_rep_2t"));
//     world.step();
//     assert!(world.get_probe("lock_rep_2t"));
//     world.step();
//     assert!(world.get_probe("lock_rep_2t"));
//     world.step();
//     assert!(world.get_probe("lock_rep_2t"));
//     world.step();
//     assert!(!world.get_probe("lock_rep_2t"));
// }
