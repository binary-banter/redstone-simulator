use std::collections::hash_map::Entry;
use std::collections::HashMap;
use petgraph::prelude::*;
use petgraph::visit::IntoEdgeReferences;
use crate::world::CBlockGraph;

pub fn prune_duplicate_edges(cblocks: &mut CBlockGraph) {
    let mut best_edges: HashMap<(NodeIndex, NodeIndex, bool), EdgeIndex> = HashMap::new();
    let mut edges_to_remove = Vec::new();
    for edge in cblocks.edge_references() {
        match best_edges.entry((edge.source(), edge.target(), edge.weight().is_side())) {
            Entry::Occupied(mut e) => {
                if *edge.weight() >= cblocks[*e.get()] {
                    // remove edge
                    edges_to_remove.push(edge.id());
                } else {
                    // new best edge
                    edges_to_remove.push(*e.get());
                    e.insert(edge.id());
                }
            }
            Entry::Vacant(e) => {
                e.insert(edge.id());
            }
        }
        edge.source();
    }

    edges_to_remove.into_iter().for_each(|e| {
        cblocks.remove_edge(e);
    })
}