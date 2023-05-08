use crate::blocks::comparator::{CComparator, Comparator};
use crate::blocks::facing::Facing;
use crate::blocks::redstone::{CRedstone, Redstone};
use crate::blocks::repeater::CRepeater;
use crate::blocks::torch::CTorch;
use crate::blocks::{Block, BlockConnections, CBlock};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Copy, Clone, Debug, Default)]
pub struct CSolid {
    /// `NodeIndex` of the block that simulates the weak logic of the block. Initially set to `None`.
    pub weak: Option<NodeIndex>,

    /// `NodeIndex` of the block that simulates the strong logic of the block. Initially set to `None`.
    pub strong: Option<NodeIndex>,
}

impl BlockConnections for CSolid {
    fn add_edge(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        let (Some(w_idx), Some(s_idx)) = (self.weak, self.strong) else {
            unreachable!("All nodes should have an index.");
        };

        #[rustfmt::skip]
        match target {
            // Strong solids always connect to neighbouring redstone.
            CBlock::Redstone(CRedstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(s_idx, *n_idx, 0);
            }

            // Strong and weak solids connect to any repeaters facing them.
            CBlock::Repeater(CRepeater { node: Some(n_idx), facing: n_facing, .. })
            if facing == n_facing.reverse() => {
                blocks.add_edge(w_idx, *n_idx, 0);
                blocks.add_edge(s_idx, *n_idx, 0);
            }

            // Strong and weak solids connect to any torches facing away from them.
            CBlock::Torch(CTorch { node: Some(n_idx), facing: n_facing, .. })
            if facing == *n_facing => {
                blocks.add_edge(w_idx, *n_idx, 0);
                blocks.add_edge(s_idx, *n_idx, 0);
            }

            // Strong and weak solids connect to the rear of any comparator that face them.
            CBlock::Comparator(CComparator { node: Some(n_idx), facing: n_facing, .. })
            if facing == *n_facing => {
                let Block::Comparator(Comparator{ rear, ..}) = blocks[*n_idx] else {
                    unreachable!("All nodes should have an index.");
                };
                blocks.add_edge(w_idx, rear, 0);
                blocks.add_edge(s_idx, rear, 0);
            }

            _ => {}
        };
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, _add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        self.weak = Some(blocks.add_node(Block::Redstone(Redstone::default())));
        self.strong = Some(blocks.add_node(Block::Redstone(Redstone::default())));
    }
}
