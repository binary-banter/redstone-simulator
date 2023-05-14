use crate::blocks::facing::Facing;
use crate::blocks::{BlockConnections, InputSide};

#[derive(Copy, Clone, Debug, Default)]
pub struct CSolidStrong {
}

#[derive(Copy, Clone, Debug, Default)]
pub struct CSolidWeak {
}

impl BlockConnections for CSolidStrong {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        Some(InputSide::Rear)
    }
}

impl BlockConnections for CSolidWeak {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        Some(InputSide::Rear)
    }
}
