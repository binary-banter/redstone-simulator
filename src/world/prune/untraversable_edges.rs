use crate::world::CBlockGraph;

pub fn prune_untraversable_edges(cblocks: &mut CBlockGraph) {
    cblocks.retain_edges(|g, e| g[e].strength_loss() < 15)
}
