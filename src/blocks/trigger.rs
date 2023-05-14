use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, BlockConnections, InputSide};
use crate::world::BlockGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Copy, Clone, Debug, Default)]
pub struct CTrigger {
    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    pub node: Option<NodeIndex>,
}

impl BlockConnections for CTrigger {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        None
    }

    fn add_node(&mut self, blocks: &mut BlockGraph) {
        let idx = blocks.add_node(Block::Redstone(Redstone::default()));
        self.node = Some(idx);
    }
}
