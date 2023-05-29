use redstone_simulator::test_macro::{T, F};
use redstone_simulator::test;

test!("redstone_block", side, 0; T, T);
test!("redstone_block", top, 0; T, T);
test!("redstone_block", hug, 0; F, F);
test!("redstone_block", torch, 0; F, F);
