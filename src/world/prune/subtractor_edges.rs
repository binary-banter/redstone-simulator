use crate::blocks::CBlock;
use crate::world::CBlockGraph;
use petgraph::prelude::EdgeRef;
use petgraph::Outgoing;

pub fn prune_subtractor_edges(cblocks: &mut CBlockGraph) {
    let mut todo = Vec::new();

    for idx in cblocks.node_indices() {
        for n_idx in cblocks.neighbors_directed(idx, Outgoing) {
            let CBlock::Comparator(c) = cblocks[n_idx] else{
                continue
            };

            if !c.is_subtractor() {
                continue;
            }

            let Some(side) = cblocks.edges_connecting(idx, n_idx).find_map(|e|if e.weight().is_side() {Some(e.weight().strength_loss())} else {None}) else{
                continue
            };

            for edge in cblocks.edges_connecting(idx, n_idx).filter_map(|e| {
                if !e.weight().is_side() && e.weight().strength_loss() >= side {
                    Some(e.id())
                } else {
                    None
                }
            }) {
                todo.push(edge);
            }
        }
    }

    for edge in todo {
        cblocks.remove_edge(edge);
    }
}
