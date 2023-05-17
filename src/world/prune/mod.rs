mod constants;
mod dead_nodes;
mod duplicate_edges;
mod groups;
mod irrelevant;
mod redstone;
mod srepeater;
mod subtractor_edges;
mod untraversable_edges;

use crate::world::prune::constants::prune_constants;
use crate::world::prune::dead_nodes::prune_dead_nodes;
use crate::world::prune::duplicate_edges::prune_duplicate_edges;
use crate::world::prune::groups::prune_groups;
use crate::world::prune::irrelevant::prune_irrelevant;
use crate::world::prune::redstone::prune_redstone;
use crate::world::prune::srepeater::replace_simple_repeaters;
use crate::world::prune::subtractor_edges::prune_subtractor_edges;
use crate::world::prune::untraversable_edges::prune_untraversable_edges;
use crate::world::CBlockGraph;

pub fn prune_graph(cblocks: &mut CBlockGraph) {
    prune_redstone(cblocks);
    prune_duplicate_edges(cblocks);
    prune_untraversable_edges(cblocks);
    prune_groups(cblocks);
    prune_duplicate_edges(cblocks);
    prune_irrelevant(cblocks);
    replace_simple_repeaters(cblocks);
    prune_subtractor_edges(cblocks);

    loop {
        let nodes = cblocks.node_count();
        prune_constants(cblocks);
        prune_dead_nodes(cblocks);
        if nodes == cblocks.node_count() {
            break;
        }
    }
}
