use crate::blocks::comparator::{CComparator, Comparator};
use crate::blocks::facing::Facing;
use crate::blocks::redstone::CRedstone;
use crate::blocks::repeater::CRepeater;
use crate::blocks::torch::CTorch;
use crate::blocks::{Block, BlockConnections, CBlock};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Copy, Clone, Debug, Default)]
pub struct CRedstoneBlock {
    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,
}

impl BlockConnections for CRedstoneBlock {
    fn add_edge(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        let Some(idx) = self.node else{
            unreachable!("All nodes should have an index.");
        };

        #[rustfmt::skip]
        match target {
            // Redstone blocks always connect to neighbouring redstone.
            CBlock::Redstone(CRedstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Redstone blocks connect to any repeaters facing it.
            CBlock::Repeater(CRepeater { node: Some(n_idx), facing: n_facing, .. })
            if facing == n_facing.reverse() => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Redstone blocks connect to any torches facing away from it.
            CBlock::Torch(CTorch { node: Some(n_idx), facing: n_facing, .. })
            if facing == *n_facing => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Redstone blocks connect to the rear of any comparator that faces it.
            CBlock::Comparator(CComparator { node: Some(n_idx), facing: n_facing, .. })
            if facing == *n_facing => {
                let Block::Comparator(Comparator{ rear, ..}) = blocks[*n_idx] else {
                    unreachable!("All nodes should have an index.");
                };
                blocks.add_edge(idx, rear, 0);
            }

            // Redstone connects to the side of any comparator that faces it.
            CBlock::Comparator(CComparator { node: Some(n_idx), facing: n_facing, .. })
            if facing == n_facing.rotate_left() || facing == n_facing.rotate_right() => {
                let Block::Comparator(Comparator { side, .. }) = blocks[*n_idx] else {
                    unreachable!("All nodes should have an index.");
                };
                blocks.add_edge(idx, side, 0);
            }

            _ => {}
        };
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, _add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        self.node = Some(blocks.add_node(Block::RedstoneBlock));
    }
}
