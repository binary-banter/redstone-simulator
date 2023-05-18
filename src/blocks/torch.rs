use std::cell::Cell;
use crate::blocks::facing::Facing;
use crate::blocks::srepeater::SRepeater;
use crate::blocks::{Block, BlockConnections, InputSide, OutputPower, ToBlock};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct CTorch {
    /// Whether the torch is currently lit.
    lit: bool,

    /// Direction the torch points in.
    facing: Facing,
}

impl OutputPower for CTorch {
    fn output_power(&self) -> u8 {
        if self.lit {
            15
        } else {
            0
        }
    }
}

impl BlockConnections for CTorch {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, facing: Facing) -> Option<InputSide> {
        if self.facing == facing {
            Some(InputSide::Rear)
        } else {
            None
        }
    }
}
impl ToBlock for CTorch {
    fn to_block(&self, on_inputs: u8) -> Block {
        Block::SRepeater(SRepeater {
            powered: Cell::new(self.lit),
            last_update: Cell::new(usize::MAX),
            on_inputs: Cell::new(on_inputs),
        })
    }
}

impl From<HashMap<&str, &str>> for CTorch {
    fn from(meta: HashMap<&str, &str>) -> Self {
        let lit = meta.get("lit").map(|&x| x == "true").unwrap();

        let facing = meta
            .get("facing")
            .map(|&f| Facing::from(f))
            .unwrap_or(Facing::Up);

        CTorch { lit, facing }
    }
}
