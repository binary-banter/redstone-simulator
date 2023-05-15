use itertools::Itertools;
use petgraph::{Incoming, Outgoing};
use crate::blocks::{CBlock, csrepeater, Edge};
use crate::world::CBlockGraph;

pub fn replace_simple_repeaters(cblocks: &mut CBlockGraph) {
            cblocks
            .node_indices()
            .collect_vec()
            .into_iter()
            .for_each(|idx| {
                let Some(CBlock::Repeater(r)) = cblocks.node_weight(idx) else{
                    return
                };

                if r.delay() != 1 {
                    return;
                }

                if cblocks
                    .edges_directed(idx, Outgoing)
                    .any(|edge| matches!(edge.weight(), Edge::Side(_)))
                {
                    return;
                }

                if cblocks
                    .edges_directed(idx, Incoming)
                    .any(|edge| matches!(edge.weight(), Edge::Side(_)))
                {
                    return;
                }

                *cblocks.node_weight_mut(idx).unwrap() = csrepeater(r.is_powered());
            });
}