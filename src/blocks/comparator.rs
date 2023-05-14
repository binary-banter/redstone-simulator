use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, Edge, OutputPower, Updatable};
use crate::world::schematic::SchemBlockEntity;
use crate::world::BlockGraph;
use nbt::Value;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::Incoming;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub struct Comparator {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,

    /// Signal of the comparator during the next tick.
    next_signal: u8,

    entity_power: Option<u8>,

    /// Mode of the comparator, can be in `Compare` or `Subtract` mode.
    // todo: we can most likely get rid off this by having both a `Comparator` and `Subtractor`.
    mode: ComparatorMode,

    last_update: usize,
}

impl Comparator {
    pub fn contains_entity_power(&self) -> bool {
        self.entity_power.is_some()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CComparator {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,

    /// Direction of the input side of the repeater.
    facing: Facing,

    /// Mode of the comparator, can be in `Compare` or `Subtract` mode.
    mode: ComparatorMode,

    entity_power: Option<u8>,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    node: Option<NodeIndex>,
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

    fn can_input(&self, facing: Facing) -> (Option<NodeIndex>, bool) {
        if self.facing == facing.rotate_left() || self.facing == facing.rotate_right() {
            (self.node, true)
        } else if self.facing == facing.rev() {
            (self.node, false)
        } else {
            (None, false)
        }
    }

    fn add_node<F, G>(&mut self, blocks: &mut BlockGraph, _add_probe: &mut F, _add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        self.node = Some(blocks.add_node(Block::Comparator(Comparator {
            signal: self.signal,
            next_signal: self.signal,
            entity_power: self.entity_power,
            mode: self.mode,
            last_update: usize::MAX,
        })));
    }
}

impl Updatable for Comparator {
    #[inline(always)]
    fn update(
        &mut self,
        idx: NodeIndex,
        _tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &BlockGraph,
    ) -> bool {
        let rear = blocks
            .edges_directed(idx, Incoming)
            .filter_map(|edge| match edge.weight() {
                Edge::Rear(s) => Some(blocks[edge.source()].output_power().saturating_sub(*s)),
                Edge::Side(_) => None,
            })
            .max()
            .max(self.entity_power)
            .unwrap_or(0);

        let side = blocks
            .edges_directed(idx, Incoming)
            .filter_map(|edge| match edge.weight() {
                Edge::Rear(_) => None,
                Edge::Side(s) => Some(blocks[edge.source()].output_power().saturating_sub(*s)),
            })
            .max()
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
        _idx: NodeIndex,
        _updatable: &mut VecDeque<NodeIndex>,
        tick_counter: usize,
    ) -> bool {
        if tick_counter == self.last_update {
            return false;
        }
        self.last_update = tick_counter;

        self.signal = self.next_signal;
        true
    }
}

impl From<HashMap<&str, &str>> for CComparator {
    fn from(meta: HashMap<&str, &str>) -> Self {
        CComparator {
            signal: 0,
            facing: Facing::from(meta["facing"]),
            mode: ComparatorMode::from(meta["mode"]),
            entity_power: None,
            node: None,
        }
    }
}

impl CComparator {
    pub fn signal_from_entity(&mut self, entity: &SchemBlockEntity) {
        let Some(Value::Byte(s)) = entity.props.get("OutputSignal") else{
            unreachable!("Every comparator should have an OutputSignal.");
        };

        self.signal = *s as u8;
    }

    pub fn signal_set(&mut self, rear_power: Option<u8>) {
        self.entity_power = rear_power;
    }

    pub fn facing(&self) -> Facing {
        self.facing
    }
}
