use crate::blocks::facing::Facing;
use crate::blocks::{BlockConnections, Edge, InputSide, OutputPower, Updatable};
use crate::world::BlockGraph;
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

    last_update: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct CRepeater {
    /// Whether the repeater is currently powered.
    powered: bool,

    /// The repeater delay in ticks, ranges from 1 to 4 inclusive.
    delay: u8,

    /// Direction of the input side of the repeater.
    facing: Facing,
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
    fn can_output(&self, facing: Facing) -> bool {
        self.facing == facing.rev()
    }

    fn can_input(&self, facing: Facing) -> Option<InputSide> {
        if self.facing == facing.rotate_left() || self.facing == facing.rotate_right() {
            Some(InputSide::Side)
        } else if self.facing == facing.rev() {
            Some(InputSide::Rear)
        } else {
            None
        }
    }
}

impl Updatable for Repeater {
    #[inline(always)]
    fn update(
        &mut self,
        idx: NodeIndex,
        tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &BlockGraph,
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
                // No sub since repeater/comparator cannot loose signal strength
                Edge::Side(_) => blocks[edge.source()].output_power() > 0,
            });

        if locked_now {
            return false;
        }

        let locked_next_tick =
            blocks
                .edges_directed(idx, Incoming)
                .any(|edge| match edge.weight() {
                    Edge::Rear(_) => false,
                    // No sub since repeater/comparator cannot loose signal strength
                    Edge::Side(_) => blocks[edge.source()].will_lock(),
                });

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
        } else if self.count + 1 == self.delay {
            self.next_powered
        } else {
            self.powered
        };

        self.powered != self.next_powered
    }

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut VecDeque<NodeIndex>,
        tick_counter: usize,
    ) -> bool {
        if tick_counter == self.last_update {
            return false;
        }
        self.last_update = tick_counter;

        self.count += 1;
        updatable.push_back(idx);
        if self.count == self.delay {
            self.count = 0;
            self.powered = self.locking_signal;
            true
        } else {
            false
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
        }
    }
}
