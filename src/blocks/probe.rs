use crate::blocks::facing::Facing;
use crate::blocks::{BlockConnections, CBlock};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Clone, Debug)]
pub struct CProbe {
    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,
}

impl BlockConnections for CProbe {
    fn connect(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        todo!()
    }
}