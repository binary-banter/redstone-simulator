mod redstone;
mod dead_nodes;
mod duplicate_edges;
mod untraversable_edges;
mod groups;
mod irrelevant;
mod srepeater;

use crate::world::prune::redstone::prune_redstone;
use crate::world::{CBlockGraph};
use crate::world::prune::dead_nodes::prune_dead_nodes;
use crate::world::prune::duplicate_edges::prune_duplicate_edges;
use crate::world::prune::groups::prune_groups;
use crate::world::prune::irrelevant::prune_irrelevant;
use crate::world::prune::srepeater::replace_simple_repeaters;
use crate::world::prune::untraversable_edges::prune_untraversable_edges;

pub fn prune_graph(cblocks: &mut CBlockGraph) {
    prune_redstone(cblocks);
    prune_duplicate_edges(cblocks);
    prune_untraversable_edges(cblocks);
    prune_groups(cblocks);
    prune_duplicate_edges(cblocks);
    prune_irrelevant(cblocks);
    replace_simple_repeaters(cblocks);
    prune_dead_nodes(cblocks);
}
