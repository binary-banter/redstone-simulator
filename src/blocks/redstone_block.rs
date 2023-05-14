use crate::blocks::facing::Facing;
use crate::blocks::{BlockConnections, InputSide};

#[derive(Copy, Clone, Debug, Default)]
pub struct CRedstoneBlock {
}

impl BlockConnections for CRedstoneBlock {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        None
    }
}
