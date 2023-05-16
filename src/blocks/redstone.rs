use crate::blocks::facing::Facing;
use crate::blocks::{
    Block, BlockConnections, CBlock, Edge, InputSide, OutputPower, ToBlock, Updatable,
};
use crate::world::data::WorldData;
use crate::world::graph::GNode;
use crate::world::{CBlockGraph, UpdatableList};
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;
use std::ops::Index;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Default)]
pub struct Redstone {
    /// Signal ranges from 0 to 15 inclusive.
    signal: AtomicBool,
}

#[derive(Copy, Clone, Debug)]
pub struct CRedstone {
    /// Signal ranges from 0 to 15 inclusive.
    signal: bool,

    /// Directions in which this block points.
    connections: Connections,
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
        if self.signal.load(Ordering::Relaxed) {
            15
        } else {
            0
        }
    }
}

impl BlockConnections for CRedstone {
    fn can_output(&self, facing: Facing) -> bool {
        self.connections[facing]
    }

    fn can_input(&self, _facing: Facing) -> Option<InputSide> {
        Some(InputSide::Rear)
    }
}
impl ToBlock for CRedstone {
    fn to_block(&self, _on_inputs: u8) -> Block {
        Block::Redstone(Redstone {
            signal: AtomicBool::new(self.signal),
        })
    }
}

impl Updatable for Redstone {
    #[inline(always)]
    fn update(
        &self,
        idx: &'static GNode<Block, u8>,
        _tick_updatable: &mut UpdatableList,
        _up: bool,
    ) -> bool {
        let s_new = idx
            .incoming_rear
            .iter()
            .any(|e| e.node.weight.output_power().saturating_sub(e.weight) > 0);

        if self.signal.load(Ordering::Relaxed) != s_new {
            self.signal.store(s_new, Ordering::Relaxed);
        }

        false
    }

    fn late_update(
        &self,
        _idx: &'static GNode<Block, u8>,
        _tick_updatable: &mut UpdatableList,
        _tick_counter: usize,
    ) -> Option<(u8, u8)> {
        unreachable!()
    }
}

impl From<HashMap<&str, &str>> for CRedstone {
    fn from(meta: HashMap<&str, &str>) -> Self {
        CRedstone {
            signal: meta["power"].parse::<u8>().unwrap() > 0,
            connections: Connections {
                north: meta["north"] != "none",
                east: meta["east"] != "none",
                south: meta["south"] != "none",
                west: meta["west"] != "none",
            },
        }
    }
}

impl Redstone {
    pub fn with_signal(signal: bool) -> Self {
        Redstone {
            signal: AtomicBool::new(signal),
        }
    }

    pub fn toggle_signal(&self) {
        self.signal.fetch_xor(true, Ordering::Relaxed);
    }
}

impl CRedstone {
    pub fn add_vertical_edges(
        &self,
        (x, y, z): (usize, usize, usize),
        blocks: &mut CBlockGraph,
        world: &WorldData,
        indexes: &Vec<Vec<Vec<Vec<NodeIndex>>>>,
    ) {
        let idx = indexes[x][y][z][0];
        let top = (x, y.wrapping_add(1), z);
        let bottom = (x, y.wrapping_sub(1), z);
        for f in [Facing::North, Facing::East, Facing::South, Facing::West] {
            let side = f.front((x, y, z));
            let side_down = (side.0, side.1.wrapping_sub(1), side.2);
            let side_up = (side.0, side.1.wrapping_add(1), side.2);

            // Side-down out
            if let [CBlock::Redstone(CRedstone { .. })] = world[side_down][..] {
                if world[side].iter().all(|b| b.is_transparent())
                    && !world[bottom].iter().all(|b| b.is_transparent())
                {
                    blocks.add_edge(
                        idx,
                        indexes[side_down.0][side_down.1][side_down.2][0],
                        Edge::Rear(1),
                    );
                }
            }

            // Side-up out
            if let [CBlock::Redstone(CRedstone { .. })] = world[side_up][..] {
                if world[top].iter().all(|b| b.is_transparent()) {
                    blocks.add_edge(
                        idx,
                        indexes[side_up.0][side_up.1][side_up.2][0],
                        Edge::Rear(1),
                    );
                }
            }
        }
    }
}
