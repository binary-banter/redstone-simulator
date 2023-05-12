use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, CBlock, Edge, OutputPower, Updatable};
use crate::world::RedGraph;
use crate::world_data::WorldData;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::{Incoming, Outgoing};
use std::collections::{HashMap, VecDeque};
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
    node: Option<NodeIndex>,
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
    fn can_output(&self, facing: Facing) -> Option<NodeIndex> {
        if self.connections[facing] {
            self.node
        } else {
            None
        }
    }

    fn can_input(&self, _facing: Facing) -> (Option<NodeIndex>, bool) {
        (self.node, false)
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
        tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &mut RedGraph,
    ) -> bool {
        let s_new = blocks
            .edges_directed(idx, Incoming)
            .filter_map(|edge| match edge.weight() {
                Edge::Rear(s) => Some(blocks[edge.source()].output_power().saturating_sub(*s)),
                Edge::Side(_) => unreachable!(),
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
        _updatable: &mut VecDeque<NodeIndex>,
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
    pub fn with_signal(signal: u8) -> Self {
        Redstone { signal }
    }
}

impl CRedstone {
    pub fn add_vertical_edges(
        &self,
        (x, y, z): (usize, usize, usize),
        blocks: &mut RedGraph,
        world: &WorldData,
    ) {
        let Some(idx) = self.node else {
            unreachable!("All nodes should have an index.");
        };

        let top = (x, y.wrapping_add(1), z);
        let bottom = (x, y.wrapping_sub(1), z);
        for f in [Facing::North, Facing::East, Facing::South, Facing::West] {
            let side = f.front((x, y, z));
            let side_down = (side.0, side.1.wrapping_sub(1), side.2);
            let side_up = (side.0, side.1.wrapping_add(1), side.2);

            // Side-down out
            if let [CBlock::Redstone(CRedstone {
                node: Some(n_idx), ..
            })] = world[side_down][..]
            {
                if world[side].iter().all(|b| b.is_transparent())
                    && !world[bottom].iter().all(|b| b.is_transparent())
                {
                    blocks.add_edge(idx, n_idx, Edge::Rear(1));
                }
            }

            // Side-up out
            if let [CBlock::Redstone(CRedstone {
                node: Some(n_idx), ..
            })] = world[side_up][..]
            {
                if world[top].iter().all(|b| b.is_transparent()) {
                    blocks.add_edge(idx, n_idx, Edge::Rear(1));
                }
            }
        }
    }
}
