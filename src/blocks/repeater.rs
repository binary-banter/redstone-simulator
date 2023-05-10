use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{redstone, Block, BlockConnections, OutputPower, Updatable};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::Outgoing;
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

    /// `NodeIndex` of the block that simulates the rear of the repeater.
    rear: NodeIndex,

    /// `NodeIndex` of the block that simulates the sides of the repeater.
    side: NodeIndex,
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

    /// `NodeIndex` of the block that simulates the rear of the repeater.
    rear: Option<NodeIndex>,

    /// `NodeIndex` of the block that simulates the sides of the repeater.
    side: Option<NodeIndex>,
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
        if self.facing == facing.rotate_left() || self.facing == facing.rotate_right() {
            self.side
        } else if self.facing == facing.rev() {
            self.rear
        } else {
            None
        }
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, _add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        let rear = blocks.add_node(Block::Redstone(Redstone::default()));
        let side = blocks.add_node(Block::Redstone(Redstone::default()));
        let comp = blocks.add_node(Block::Repeater(Repeater {
            powered: self.powered,
            next_powered: self.powered,
            delay: self.delay,
            count: 0,
            rear,
            side,
        }));
        blocks.add_edge(rear, comp, 0);
        blocks.add_edge(side, comp, 0);
        self.node = Some(comp);
        self.rear = Some(rear);
        self.side = Some(side);
    }
}

impl Updatable for Repeater {
    fn update(
        &mut self,
        idx: NodeIndex,
        tick_updatable: &mut Vec<NodeIndex>,
        blocks: &mut RedGraph,
    ) -> bool {
        let s_new = blocks
            .node_weight(self.rear)
            .map(|b| b.output_power())
            .unwrap_or(0)
            > 0;
        let locked = blocks
            .node_weight(self.side)
            .map(|b| b.output_power())
            .unwrap_or(0)
            > 0;

        if self.count == self.delay {
            self.count = 0;
            self.powered = self.next_powered;
            tick_updatable.extend(blocks.neighbors_directed(idx, Outgoing));
        }

        if locked {
            self.count = 0;
            return false;
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

        self.powered != self.next_powered
    }

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut Vec<NodeIndex>,
        blocks: &mut RedGraph,
    ) {
        self.count += 1;
        if self.count + 1 == self.delay {
            updatable.push(idx);
            updatable.extend(blocks.neighbors_directed(idx, Outgoing)); // lockable
        }
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
            rear: None,
            side: None,
        }
    }
}
