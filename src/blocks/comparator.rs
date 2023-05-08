use crate::blocks::facing::Facing;
use crate::blocks::probe::CProbe;
use crate::blocks::redstone::{CRedstone, Redstone};
use crate::blocks::repeater::CRepeater;
use crate::blocks::solid::CSolid;
use crate::blocks::{Block, BlockConnections, CBlock, OutputPower, Updatable};
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
    pub rear: NodeIndex,

    /// `NodeIndex` of the block that simulates the sides of the comparator.
    pub side: NodeIndex,
}

#[derive(Copy, Clone, Debug)]
pub struct CComparator {
    /// Signal ranges from 0 to 15 inclusive.
    signal: u8,

    /// Direction of the input side of the repeater.
    pub facing: Facing,

    /// Mode of the comparator, can be in `Compare` or `Subtract` mode.
    mode: ComparatorMode,

    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    pub node: Option<NodeIndex>,
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
    fn add_edge(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        // Return early if the target block is not behind the comparator.
        if self.facing != facing.reverse() {
            return;
        }

        let Some(idx) = self.node else{
            unreachable!("All nodes should have an index.");
        };

        #[rustfmt::skip]
        match target {
            // Repeaters always connect to redstone.
            CBlock::Redstone(CRedstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Repeaters always connect to strong solid blocks.
            CBlock::Solid(CSolid { strong: Some(s_idx), .. }) => {
                blocks.add_edge(idx, *s_idx, 0);
            }

            // Repeaters always connect to probes.
            CBlock::Probe(CProbe { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Repeaters connect to any repeaters with the same facing.
            CBlock::Repeater(CRepeater { node: Some(n_idx), facing: n_facing, .. })
            if self.facing == *n_facing => {
                blocks.add_edge(idx, *n_idx, 0);
            }

            // Repeaters connect to the rear of any comparator that faces it.
            CBlock::Comparator(CComparator { node: Some(n_idx), facing: n_facing, .. })
            if self.facing == *n_facing => {
                let Block::Comparator(Comparator { rear, .. }) = blocks[*n_idx] else {
                    unreachable!("All nodes should have an index.");
                };
                blocks.add_edge(idx, rear, 0);
            }

            // Repeaters connect to the side of any comparator that faces it.
            CBlock::Comparator(CComparator { node: Some(n_idx), facing: n_facing, .. })
            if self.facing == n_facing.rotate_left() || self.facing == n_facing.rotate_right() => {
                let Block::Comparator(Comparator { side, .. }) = blocks[*n_idx] else {
                    unreachable!("All nodes should have an index.");
                };
                blocks.add_edge(idx, side, 0);
            }

            _ => {}
        };
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
        self.node = Some(comp)
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
        CComparator {
            signal: 0,
            facing: Facing::from(meta["facing"]),
            mode: ComparatorMode::from(meta["mode"]),
            node: None,
        }
    }
}
