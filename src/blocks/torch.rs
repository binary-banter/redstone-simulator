use crate::blocks::comparator::{CComparator, Comparator};
use crate::blocks::facing::Facing;
use crate::blocks::probe::CProbe;
use crate::blocks::redstone::CRedstone;
use crate::blocks::repeater::CRepeater;
use crate::blocks::solid::CSolid;
use crate::blocks::{Block, BlockConnections, CBlock, OutputPower};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;
use bimap::BiMap;

#[derive(Clone, Debug)]
pub struct Torch {
    /// Whether the torch is currently lit.
    lit: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct CTorch {
    /// Whether the torch is currently lit.
    lit: bool,

    /// Direction the torch points in.
    pub facing: Facing,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    pub node: Option<NodeIndex>,
}

impl OutputPower for Torch {
    fn output_power(&self) -> u8 {
        if self.lit {
            15
        } else {
            0
        }
    }
}

impl BlockConnections for CTorch {
    fn add_edge(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        let Some(idx) = self.node else{
            unreachable!("All nodes should have an index.");
        };

        #[rustfmt::skip]
        match target {
            // Torches always connect to neighbouring redstone.
            CBlock::Redstone(CRedstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Torches connect to strong solid blocks above it.
            CBlock::Solid(CSolid { strong: Some(s_idx), .. })
            if facing == Facing::Up => {
                blocks.add_edge(idx, *s_idx, 0);
            }

            // Torches connect to probes above it.
            CBlock::Probe(CProbe { node: Some(n_idx), .. })
            if facing == Facing::Up => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Torches connect to any repeaters facing it.
            CBlock::Repeater(CRepeater { node: Some(n_idx), facing: n_facing, .. })
            if facing == n_facing.reverse() => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Torches connect to the rear of any comparator that faces it.
            CBlock::Comparator(CComparator { node: Some(n_idx), facing: n_facing, .. })
            if facing == n_facing.reverse() => {
                let Block::Comparator(Comparator{ rear, ..}) = blocks[*n_idx] else {
                    unreachable!("All nodes should have an index.");
                };
                blocks.add_edge(idx, rear, 0);
            }

            _ => {}
        };
    }

    fn add_node(&mut self, blocks: &mut RedGraph, probes: &mut BiMap<NodeIndex, String>, triggers: &mut Vec<NodeIndex>, signs: &HashMap<(usize, usize, usize), String>) {
        todo!()
    }
}

impl From<HashMap<&str, &str>> for CTorch {
    fn from(meta: HashMap<&str, &str>) -> Self {
        let lit = meta.get("lit").map(|&x| x == "true").unwrap();

        let facing = meta
            .get("facing")
            .map(|&f| Facing::from(f))
            .unwrap_or(Facing::Up);

        CTorch {
            lit,
            facing,
            node: None,
        }
    }
}
