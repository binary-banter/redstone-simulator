use crate::blocks::CBlock;
use crate::world::CBlockGraph;
use petgraph::{Incoming, Outgoing};

pub fn prune_dead_nodes(cblocks: &mut CBlockGraph) {
    cblocks.retain_nodes(|blocks, y| {
        // block has both input and output
        if blocks.neighbors_directed(y, Outgoing).count() > 0
            && blocks.neighbors_directed(y, Incoming).count() > 0
        {
            return true;
        }

        match &blocks[y] {
            // retain triggers and probes
            CBlock::Probe(_) | CBlock::Trigger(_) => true,
            CBlock::Repeater(_) | CBlock::SRepeater(_) => false,
            // CBlock::SRepeater(_) => false, todo
            // retain torches with outputs (can be used as redstone blocks)
            CBlock::Torch(_) => blocks.neighbors_directed(y, Outgoing).count() > 0,
            CBlock::Comparator(c) => {
                c.contains_entity_power() && blocks.neighbors_directed(y, Outgoing).count() > 0
            }
            CBlock::Redstone(_) => unreachable!(),
            CBlock::SolidWeak(_) => unreachable!(),
            CBlock::SolidStrong(_) => unreachable!(),
            CBlock::RedstoneBlock(_) => true,
        }
    });
}
