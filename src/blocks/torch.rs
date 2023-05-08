use crate::blocks::facing::Facing;
use crate::blocks::{BlockConnections, CBlock, OutputPower};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Clone, Debug)]
pub struct Torch {
    /// Whether the torch is currently lit.
    lit: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct CTorch {
    /// Whether the torch is currently lit.
    lit: bool,

    /// Direction the torch points in.
    pub facing: Facing,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    pub node: Option<NodeIndex>,
}

impl OutputPower for Torch {
    fn output_power(&self) -> u8 {
        if self.lit {
            15
        } else {
            0
        }
    }
}

impl BlockConnections for CTorch {
    fn connect(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        todo!()
    }
}
