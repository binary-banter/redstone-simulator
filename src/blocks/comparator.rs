use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::{Block, BlockConnections, OutputPower, Updatable};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::Outgoing;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Comparator {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,

    /// Signal of the comparator during the next tick.
    next_signal: u8,

    /// Mode of the comparator, can be in `Compare` or `Subtract` mode.
    // todo: we can most likely get rid off this by having both a `Comparator` and `Subtractor`.
    mode: ComparatorMode,

    /// `NodeIndex` of the block that simulates the rear of the comparator.
    rear: NodeIndex,

    /// `NodeIndex` of the block that simulates the sides of the comparator.
    side: NodeIndex,
}

#[derive(Copy, Clone, Debug)]
pub struct CComparator {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,

    /// Direction of the input side of the repeater.
    facing: Facing,

    /// Mode of the comparator, can be in `Compare` or `Subtract` mode.
    mode: ComparatorMode,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,

    /// `NodeIndex` of the block that simulates the rear of the comparator.
    rear: Option<NodeIndex>,

    /// `NodeIndex` of the block that simulates the sides of the comparator.
    side: Option<NodeIndex>,
}

#[derive(Copy, Clone, Debug)]
pub enum ComparatorMode {
    Compare,
    Subtract,
}

impl From<&str> for ComparatorMode {
    fn from(s: &str) -> Self {
        match s {
            "compare" => Self::Compare,
            "subtract" => Self::Subtract,
            _ => unreachable!(),
        }
    }
}

impl OutputPower for Comparator {
    fn output_power(&self) -> u8 {
        self.signal
    }
}

impl BlockConnections for CComparator {
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
        let comp = blocks.add_node(Block::Comparator(Comparator {
            signal: self.signal,
            next_signal: self.signal,
            mode: self.mode,
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

impl Updatable for Comparator {
    fn update(
        &mut self,
        _idx: NodeIndex,
        _tick_updatable: &mut Vec<NodeIndex>,
        blocks: &mut RedGraph,
    ) -> bool {
        let rear = blocks
            .node_weight(self.rear)
            .map(|b| b.output_power())
            .unwrap_or(0);
        let side = blocks
            .node_weight(self.side)
            .map(|b| b.output_power())
            .unwrap_or(0);

        self.next_signal = match self.mode {
            ComparatorMode::Compare if side <= rear => rear,
            ComparatorMode::Compare => 0,
            ComparatorMode::Subtract => rear.saturating_sub(side),
        };

        self.signal != self.next_signal
    }

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut Vec<NodeIndex>,
        blocks: &mut RedGraph,
    ) {
        self.signal = self.next_signal;
        updatable.extend(blocks.neighbors_directed(idx, Outgoing));
    }
}

impl From<HashMap<&str, &str>> for CComparator {
    fn from(meta: HashMap<&str, &str>) -> Self {
        let signal = if meta["powered"] == "true" {
            1
        } else{
            0
        };

        CComparator {
            signal,
            facing: Facing::from(meta["facing"]),
            mode: ComparatorMode::from(meta["mode"]),
            node: None,
            rear: None,
            side: None,
        }
    }
}

impl CComparator {
    pub fn facing(&self) -> Facing {
        self.facing
    }
}
