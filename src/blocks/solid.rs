use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, BlockConnections};
use crate::world::RedGraph;
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
    fn can_output(&self, _facing: Facing) -> Option<NodeIndex> {
        self.node
    }

    fn can_input(&self, _facing: Facing) -> (Option<NodeIndex>, bool) {
        (self.node, false)
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, _add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        self.node = Some(blocks.add_node(Block::Redstone(Redstone::default())));
    }
}

impl BlockConnections for CSolidWeak {
    fn can_output(&self, _facing: Facing) -> Option<NodeIndex> {
        self.node
    }

    fn can_input(&self, _facing: Facing) -> (Option<NodeIndex>, bool) {
        (self.node, false)
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, _add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        self.node = Some(blocks.add_node(Block::Redstone(Redstone::default())));
    }
}
