use redstone_simulator::test_macro::{T, F};
use redstone_simulator::test;

test!("torch", torch; T, T, F, T);
test!("torch", torch_wall; T, T, F, T);
test!("torch", torch_strong; T, T, F, T);
