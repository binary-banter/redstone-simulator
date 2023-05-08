use crate::blocks::facing::Facing;
use crate::blocks::{BlockConnections, CBlock};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Clone, Debug)]
pub struct CSolid {
    /// `NodeIndex` of the block that simulates the weak logic of the block. Initially set to `None`.
    weak: Option<NodeIndex>,

    /// `NodeIndex` of the block that simulates the strong logic of the block. Initially set to `None`.
    strong: Option<NodeIndex>,
}

impl BlockConnections for CSolid {
    fn connect(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        todo!()
    }
}
