use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, BlockConnections};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Copy, Clone, Debug, Default)]
pub struct CProbe {
    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,
}

impl BlockConnections for CProbe {
    fn can_output(&self, _facing: Facing) -> Option<NodeIndex> {
        None
    }

    fn can_input(&self, _facing: Facing) -> Option<NodeIndex> {
        self.node
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        let idx = blocks.add_node(Block::Redstone(Redstone::default()));
        add_probe(idx);
        self.node = Some(idx);
    }
}
