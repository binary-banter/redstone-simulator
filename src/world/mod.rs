pub mod create;
pub mod data;
pub mod graph;
mod prune;
pub mod schematic;
mod step;

use crate::blocks::{Block, Edge};
use crate::blocks::{CBlock, OutputPower};
use crate::world::graph::{FastGraph, GNode};
use petgraph::prelude::StableGraph;
use std::collections::HashMap;

pub type CBlockGraph = StableGraph<CBlock, Edge, petgraph::Directed, u32>;
pub type BlockGraph = FastGraph<Block, u8>;
pub struct TickUpdatableLists {
    pub down: UpdatableList,
    pub up: UpdatableList,
}
pub type UpdatableList = Vec<&'static GNode<Block, u8>>;

/// The `World` is a pruned instance of a redstone circuit.
pub struct World {
    /// Holds the graph of the redstone circuit.
    // todo: make this private, requires implementation of Display for `World`.
    pub blocks: BlockGraph,

    /// Stores the indexes of the triggers in the `blocks` graph.
    triggers: Vec<&'static GNode<Block, u8>>,

    /// Stores a bijective map of the indexes the probes in the `blocks` graph to their names.
    probes: HashMap<String, &'static GNode<Block, u8>>,

    /// Queue that holds indexes of blocks that require an end-of-tick update.
    updatable: UpdatableList,

    /// Queue that holds indexes of blocks that require intra-tick update.
    tick_updatable: TickUpdatableLists,

    /// Global tick counter.
    tick_counter: usize,
}

impl World {
    /// Returns whether the probe is currently powered.
    pub fn get_probe(&self, name: &str) -> Option<bool> {
        let Block::Redstone(v) = &self.probes.get(name)?.weight else {
            panic!("Probe was not a `Redstone` block, something went wrong!");
        };
        Some(v.output_power() > 0)
    }

    /// Returns `HashMap` from the names of probes to whether they are currently powered.
    pub fn get_probes(&self) -> HashMap<&str, bool> {
        self.probes
            .iter()
            .map(|(s, i)| {
                let Block::Redstone(v) = &i.weight else {
                    panic!("Probe was not a `Redstone` block, something went wrong!");
                };
                (s.as_str(), v.output_power() > 0)
            })
            .collect()
    }
}
