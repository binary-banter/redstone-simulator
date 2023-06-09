use crate::blocks::facing::Facing;
use crate::blocks::srepeater::SRepeater;
use crate::blocks::{Block, BlockConnections, InputSide, ToBlock};
use std::cell::Cell;

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
            powered: Cell::new(true),
            on_inputs: Cell::new(on_inputs),
        })
    }
}
