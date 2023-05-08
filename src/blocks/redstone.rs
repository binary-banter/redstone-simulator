use crate::blocks::comparator::{CComparator, Comparator};
use crate::blocks::facing::Facing;
use crate::blocks::probe::CProbe;
use crate::blocks::repeater::CRepeater;
use crate::blocks::solid::CSolid;
use crate::blocks::{Block, BlockConnections, CBlock, OutputPower, Updatable};
use crate::world::RedGraph;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::{Incoming, Outgoing};
use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone, Debug, Default)]
pub struct Redstone {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct CRedstone {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,

    /// Directions in which this block points.
    connections: Connections,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    pub node: Option<NodeIndex>,
}

#[derive(Copy, Clone, Debug)]
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

impl OutputPower for Redstone {
    fn output_power(&self) -> u8 {
        self.signal
    }
}

impl BlockConnections for CRedstone {
    fn add_edge(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        let Some(idx) = self.node else{
            unreachable!("All nodes should have an index.");
        };

        #[rustfmt::skip]
        match target {
            // Redstone always connects to neighbouring redstone.
            CBlock::Redstone(CRedstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, *n_idx, 1);
            }

            // Redstone connects to solid blocks that it faces into.
            CBlock::Solid(CSolid { weak: Some(w_idx), .. })
            if self.connections[facing] => {
                blocks.add_edge(idx, *w_idx, 0);
            }

            // Redstone connects to probe blocks that it faces into.
            CBlock::Probe(CProbe { node: Some(n_idx) })
            if self.connections[facing] => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Redstone connects to any repeaters facing it.
            CBlock::Repeater(CRepeater { node: Some(n_idx), facing: n_facing, .. })
            if facing == n_facing.reverse() => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Redstone connects to the rear of any comparator that faces it.
            CBlock::Comparator(CComparator { node: Some(n_idx), facing: n_facing, .. })
            if facing == n_facing.reverse() => {
                let Block::Comparator(Comparator { rear, .. }) = blocks[*n_idx] else {
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
        self.node = Some(blocks.add_node(Block::Redstone(Redstone {
            signal: self.signal,
        })));
    }
}

impl Updatable for Redstone {
    fn update(
        &mut self,
        idx: NodeIndex,
        tick_updatable: &mut Vec<NodeIndex>,
        blocks: &mut RedGraph,
    ) -> bool {
        let s_new = blocks
            .edges_directed(idx, Incoming)
            .map(|edge| {
                blocks[edge.source()]
                    .output_power()
                    .saturating_sub(*edge.weight())
            })
            .max()
            .unwrap_or(0);

        if self.signal != s_new {
            self.signal = s_new;
            tick_updatable.extend(blocks.neighbors_directed(idx, Outgoing));
        }

        false
    }

    fn late_updatable(
        &mut self,
        _idx: NodeIndex,
        _updatable: &mut Vec<NodeIndex>,
        _blocks: &mut RedGraph,
    ) {
    }
}

impl From<HashMap<&str, &str>> for CRedstone {
    fn from(meta: HashMap<&str, &str>) -> Self {
        CRedstone {
            signal: meta["power"].parse().unwrap(),
            connections: Connections {
                north: meta["north"] != "none",
                east: meta["east"] != "none",
                south: meta["south"] != "none",
                west: meta["west"] != "none",
            },
            node: None,
        }
    }
}

impl Redstone {
    pub fn max() -> Self {
        Redstone { signal: 15 }
    }
}
