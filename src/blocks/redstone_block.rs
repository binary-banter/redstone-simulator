use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize};
use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, InputSide, ToBlock};
use crate::blocks::srepeater::SRepeater;

#[derive(Copy, Clone, Debug, Default)]
pub struct CRedstoneBlock {}

impl BlockConnections for CRedstoneBlock {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        None
    }
}
impl ToBlock for CRedstoneBlock {
    fn to_block(&self, on_inputs: u8) -> Block {
        Block::SRepeater(SRepeater {
            powered: AtomicBool::new(true),
            last_update: AtomicUsize::new(usize::MAX),
            on_inputs: AtomicU8::new(on_inputs),
        })
    }
}
