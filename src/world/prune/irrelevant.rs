use std::collections::HashSet;
use itertools::Itertools;
use petgraph::prelude::*;
use crate::blocks::CBlock;
use crate::world::CBlockGraph;

pub fn prune_irrelevant(cblocks: &mut CBlockGraph) {
    let mut visited: HashSet<NodeIndex> = HashSet::from_iter(
        cblocks.node_indices().filter(|idx| matches!(cblocks[*idx], CBlock::Probe(_) | CBlock::Trigger(_)))
    );
    let mut todo: Vec<NodeIndex> = cblocks.node_indices().filter(|idx| matches!(cblocks[*idx], CBlock::Probe(_))).collect_vec();

    while let Some(idx) = todo.pop() {
        for nb in cblocks.neighbors_directed(idx, Incoming) {
            if visited.contains(&nb) {
                continue;
            }
            visited.insert(nb);
            todo.push(nb);
        }
    }

    cblocks.retain_nodes(|_, n| visited.contains(&n));
}