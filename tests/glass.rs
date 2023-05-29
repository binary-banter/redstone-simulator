use redstone_simulator::test_macro::{T, F};
use redstone_simulator::test;

test!("glass", rep_in_glass; F, F, F);
test!("glass", rep_out_glass; F, F);
test!("glass", over_glass; F, F, F);
test!("glass", glass_tower_1; F, T);
test!("glass", glass_tower_2; F, T);
test!("glass", glass_down; F, F);
test!("glass", glass_through; F, T);
test!("redstone_split", up; F, T);
test!("redstone_split", down; F, T);

