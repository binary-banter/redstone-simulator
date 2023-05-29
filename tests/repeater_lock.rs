use redstone_simulator::test_macro::{T, F};
use redstone_simulator::test;

test!("locking", lock_rstone; F, F, T, F, F, F, F);
test!("locking", lock_rep_off; F, F, F, F, F, F, F);
test!("locking", lock_cmp_off; F, F, F, F, F, F, F);
test!("locking", lock_rep_on; T, T, T, T, T, T, T);
test!("locking", lock_rep_1t; F, F, F, F, F, F, F);
test!("locking", lock_rep_2t; F, F, T, T, T, T, F);
test!("locking", register, 2; F, F, F, F, F, T, T, T, T);
