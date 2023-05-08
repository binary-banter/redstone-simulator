use crate::blocks::comparator::{CComparator, Comparator};
use crate::blocks::facing::Facing;
use crate::blocks::probe::CProbe;
use crate::blocks::redstone::{CRedstone, Redstone};
use crate::blocks::redstone_block::CRedstoneBlock;
use crate::blocks::repeater::{CRepeater, Repeater};
use crate::blocks::solid::CSolid;
use crate::blocks::torch::{CTorch, Torch};
use crate::blocks::trigger::CTrigger;
use crate::world::RedGraph;
use petgraph::stable_graph::StableGraph;
use petgraph::Directed;

mod comparator;
mod facing;
mod probe;
mod redstone;
mod redstone_block;
mod repeater;
mod solid;
mod torch;
mod trigger;

#[derive(Clone, Debug)]
/// Blocks that end up in the graph structure of the world.
pub enum Block {
    Redstone(Redstone),
    Repeater(Repeater),
    RedstoneBlock,
    Torch(Torch),
    Comparator(Comparator),
}

/// Blocks used during the creation of the graph structure of the world.
#[derive(Clone, Debug)]
pub enum CBlock {
    Redstone(CRedstone),
    Solid(CSolid),
    Trigger(CTrigger),
    Probe(CProbe),
    Repeater(CRepeater),
    RedstoneBlock(CRedstoneBlock),
    Torch(CTorch),
    Comparator(CComparator),
    Air,
}

trait BlockConnections {
    fn connect(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph);
}

impl BlockConnections for CBlock {
    fn connect(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        match self {
            CBlock::Redstone(v) => v.connect(target, facing, blocks),
            CBlock::Solid(v) => v.connect(target, facing, blocks),
            CBlock::Trigger(v) => v.connect(target, facing, blocks),
            CBlock::Probe(v) => v.connect(target, facing, blocks),
            CBlock::Repeater(v) => v.connect(target, facing, blocks),
            CBlock::RedstoneBlock(v) => v.connect(target, facing, blocks),
            CBlock::Torch(v) => v.connect(target, facing, blocks),
            CBlock::Comparator(v) => v.connect(target, facing, blocks),
            CBlock::Air => {}
        }
    }
}
