use redstone_simulator::test_macro::{T, F};
use redstone_simulator::test;

test!("repeater", repeater_1t; F, F, T, F);
test!("repeater", repeater_2t; F, F, F, T, T, F);
test!("repeater", repeater_3t; F, F, F, F, T, T, T, F);
test!("repeater", repeater_4t; F, F, F, F, F, T, T, T, T, F);
test!("repeater", extender; F, F, T, F);
