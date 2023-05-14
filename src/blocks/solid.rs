use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, BlockConnections, InputSide};
use crate::world::BlockGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Copy, Clone, Debug, Default)]
pub struct CSolidStrong {
    /// `NodeIndex` of the block that simulates the strong logic of the block. Initially set to `None`.
    node: Option<NodeIndex>,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct CSolidWeak {
    /// `NodeIndex` of the block that simulates the weak logic of the block. Initially set to `None`.
    node: Option<NodeIndex>,
}

impl BlockConnections for CSolidStrong {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        Some(InputSide::Rear)
    }

    fn add_node(&mut self, blocks: &mut BlockGraph) {
        self.node = Some(blocks.add_node(Block::Redstone(Redstone::default())));
    }
}

impl BlockConnections for CSolidWeak {
    fn can_output(&self, _facing: Facing) -> bool {
        true
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        Some(InputSide::Rear)
    }

    fn add_node(&mut self, blocks: &mut BlockGraph) {
        self.node = Some(blocks.add_node(Block::Redstone(Redstone::default())));
    }
}
