use crate::block::Block;
use crate::world::World;
use petgraph::{Incoming, Outgoing};

impl World {
    pub fn prune_graph(&mut self) {
        for _ in 0..20 {
            self.blocks.retain_nodes(|blocks, y| {
                (blocks.neighbors_directed(y, Outgoing).count() > 0
                    && blocks.neighbors_directed(y, Incoming).count() > 0)
                    || self.probes.contains_left(&y)
                    || self.triggers.contains(&y)
                    || matches!(blocks[y], Block::RedstoneBlock | Block::Torch { .. })
            });
        }
    }
}
