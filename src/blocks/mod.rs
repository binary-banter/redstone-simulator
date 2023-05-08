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
use once_cell::sync::Lazy;
use petgraph::stable_graph::StableGraph;
use petgraph::Directed;
use std::collections::{HashMap, HashSet};

mod comparator;
pub mod facing;
mod probe;
mod redstone;
mod redstone_block;
mod repeater;
mod solid;
mod torch;
mod trigger;

static SOLID_BLOCKS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| include_str!("../../resources/solid.txt").lines().collect());
static TRANSPARENT_BLOCKS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    include_str!("../../resources/transparent.txt")
        .lines()
        .collect()
});

/// Blocks that end up in the graph structure of the world.
#[derive(Clone, Debug)]
pub enum Block {
    Redstone(Redstone),
    Repeater(Repeater),
    RedstoneBlock,
    Torch(Torch),
    Comparator(Comparator),
}

/// Blocks used during the creation of the graph structure of the world.
#[derive(Copy, Clone, Debug)]
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

pub trait OutputPower {
    fn output_power(&self) -> u8;
}

impl OutputPower for Block {
    fn output_power(&self) -> u8 {
        match self {
            Block::Redstone(v) => v.output_power(),
            Block::Repeater(v) => v.output_power(),
            Block::RedstoneBlock => 15,
            Block::Torch(v) => v.output_power(),
            Block::Comparator(v) => v.output_power(),
        }
    }
}

pub trait BlockConnections {
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

impl From<&str> for CBlock {
    fn from(id: &str) -> Self {
        let (id, meta) = id
            .split_once('[')
            .map_or((id, ""), |(x, y)| (x, y.trim_end_matches(']')));

        let meta = meta
            .split(',')
            .filter(|v| !v.is_empty())
            .map(|key_value| key_value.split_once('=').unwrap())
            .collect::<HashMap<&str, &str>>();

        match id {
            "minecraft:redstone_wire" => CBlock::Redstone(CRedstone::from(meta)),
            "minecraft:gold_block" => CBlock::Trigger(CTrigger::default()),
            "minecraft:lightning_rod" => CBlock::Trigger(CTrigger::default()),
            "minecraft:diamond_block" => CBlock::Probe(CProbe::default()),
            "minecraft:redstone_block" => CBlock::RedstoneBlock(CRedstoneBlock::default()),
            "minecraft:redstone_torch" => CBlock::Torch(CTorch::from(meta)),
            "minecraft:redstone_wall_torch" => CBlock::Torch(CTorch::from(meta)),
            "minecraft:comparator" => CBlock::Comparator(CComparator::from(meta)),
            "minecraft:repeater" => CBlock::Repeater(CRepeater::from(meta)),
            id if SOLID_BLOCKS.contains(id) => CBlock::Solid(CSolid::default()),
            id if TRANSPARENT_BLOCKS.contains(id) => CBlock::Air,
            _ => panic!("Undefined block with id {id}."),
        }
    }
}

impl CBlock {
    pub fn is_transparent(&self) -> bool {
        match self {
            CBlock::Solid { .. } => false,
            CBlock::Redstone { .. } => true,
            CBlock::RedstoneBlock { .. } => false,
            CBlock::Trigger { .. } => false,
            CBlock::Repeater { .. } => true,
            CBlock::Comparator { .. } => true,
            CBlock::Torch { .. } => true,
            CBlock::Air => true,
            CBlock::Probe { .. } => false,
        }
    }
}
