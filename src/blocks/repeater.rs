use crate::blocks::comparator::{CComparator, Comparator};
use crate::blocks::facing::Facing;
use crate::blocks::probe::CProbe;
use crate::blocks::redstone::CRedstone;
use crate::blocks::solid::CSolid;
use crate::blocks::{Block, BlockConnections, CBlock, OutputPower};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Repeater {
    /// Whether the repeater is currently powered.
    powered: bool,

    /// Next power when count reaches the repeater delay.
    next_powered: bool,

    /// The repeater delay in ticks, ranges from 1 to 4 inclusive.
    delay: u8,

    /// Number of ticks passed since a new input signal was detected.
    count: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct CRepeater {
    /// Whether the repeater is currently powered.
    powered: bool,

    /// The repeater delay in ticks, ranges from 1 to 4 inclusive.
    delay: u8,

    /// Direction of the input side of the repeater.
    pub facing: Facing,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    pub node: Option<NodeIndex>,
}

impl OutputPower for Repeater {
    fn output_power(&self) -> u8 {
        if self.powered {
            15
        } else {
            0
        }
    }
}

impl BlockConnections for CRepeater {
    fn connect(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        // Return early if the target block is not behind the repeater.
        if self.facing != facing.reverse() {
            return;
        }

        let Some(idx) = self.node else{
            unreachable!("All nodes should have an index.");
        };

        #[rustfmt::skip]
        match target {
            // Repeaters always connect to redstone.
            CBlock::Redstone(CRedstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Repeaters always connect to strong solid blocks.
            CBlock::Solid(CSolid { strong: Some(s_idx), .. }) => {
                blocks.add_edge(idx, *s_idx, 0);
            }

            // Repeaters always connect to probes.
            CBlock::Probe(CProbe { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Repeaters connect to any repeaters with the same facing.
            CBlock::Repeater(CRepeater { node: Some(n_idx), facing: n_facing, .. })
            if self.facing == *n_facing => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Repeaters connect to the rear of any comparator that faces it.
            CBlock::Comparator(CComparator { node: Some(n_idx), facing: n_facing, .. })
            if self.facing == *n_facing => {
                let Block::Comparator(Comparator { rear, .. }) = blocks[*n_idx] else {
                    unreachable!("All nodes should have an index.");
                };
                blocks.add_edge(idx, rear, 0);
            }

            // Repeaters connect to the side of any comparator that faces it.
            CBlock::Comparator(CComparator { node: Some(n_idx), facing: n_facing, .. })
            if self.facing == n_facing.rotate_left() || self.facing == n_facing.rotate_right() => {
                let Block::Comparator(Comparator { side, .. }) = blocks[*n_idx] else {
                    unreachable!("All nodes should have an index.");
                };
                blocks.add_edge(idx, side, 0);
            }

            _ => {}
        };
    }
}

impl From<HashMap<&str, &str>> for CRepeater {
    fn from(meta: HashMap<&str, &str>) -> Self {
        CRepeater {
            powered: false,
            facing: Facing::from(meta["facing"]),
            delay: meta["delay"].parse().unwrap(),
            node: None,
        }
    }
}
