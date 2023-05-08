use crate::blocks::facing::Facing;
use crate::blocks::{BlockConnections, CBlock, OutputPower};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Clone, Debug)]
pub struct Repeater {
    /// Whether the repeater is currently powered.
    powered: bool,

    /// Next power when count reaches the repeater delay.
    next_powered: bool,

    /// The repeater delay in ticks, ranges from 1 to 4 inclusive.
    delay: u8,

    /// Number of ticks passed since a new input signal was detected.
    count: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct CRepeater {
    /// Whether the repeater is currently powered.
    powered: bool,

    /// The repeater delay in ticks, ranges from 1 to 4 inclusive.
    delay: u8,

    /// Direction of the input side of the repeater.
    pub facing: Facing,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    pub node: Option<NodeIndex>,
}

impl OutputPower for Repeater {
    fn output_power(&self) -> u8 {
        if self.powered {
            15
        } else {
            0
        }
    }
}

impl BlockConnections for CRepeater {
    fn connect(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        todo!()
    }
}
