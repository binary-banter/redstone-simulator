use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, Edge, OutputPower, Updatable};
use crate::world::RedGraph;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::{Incoming, Outgoing};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub struct Repeater {
    /// Whether the repeater is currently powered.
    powered: bool,

    /// Next power when count reaches the repeater delay.
    next_powered: bool,

    locking_signal: bool,

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
    facing: Facing,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,
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

    fn can_input(&self, facing: Facing) -> (Option<NodeIndex>, bool) {
        if self.facing == facing.rotate_left() || self.facing == facing.rotate_right() {
            (self.node, true)
        } else if self.facing == facing.rev() {
            (self.node, false)
        } else {
            (None, false)
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
            locking_signal: false,
            delay: self.delay,
            count: 0,
        })));
    }
}

impl Updatable for Repeater {
    fn update(
        &mut self,
        idx: NodeIndex,
        tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &mut RedGraph,
    ) -> bool {
        let s_new = blocks
            .edges_directed(idx, Incoming)
            .any(|edge| match edge.weight() {
                Edge::Rear(s) => blocks[edge.source()].output_power().saturating_sub(*s) > 0,
                Edge::Side(_) => false,
            });

        let locked_now = blocks
            .edges_directed(idx, Incoming)
            .any(|edge| match edge.weight() {
                Edge::Rear(_) => false,
                Edge::Side(s) => blocks[edge.source()].output_power().saturating_sub(*s) > 0,
            });
        let locked_next_tick =
            blocks
                .edges_directed(idx, Incoming)
                .any(|edge| match edge.weight() {
                    Edge::Rear(_) => false,
                    // No sub since repeater/comparator cannot loose signal strength
                    Edge::Side(_) => blocks[edge.source()].will_lock(),
                });

        if locked_now {
            self.count = 0;
            return false;
        }

        if locked_next_tick == self.locking_signal {
            tick_updatable.extend(blocks.neighbors_directed(idx, Outgoing));
        }

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

        self.locking_signal = if locked_next_tick {
            self.powered
        } else {
            if self.count + 1 == self.delay {
                self.next_powered
            } else {
                self.powered
            }
        };

        self.powered != self.next_powered
    }

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut VecDeque<NodeIndex>,
        blocks: &mut RedGraph,
    ) {
        self.count += 1;
        if self.count == self.delay {
            self.count = 0;
            self.powered = self.locking_signal;
            updatable.extend(blocks.neighbors_directed(idx, Outgoing));
        }
    }
}

impl Repeater {
    pub fn will_lock(&self) -> bool {
        self.locking_signal
    }

    pub fn delay(&self) -> u8 {
        self.delay
    }
}

impl CRepeater {
    pub fn facing(&self) -> Facing {
        self.facing
    }
}

impl From<HashMap<&str, &str>> for CRepeater {
    fn from(meta: HashMap<&str, &str>) -> Self {
        let powered = meta.get("powered").map(|&x| x == "true").unwrap();

        CRepeater {
            powered,
            facing: Facing::from(meta["facing"]),
            delay: meta["delay"].parse().unwrap(),
            node: None,
        }
    }
}
