use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, CBlock};
use crate::world::RedGraph;
use petgraph::prelude::StableGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::Directed;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Redstone {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,
}

#[derive(Clone, Debug)]
pub struct CRedstone {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,

    /// Directions in which this block points.
    connections: Connections,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,
}

#[derive(Clone, Debug)]
pub struct Connections {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Index<Facing> for Connections {
    type Output = bool;

    fn index(&self, index: Facing) -> &Self::Output {
        match index {
            Facing::North => &self.north,
            Facing::East => &self.east,
            Facing::South => &self.south,
            Facing::West => &self.west,
            Facing::Up => &false,
            Facing::Down => &true,
        }
    }
}

impl BlockConnections for CRedstone {
    fn connect(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        // connects to:
        // Redstone
        // weak solids if connections[facing]
        // probes if connections[facing]
        // repeater if facing == r.facing.reverse()
        //
        //             (CBlock::Redstone { node: Some(idx), .. }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if n_facing == f.reverse() => {
        //                 let Block::Comparator { rear, ..} = blocks[n_idx] else {
        //                     unreachable!();
        //                 };
        //                 blocks.add_edge(idx, rear, 0);
        //             }
        //             (CBlock::Redstone { node: Some(idx), .. }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if n_facing == f.rotate_right() || n_facing == f.rotate_left() => {
        //                 let Block::Comparator { side, ..} = blocks[n_idx] else {
        //                     unreachable!();
        //                 };
        //                 blocks.add_edge(idx, side, 0);
        //             }
    }
}
