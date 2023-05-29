use redstone_simulator::test_macro::{T, F};
use redstone_simulator::test;

test!("connections", rstone_rstone; F, T, F, F);
test!("connections", rstone_wsolid; F, F, T, F);
test!("connections", rstone_ssolid; F, F, F, F);
test!("connections", rstone_rep; F, F, T, F);
test!("connections", rstone_cmp; F, F, T, F);

test!("connections", wsolid_rstone; F, F, F, F);
test!("connections", wsolid_wsolid; F, F, F, F);
test!("connections", wsolid_ssolid; F, F, F, F);
test!("connections", wsolid_rep; F, F, T, F);
test!("connections", wsolid_torch; T, T, F, T);
test!("connections", wsolid_cmp; F, F, T, F);

test!("connections", ssolid_rstone; F, F, T, F);
test!("connections", ssolid_wsolid; F, F, F, F);
test!("connections", ssolid_ssolid; F, F, F, F);
test!("connections", ssolid_rep; F, F, F, T);
test!("connections", ssolid_torch; T, T, T, F);
test!("connections", ssolid_cmp; F, F, F, T);

test!("connections", rep_rstone; F, F, T, F);
test!("connections", rep_wsolid; F, F, F, T);
test!("connections", rep_ssolid; F, F, T, F);
test!("connections", rep_rep; F, F, F, T);
test!("connections", rep_cmp; F, F, F, T);

test!("connections", torch_rstone; T, T, F, T);
test!("connections", torch_wsolid; F, F, F, F);
test!("connections", torch_ssolid; F, F, F, F);
test!("connections", torch_rep; T, T, T, F);
test!("connections", torch_cmp; T, T, T, F);

test!("connections", cmp_rstone; F, F, T, F);
test!("connections", cmp_wsolid; F, F, F, T);
test!("connections", cmp_ssolid; F, F, T, F);
test!("connections", cmp_rep; F, F, F, T);
test!("connections", cmp_cmp; F, F, F, T);

test!("connections", rblock_rstone; T, T, T, T);
test!("connections", rblock_wsolid; F, F, F, F);
test!("connections", rblock_ssolid; F, F, F, F);
test!("connections", rblock_rep; T, T, T, T);
test!("connections", rblock_torch; F, F, F, F);
test!("connections", rblock_cmp; T, T, T, T);

test!("connections", torch_wsolid_up; T, T, T, F);
test!("connections", torch_ssolid_up; T, T, F, T);

test!("connections", rblock_rstone_up; T, T, T, T);
test!("connections", rblock_wsolid_up; F, F, F, F);
test!("connections", rblock_ssolid_up; F, F, F, F);
test!("connections", rblock_rep_up; F, F, F, F);
test!("connections", rblock_cmp_up; F, F, F, F);
test!("connections", rblock_torch_up; F, F, F, F);

test!("connections", rblock_rstone_dn; T, T, T, T);
test!("connections", rblock_wsolid_dn; F, F, F, F);
test!("connections", rblock_ssolid_dn; F, F, F, F);
test!("connections", rblock_rep_dn; F, F, F, F);
test!("connections", rblock_cmp_dn; F, F, F, F);
test!("connections", rblock_torch_dn; T, T, T, T);

test!("connections", rstone_north; F, T, F, F);
test!("connections", rstone_east; F, T, F, F);
test!("connections", rstone_south; F, T, F, F);
test!("connections", rstone_west; F, T, F, F);
test!("connections", rstone_up; F, F, F, F);
test!("connections", rstone_dn; F, T, F, F);

test!("connections", rstone_off_north; F, F, F, F);
test!("connections", rstone_off_east; F, F, F, F);
test!("connections", rstone_off_south; F, F, F, F);
test!("connections", rstone_off_west; F, F, F, F);

test!("connections", rep_out_top; F, F, F, F);
test!("connections", rep_out_down; F, F, F, F);
test!("connections", rep_out_left; F, F, F, F);
test!("connections", rep_out_right; F, F, F, F);
test!("connections", rstone_rep_in; F, F, F, F);

test!("connections", cmp_out_top; F, F, F, F);
test!("connections", cmp_out_down; F, F, F, F);
test!("connections", cmp_out_left; F, F, F, F);
test!("connections", cmp_out_right; F, F, F, F);
test!("connections", cmp_in; F, F, F, F);
