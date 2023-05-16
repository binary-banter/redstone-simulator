use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, BlockConnections, InputSide, ToBlock};

#[derive(Copy, Clone, Debug, Default)]
pub struct CTrigger {}

impl BlockConnections for CTrigger {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        None
    }
}
impl ToBlock for CTrigger {
    fn to_block(&self, _on_inputs: u8) -> Block {
        Block::Redstone(Redstone::default())
    }
}
