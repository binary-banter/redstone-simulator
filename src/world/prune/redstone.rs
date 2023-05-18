use crate::blocks::CBlock;
use crate::world::edge::Edge;
use crate::world::CBlockGraph;
use itertools::Itertools;
use petgraph::prelude::*;
use std::collections::HashSet;

pub fn prune_redstone(cblocks: &mut CBlockGraph) {
    for node in cblocks.node_indices().collect_vec() {
        if matches!(
            cblocks[node],
            CBlock::Redstone(_) | CBlock::SolidStrong(_) | CBlock::SolidWeak(_)
        ) {
            continue;
        }

        let mut state = vec![(node, Edge::Rear(0))];
        let mut visited: HashSet<(NodeIndex, bool)> = HashSet::new();
        let mut ends = vec![];

        loop {
            let mut new_state = vec![];

            for (s, c) in state {
                for nb_edge in cblocks.edges_directed(s, Outgoing) {
                    let nb = nb_edge.target();

                    if visited.contains(&(nb, nb_edge.weight().is_side())) {
                        continue;
                    }

                    visited.insert((nb, nb_edge.weight().is_side()));

                    if !matches!(
                        cblocks[nb],
                        CBlock::Redstone(_) | CBlock::SolidStrong(_) | CBlock::SolidWeak(_)
                    ) {
                        ends.push((nb, c + nb_edge.weight()));
                        continue;
                    }

                    new_state.push((nb, c + nb_edge.weight()));
                }
            }

            if new_state.is_empty() {
                break;
            }

            state = new_state;
        }

        for (end, i) in ends {
            cblocks.add_edge(node, end, i);
        }
    }
    cblocks.retain_nodes(|blocks, n| {
        !matches!(
            blocks[n],
            CBlock::Redstone(_) | CBlock::SolidStrong(_) | CBlock::SolidWeak(_)
        )
    });
}
