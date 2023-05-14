use crate::blocks::facing::Facing;
use crate::blocks::torch::Torch;
use crate::blocks::{Block, BlockConnections, InputSide};
use crate::world::BlockGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Copy, Clone, Debug, Default)]
pub struct CRedstoneBlock {
    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,
}

impl BlockConnections for CRedstoneBlock {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        None
    }

    fn add_node(&mut self, blocks: &mut BlockGraph) {
        self.node = Some(blocks.add_node(Block::Torch(Torch::default())));
    }
}
