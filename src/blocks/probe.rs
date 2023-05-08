use std::collections::HashMap;
use bimap::BiMap;
use crate::blocks::facing::Facing;
use crate::blocks::{Block, BlockConnections, CBlock};
use crate::world::{RedGraph, World};
use petgraph::stable_graph::NodeIndex;
use crate::blocks::redstone::Redstone;

#[derive(Copy, Clone, Debug, Default)]
pub struct CProbe {
    /// `NodeIndex` of this block in the graph. Initially set to `None`.
    pub node: Option<NodeIndex>,
}

impl BlockConnections for CProbe {
    fn add_edge(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {}

    fn add_node(&mut self, blocks: &mut RedGraph, probes: &mut BiMap<NodeIndex, String>, triggers: &mut Vec<NodeIndex>, signs: &HashMap<(usize, usize, usize), String>) {
        let idx = blocks.add_node(Block::Redstone(Redstone::default()));
        self.node = Some(idx);

        let name: String =
            World::neighbours((x, y, z))
            .into_iter()
            .find_map(|nb| signs.get(&nb).cloned())
            .unwrap_or(format!("{x},{y},{z}"));
        probes.insert(idx, name);
    }
}
