use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, OutputPower, Updatable};
use crate::world::RedGraph;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::{Incoming, Outgoing};
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
    fn can_output(&self, facing: Facing) -> Option<NodeIndex> {
        if self.facing == facing.rev() {
            self.node
        } else {
            None
        }
    }

    fn can_input(&self, facing: Facing) -> Option<NodeIndex> {
        if self.facing == facing.rev() {
            self.node
        } else {
            None
        }
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, _add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        self.node = Some(blocks.add_node(Block::Repeater(Repeater {
            powered: self.powered,
            next_powered: self.powered,
            delay: self.delay,
            count: 0,
        })))
    }
}

impl Updatable for Repeater {
    fn update(
        &mut self,
        idx: NodeIndex,
        _tick_updatable: &mut Vec<NodeIndex>,
        blocks: &mut RedGraph,
    ) -> bool {
        let s_new = blocks
            .edges_directed(idx, Incoming)
            .map(|edge| {
                blocks[edge.source()]
                    .output_power()
                    .saturating_sub(*edge.weight())
            })
            .any(|s| s > 0);

        // if signal strength has changed, update neighbours
        match (s_new, self.next_powered == s_new, self.count == 0) {
            // Signal changed upwards: update next signal and reset count.
            (true, false, _) => {
                self.next_powered = s_new;
                self.count = 0;
            }
            // Signal changed downward, and is not propagating already: update next signal.
            (false, false, true) => {
                self.next_powered = s_new;
            }
            // Other cases.
            (_, _, _) => {}
        };

        self.powered != self.next_powered
    }

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut Vec<NodeIndex>,
        blocks: &mut RedGraph,
    ) {
        self.count += 1;
        if self.count == self.delay {
            self.powered = self.next_powered;
            self.count = 0;
            updatable.extend(blocks.neighbors_directed(idx, Outgoing));
        }
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
