use crate::blocks::redstone_block::CRedstoneBlock;
use crate::blocks::CBlock;
use crate::world::CBlockGraph;
use itertools::Itertools;
use petgraph::prelude::*;
use petgraph::{Incoming, Outgoing};

pub fn prune_constants(cblocks: &mut CBlockGraph) {
    prune_torches(cblocks);

    for rblock in cblocks
        .node_indices()
        .filter(|rblock| matches!(cblocks[*rblock], CBlock::RedstoneBlock(_)))
        .collect_vec()
        .into_iter()
    {
        let mut todo: Vec<EdgeIndex> = cblocks
            .edges_directed(rblock, Outgoing)
            .map(|e| e.id())
            .collect_vec();

        while let Some(e) = todo.pop() {
            if cblocks[e].is_side() {
                continue;
            }
            let nb = cblocks.edge_endpoints(e).unwrap().1;
            match cblocks[nb] {
                CBlock::SRepeater(_) => {
                    cblocks
                        .edges_directed(nb, Outgoing)
                        .map(|e| e.id())
                        .collect_vec()
                        .into_iter()
                        .for_each(|nb2| {
                            todo.push(cblocks.add_edge(
                                rblock,
                                cblocks.edge_endpoints(nb2).unwrap().1,
                                cblocks[nb2],
                            ));
                        });
                    cblocks.remove_node(nb);
                }
                CBlock::Torch(_) => {
                    cblocks.remove_node(nb);
                }
                CBlock::Comparator(_) => {}
                CBlock::Repeater(_) => {}

                CBlock::Redstone(_)
                | CBlock::SolidWeak(_)
                | CBlock::SolidStrong(_)
                | CBlock::Trigger(_)
                | CBlock::RedstoneBlock(_) => unreachable!(),
                CBlock::Probe(_) => {}
            }
        }
    }
}

fn prune_torches(cblocks: &mut CBlockGraph) {
    for idx in cblocks.node_indices().collect_vec() {
        if !matches!(cblocks[idx], CBlock::Torch(_)) {
            continue;
        }

        if cblocks.neighbors_directed(idx, Incoming).count() > 0 {
            continue;
        }

        cblocks[idx] = CBlock::RedstoneBlock(CRedstoneBlock::default());
    }
}
