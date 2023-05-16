use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::blocks::{Block, Edge, OutputPower, ToBlock, Updatable};
use crate::world::BlockGraph;
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::NodeIndex;
use petgraph::Incoming;

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
            powered: AtomicBool::new(self.powered),
            last_update: AtomicUsize::new(usize::MAX),
        })
    }
}

#[derive(Debug)]
pub struct SRepeater {
    /// Whether the repeater is currently powered.
    powered: AtomicBool,

    last_update: AtomicUsize,
}

impl OutputPower for SRepeater {
    fn output_power(&self) -> u8 {
        if self.powered.load(Ordering::Relaxed) {
            15
        } else {
            0
        }
    }
}

impl Updatable for SRepeater {
    #[inline(always)]
    fn update(
        &self,
        idx: NodeIndex,
        _tick_updatable: &mut Vec<NodeIndex>,
        blocks: &BlockGraph,
    ) -> bool {
        let s_new = blocks
            .edges_directed(idx, Incoming)
            .any(|edge| match edge.weight() {
                Edge::Rear(s) => blocks[edge.source()].output_power().saturating_sub(*s) > 0,
                Edge::Side(_) => unreachable!(),
            });

        s_new != self.powered.load(Ordering::Relaxed)
    }

    fn late_updatable(
        &self,
        _idx: NodeIndex,
        _updatable: &mut Vec<NodeIndex>,
        tick_counter: usize,
    ) -> bool {
        if tick_counter == self.last_update.load(Ordering::Relaxed) {
            return false;
        }
        self.last_update.store(tick_counter, Ordering::Relaxed);

        self.powered.store(!self.powered.load(Ordering::Relaxed), Ordering::Relaxed);

        true
    }
}

impl SRepeater {
    pub fn with_power(powered: bool) -> SRepeater {
        SRepeater {
            powered: AtomicBool::new(powered),
            last_update: AtomicUsize::new(usize::MAX),
        }
    }
}
