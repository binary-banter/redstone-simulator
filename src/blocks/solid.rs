use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, BlockConnections, InputSide};

#[derive(Copy, Clone, Debug, Default)]
pub struct CSolidStrong {}

#[derive(Copy, Clone, Debug, Default)]
pub struct CSolidWeak {}

impl BlockConnections for CSolidStrong {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        Some(InputSide::Rear)
    }

    fn to_block(&self) -> Block {
        Block::Redstone(Redstone::default())
    }
}

impl BlockConnections for CSolidWeak {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        Some(InputSide::Rear)
    }

    fn to_block(&self) -> Block {
        Block::Redstone(Redstone::default())
    }
}
