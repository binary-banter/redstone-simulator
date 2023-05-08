use crate::blocks::facing::Facing;
use crate::blocks::{BlockConnections, CBlock, OutputPower};
use crate::world::RedGraph;
use petgraph::stable_graph::NodeIndex;

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
    fn connect(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        todo!()
    }
}
