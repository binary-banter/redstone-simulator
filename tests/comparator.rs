use redstone_simulator::test_macro::{T, F};
use redstone_simulator::test;

test!("comparator", cmp_repeat; F, F, T, F);
test!("comparator", sub_repeat; F, F, T, F);
test!("comparator", cmp_off; F, F, F, F);
test!("comparator", cmp_on; F, F, T, F);
test!("comparator", cmp_double_off; F, F, F, F);
test!("comparator", cmp_double_on; F, F, T, F);
test!("comparator", sub_off; F, F, F, F);
test!("comparator", sub_on; F, F, T, F);
test!("comparator", sub_double_off; F, F, F, F);
test!("comparator", sub_double_on; F, F, T, F);

//cmp_ss1
test!("comparator", cmp_ss_on; F, F, T, F);
test!("comparator", cmp_ss_off; F, F, F, F);

//cmp_ss2
test!("comparator", cmp_ss_on_2; F, F, F, T, F);
test!("comparator", cmp_ss_off_2; F, F, F, F, F);

//cmp_inputs
test!("comparator_inputs", sub_redblock; F, F, F, F);
test!("comparator_inputs", sub_solid; F, F, T, F);
test!("comparator_inputs", sub_repeater; F, F, F, F);

//cmp_cycle
test!("cmp_cycle", cmp_cycle_off, 0; F, F);
test!("cmp_cycle", cmp_cycle_on, 0; T, T);

test!("cmp_entity", cmp_entity; T, T, F, T);
test!("cmp_prune", cmp_prune; F, F, F);
