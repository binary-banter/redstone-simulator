pub mod schematic;
pub mod world_create;
pub(crate) mod world_data;
mod world_logic;
mod world_prune;

use crate::blocks::{Block, Edge};
use crate::blocks::{CBlock, OutputPower};
use bimap::BiMap;
use petgraph::prelude::StableGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::{HashMap, VecDeque};

pub type CBlockGraph = StableGraph<CBlock, Edge, petgraph::Directed, u32>;
pub type BlockGraph = StableGraph<Block, Edge, petgraph::Directed, u32>;

/// The `World` is a pruned instance of a redstone circuit.
pub struct World {
    /// Holds the graph of the redstone circuit.
    // todo: make this private, requires implementation of Display for `World`.
    pub blocks: BlockGraph,

    /// Stores the indexes of the triggers in the `blocks` graph.
    triggers: Vec<NodeIndex>,

    /// Stores a bijective map of the indexes the probes in the `blocks` graph to their names.
    probes: BiMap<NodeIndex, String>,

    /// Queue that holds indexes of blocks that require an end-of-tick update.
    updatable: VecDeque<NodeIndex>,

    /// Queue that holds indexes of blocks that require intra-tick update.
    tick_updatable: VecDeque<NodeIndex>,

    /// Global tick counter.
    tick_counter: usize,
}

impl World {
    /// Returns whether the probe is currently powered.
    // todo: maybe return an Option instead.
    pub fn get_probe(&self, name: &str) -> Option<bool> {
        let Block::Redstone(v) = &self.blocks[*self.probes.get_by_right(name)?] else {
            panic!("Probe was not a `Redstone` block, something went wrong!");
        };
        Some(v.output_power() > 0)
    }

    pub fn get_probes(&self) -> HashMap<&str, bool> {
        self.probes
            .iter()
            .map(|(i, s)| {
                (
                    &s[..],
                    match &self.blocks[*i] {
                        Block::Redstone(s) => s.output_power() > 0,
                        _ => unreachable!("Probe was not a Solid block. Parsing went wrong."),
                    },
                )
            })
            .collect()
    }
}
