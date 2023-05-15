use crate::blocks::CBlock;
use crate::world::CBlockGraph;
use itertools::Itertools;
use petgraph::prelude::*;
use std::collections::HashMap;

pub fn prune_groups(cblocks: &mut CBlockGraph) {
    let mut todo = cblocks
        .node_indices()
        .filter(|i| matches!(cblocks[*i], CBlock::Repeater(_) | CBlock::Torch(_)))
        .collect_vec();

    while let Some(idx) = todo.pop() {
        let mut repeaters: HashMap<usize, Vec<NodeIndex>> = HashMap::new();
        let mut torches = Vec::new();

        for n_idx in cblocks.neighbors_directed(idx, Outgoing) {
            // Only group items with a single parent
            if cblocks.neighbors_directed(n_idx, Incoming).count() > 1 {
                continue;
            }

            // Only group repeaters and torches.
            match &cblocks[n_idx] {
                CBlock::Repeater(v) => {
                    repeaters.entry(v.delay() as usize).or_default().push(n_idx);
                }
                CBlock::Torch(_) => torches.push(n_idx),
                _ => continue,
            }
        }

        if torches.len() > 1 {
            todo.push(merge_nodes(cblocks, torches.into_iter()));
        }
        for (_, repeaters) in repeaters.into_iter() {
            if repeaters.len() > 1 {
                todo.push(merge_nodes(cblocks, repeaters.into_iter()));
            }
        }
    }
}

fn merge_nodes(cblocks: &mut CBlockGraph, mut nodes: impl Iterator<Item = NodeIndex>) -> NodeIndex {
    let first = nodes.next().unwrap();
    for other in nodes {
        let edges = cblocks
            .edges_directed(other, Outgoing)
            .map(|e| e.id())
            .collect_vec();
        for edge in edges {
            cblocks.add_edge(
                first,
                cblocks.edge_endpoints(edge).unwrap().1,
                cblocks[edge],
            );
        }
        cblocks.remove_node(other);
    }
    first
}
