use crate::blocks::facing::Facing;
use crate::blocks::torch::Torch;
use crate::blocks::{Block, BlockConnections, InputSide};

#[derive(Copy, Clone, Debug, Default)]
pub struct CRedstoneBlock {}

impl BlockConnections for CRedstoneBlock {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        None
    }

    fn to_block(&self) -> Block {
        Block::Torch(Torch::default())
    }
}
