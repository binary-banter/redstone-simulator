use crate::blocks::{Block, Edge, OutputPower, ToBlock, Updatable};
use crate::world::BlockGraph;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::Incoming;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct CSRepeater {
    /// Whether the repeater is currently powered.
    powered: bool,
}

impl CSRepeater {
    pub fn with_powered(powered: bool) -> CSRepeater {
        CSRepeater { powered }
    }
}

impl ToBlock for CSRepeater {
    fn to_block(&self) -> Block {
        Block::SRepeater(SRepeater {
            powered: self.powered,
            last_update: usize::MAX,
        })
    }
}

#[derive(Clone, Debug)]
pub struct SRepeater {
    /// Whether the repeater is currently powered.
    powered: bool,

    last_update: usize,
}

impl OutputPower for SRepeater {
    fn output_power(&self) -> u8 {
        if self.powered {
            15
        } else {
            0
        }
    }
}

impl Updatable for SRepeater {
    #[inline(always)]
    fn update(
        &mut self,
        idx: NodeIndex,
        _tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &BlockGraph,
    ) -> bool {
        let s_new = blocks
            .edges_directed(idx, Incoming)
            .any(|edge| match edge.weight() {
                Edge::Rear(s) => blocks[edge.source()].output_power().saturating_sub(*s) > 0,
                Edge::Side(_) => unreachable!(),
            });

        s_new != self.powered
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

        self.powered = !self.powered;

        true
    }
}

impl SRepeater {
    pub fn with_power(powered: bool) -> SRepeater {
        SRepeater {
            powered,
            last_update: usize::MAX,
        }
    }
}
