use redstone_simulator::world::World;
use std::fs::File;

macro_rules! test {
    ($name:ident, $b1:expr, $b2:expr, $b3:expr, $b4:expr) => {
        #[test]
        fn $name() {
            let file = File::open("./schematics/connections.schem").unwrap();
            let mut world = World::from(file);

            assert_eq!(
                world.get_probe(stringify!($name)),
                $b1,
                "T0 expected {} got {}",
                $b1,
                world.get_probe(stringify!($name))
            );
            world.step_with_trigger();
            assert_eq!(
                world.get_probe(stringify!($name)),
                $b2,
                "T1 expected {} got {}",
                $b2,
                world.get_probe(stringify!($name))
            );
            world.step();
            assert_eq!(
                world.get_probe(stringify!($name)),
                $b3,
                "T2 expected {} got {}",
                $b3,
                world.get_probe(stringify!($name))
            );
            world.step();
            assert_eq!(
                world.get_probe(stringify!($name)),
                $b4,
                "T3 expected {} got {}",
                $b4,
                world.get_probe(stringify!($name))
            );
        }
    };
}

test!(rstone_rstone, false, true, false, false);
test!(rstone_wsolid, false, false, true, false);
test!(rstone_ssolid, false, false, false, false);
test!(rstone_rep, false, false, true, false);
test!(rstone_cmp, false, false, true, false);

test!(wsolid_rstone, false, false, false, false);
test!(wsolid_wsolid, false, false, false, false);
test!(wsolid_ssolid, false, false, false, false);
test!(wsolid_rep, false, false, true, false);
test!(wsolid_torch, true, true, false, true);
test!(wsolid_cmp, false, false, true, false);

test!(ssolid_rstone, false, false, true, false);
test!(ssolid_wsolid, false, false, false, false);
test!(ssolid_ssolid, false, false, false, false);
test!(ssolid_rep, false, false, false, true);
test!(ssolid_torch, true, true, true, false);
test!(ssolid_cmp, false, false, false, true);

test!(rep_rstone, false, false, true, false);
test!(rep_wsolid, false, false, false, true);
test!(rep_ssolid, false, false, true, false);
test!(rep_rep, false, false, false, true);
test!(rep_cmp, false, false, false, true);

test!(torch_rstone, true, true, false, true);
test!(torch_wsolid, false, false, false, false);
test!(torch_ssolid, false, false, false, false);
test!(torch_rep, true, true, true, false);
test!(torch_cmp, true, true, true, false);

test!(cmp_rstone, false, false, true, false);
test!(cmp_wsolid, false, false, false, true);
test!(cmp_ssolid, false, false, true, false);
test!(cmp_rep, false, false, false, true);
test!(cmp_cmp, false, false, false, true);

test!(rblock_rstone, true, true, true, true);
test!(rblock_wsolid, false, false, false, false);
test!(rblock_ssolid, false, false, false, false);
test!(rblock_rep, true, true, true, true);
test!(rblock_torch, false, false, false, false);
test!(rblock_cmp, true, true, true, true);

test!(torch_wsolid_up, true, true, true, false);
test!(torch_ssolid_up, true, true, false, true);

test!(rblock_rstone_up, true, true, true, true);
test!(rblock_wsolid_up, false, false, false, false);
test!(rblock_ssolid_up, false, false, false, false);
test!(rblock_rep_up, false, false, false, false);
test!(rblock_cmp_up, false, false, false, false);
test!(rblock_torch_up, false, false, false, false);

test!(rblock_rstone_dn, true, true, true, true);
test!(rblock_wsolid_dn, false, false, false, false);
test!(rblock_ssolid_dn, false, false, false, false);
test!(rblock_rep_dn, false, false, false, false);
test!(rblock_cmp_dn, false, false, false, false);
test!(rblock_torch_dn, true, true, true, true);

test!(rstone_north, false, true, false, false);
test!(rstone_east, false, true, false, false);
test!(rstone_south, false, true, false, false);
test!(rstone_west, false, true, false, false);
test!(rstone_up, false, false, false, false);
test!(rstone_dn, false, true, false, false);

test!(rstone_off_north, false, false, false, false);
test!(rstone_off_east, false, false, false, false);
test!(rstone_off_south, false, false, false, false);
test!(rstone_off_west, false, false, false, false);

test!(rep_out_top, false, false, false, false);
test!(rep_out_down, false, false, false, false);
test!(rep_out_left, false, false, false, false);
test!(rep_out_right, false, false, false, false);
test!(rstone_rep_in, false, false, false, false);

test!(cmp_out_top, false, false, false, false);
test!(cmp_out_down, false, false, false, false);
test!(cmp_out_left, false, false, false, false);
test!(cmp_out_right, false, false, false, false);
test!(cmp_in, false, false, false, false);
