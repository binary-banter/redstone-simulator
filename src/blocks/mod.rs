use crate::blocks::comparator::{CComparator, Comparator};
use crate::blocks::facing::Facing;
use crate::blocks::probe::CProbe;
use crate::blocks::redstone::{CRedstone, Redstone};
use crate::blocks::redstone_block::CRedstoneBlock;
use crate::blocks::repeater::{CRepeater, Repeater};
use crate::blocks::solid::{CSolidStrong, CSolidWeak};
use crate::blocks::torch::{CTorch, Torch};
use crate::blocks::trigger::CTrigger;
use crate::world::RedGraph;
use once_cell::sync::Lazy;
use petgraph::stable_graph::NodeIndex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;

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
    SolidWeak(CSolidWeak),
    SolidStrong(CSolidStrong),
    Trigger(CTrigger),
    Probe(CProbe),
    Repeater(CRepeater),
    RedstoneBlock(CRedstoneBlock),
    Torch(CTorch),
    Comparator(CComparator),
}

#[derive(Debug, Clone, Copy)]
pub enum Edge {
    Rear(u8),
    Side(u8),
}

impl Add<&Edge> for Edge {
    type Output = Self;

    fn add(self, rhs: &Edge) -> Self::Output {
        match (self, rhs) {
            (Edge::Rear(s1), Edge::Side(s2)) => Edge::Side(s1 + s2),
            (Edge::Rear(s1), Edge::Rear(s2)) => Edge::Rear(s1 + s2),
            _ => unreachable!(),
        }
    }
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

impl Block {
    fn locking_power(&self) -> u8 {
        match self {
            Block::Repeater(v) => v.locking_power(),
            _ => self.output_power(),
        }
    }
}


pub trait BlockConnections {
    fn can_output(&self, facing: Facing) -> Option<NodeIndex>;

    fn can_input(&self, facing: Facing) -> (Option<NodeIndex>, bool);

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, add_probe: &mut F, add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex);
}

fn can_connect(source: &CBlock, target: &CBlock, facing: Facing) -> bool {
    #[rustfmt::skip]
    return match (source, target) {
        (CBlock::Redstone(_), CBlock::Redstone(_)) => true,
        (CBlock::Redstone(_), CBlock::SolidWeak(_)) => true,
        (CBlock::Redstone(_), CBlock::Probe(_)) => true,
        (CBlock::Redstone(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::Redstone(_), CBlock::Comparator(_)) => true,

        (CBlock::Trigger(_), CBlock::Redstone(_)) => true,
        (CBlock::Trigger(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::Trigger(_), CBlock::Torch(_)) => true,
        (CBlock::Trigger(_), CBlock::Comparator(v)) if facing == v.facing().rev() => true,

        (CBlock::SolidStrong(_), CBlock::Redstone(_)) => true,
        (CBlock::SolidWeak(_) | CBlock::SolidStrong(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::SolidWeak(_) | CBlock::SolidStrong(_), CBlock::Torch(_)) => true,
        (CBlock::SolidWeak(_) | CBlock::SolidStrong(_), CBlock::Comparator(v)) if facing == v.facing().rev() => true,

        (CBlock::Repeater(_), CBlock::Redstone(_)) => true,
        (CBlock::Repeater(_), CBlock::SolidStrong(_)) => true,
        (CBlock::Repeater(_), CBlock::Probe(_)) => true,
        (CBlock::Repeater(_), CBlock::Repeater(_)) => true,
        (CBlock::Repeater(_), CBlock::Comparator(_)) => true,

        (CBlock::RedstoneBlock(_), CBlock::Redstone(_)) => true,
        (CBlock::RedstoneBlock(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::RedstoneBlock(_), CBlock::Torch(_)) => true,
        (CBlock::RedstoneBlock(_), CBlock::Comparator(_)) => true,

        (CBlock::Torch(_), CBlock::Redstone(_)) => true,
        (CBlock::Torch(_), CBlock::SolidStrong(_)) if facing == Facing::Up => true,
        (CBlock::Torch(_), CBlock::Probe(_)) if facing == Facing::Up => true,
        (CBlock::Torch(_), CBlock::Repeater(v)) if facing == v.facing().rev() => true,
        (CBlock::Torch(_), CBlock::Comparator(v)) if facing == v.facing().rev() => true,

        (CBlock::Comparator(_), CBlock::Redstone(_)) => true,
        (CBlock::Comparator(_), CBlock::SolidStrong(_)) => true,
        (CBlock::Comparator(_), CBlock::Probe(_)) => true,
        (CBlock::Comparator(_), CBlock::Repeater(_)) => true,
        (CBlock::Comparator(_), CBlock::Comparator(_)) => true,

        _ => false,
    };
}

impl BlockConnections for CBlock {
    fn can_output(&self, facing: Facing) -> Option<NodeIndex> {
        match self {
            CBlock::Redstone(v) => v.can_output(facing),
            CBlock::SolidWeak(v) => v.can_output(facing),
            CBlock::SolidStrong(v) => v.can_output(facing),
            CBlock::Trigger(v) => v.can_output(facing),
            CBlock::Probe(_) => None,
            CBlock::Repeater(v) => v.can_output(facing),
            CBlock::RedstoneBlock(v) => v.can_output(facing),
            CBlock::Torch(v) => v.can_output(facing),
            CBlock::Comparator(v) => v.can_output(facing),
        }
    }

    fn can_input(&self, facing: Facing) -> (Option<NodeIndex>, bool) {
        match self {
            CBlock::Redstone(v) => v.can_input(facing),
            CBlock::SolidWeak(v) => v.can_input(facing),
            CBlock::SolidStrong(v) => v.can_input(facing),
            CBlock::Trigger(_) => (None, false),
            CBlock::Probe(v) => v.can_input(facing),
            CBlock::Repeater(v) => v.can_input(facing),
            CBlock::RedstoneBlock(_) => (None, false),
            CBlock::Torch(v) => v.can_input(facing),
            CBlock::Comparator(v) => v.can_input(facing),
        }
    }

    fn add_node<F, G>(&mut self, blocks: &mut RedGraph, add_probe: &mut F, add_trigger: &mut G)
    where
        F: FnMut(NodeIndex),
        G: FnMut(NodeIndex),
    {
        match self {
            CBlock::Redstone(v) => v.add_node(blocks, add_probe, add_trigger),
            CBlock::SolidWeak(v) => v.add_node(blocks, add_probe, add_trigger),
            CBlock::SolidStrong(v) => v.add_node(blocks, add_probe, add_trigger),
            CBlock::Trigger(v) => v.add_node(blocks, add_probe, add_trigger),
            CBlock::Probe(v) => v.add_node(blocks, add_probe, add_trigger),
            CBlock::Repeater(v) => v.add_node(blocks, add_probe, add_trigger),
            CBlock::RedstoneBlock(v) => v.add_node(blocks, add_probe, add_trigger),
            CBlock::Torch(v) => v.add_node(blocks, add_probe, add_trigger),
            CBlock::Comparator(v) => v.add_node(blocks, add_probe, add_trigger),
        }
    }
}

impl CBlock {
    pub(crate) fn from_id(id: &str) -> Vec<Self> {
        let (id, meta) = id
            .split_once('[')
            .map_or((id, ""), |(x, y)| (x, y.trim_end_matches(']')));

        let meta = meta
            .split(',')
            .filter(|v| !v.is_empty())
            .map(|key_value| key_value.split_once('=').unwrap())
            .collect::<HashMap<&str, &str>>();

        match id {
            "minecraft:redstone_wire" => vec![CBlock::Redstone(CRedstone::from(meta))],
            "minecraft:gold_block" => vec![CBlock::Trigger(CTrigger::default())],
            "minecraft:lightning_rod" => vec![CBlock::Trigger(CTrigger::default())],
            "minecraft:diamond_block" => vec![CBlock::Probe(CProbe::default())],
            "minecraft:redstone_block" => vec![CBlock::RedstoneBlock(CRedstoneBlock::default())],
            "minecraft:redstone_torch" => vec![CBlock::Torch(CTorch::from(meta))],
            "minecraft:redstone_wall_torch" => vec![CBlock::Torch(CTorch::from(meta))],
            "minecraft:comparator" => vec![CBlock::Comparator(CComparator::from(meta))],
            "minecraft:repeater" => vec![CBlock::Repeater(CRepeater::from(meta))],
            id if SOLID_BLOCKS.contains(id) => vec![
                CBlock::SolidWeak(CSolidWeak::default()),
                CBlock::SolidStrong(CSolidStrong::default()),
            ],
            id if TRANSPARENT_BLOCKS.contains(id) => vec![],
            _ => panic!("Undefined block with id {id}."),
        }
    }
}

impl CBlock {
    pub fn is_transparent(&self) -> bool {
        match self {
            CBlock::SolidWeak { .. } => false,
            CBlock::SolidStrong { .. } => false,
            CBlock::Redstone { .. } => true,
            CBlock::RedstoneBlock { .. } => false,
            CBlock::Trigger { .. } => false,
            CBlock::Repeater { .. } => true,
            CBlock::Comparator { .. } => true,
            CBlock::Torch { .. } => true,
            CBlock::Probe { .. } => false,
        }
    }

    pub fn add_edge(&self, target: &CBlock, facing: Facing, blocks: &mut RedGraph) {
        let Some(idx) = self.can_output(facing) else {
            return;
        };

        let (Some(n_idx), alternate) = target.can_input(facing) else {
            return;
        };

        if !can_connect(self, target, facing) {
            return;
        }

        let redstone_connection =
            matches!(self, CBlock::Redstone(_)) && matches!(target, CBlock::Redstone(_));

        let weight = match (alternate, redstone_connection) {
            (true, _) => Edge::Side(0),
            (false, false) => Edge::Rear(0),
            (false, true) => Edge::Rear(1),
        };

        blocks.add_edge(idx, n_idx, weight);
    }
}

pub trait Updatable {
    fn update(
        &mut self,
        idx: NodeIndex,
        tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &mut RedGraph,
    ) -> bool;

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut VecDeque<NodeIndex>,
        blocks: &mut RedGraph,
    );
}

impl Updatable for Block {
    fn update(
        &mut self,
        idx: NodeIndex,
        tick_updatable: &mut VecDeque<NodeIndex>,
        blocks: &mut RedGraph,
    ) -> bool {
        match self {
            Block::Redstone(v) => v.update(idx, tick_updatable, blocks),
            Block::Repeater(v) => v.update(idx, tick_updatable, blocks),
            Block::RedstoneBlock => false,
            Block::Torch(v) => v.update(idx, tick_updatable, blocks),
            Block::Comparator(v) => v.update(idx, tick_updatable, blocks),
        }
    }

    fn late_updatable(
        &mut self,
        idx: NodeIndex,
        updatable: &mut VecDeque<NodeIndex>,
        blocks: &mut RedGraph,
    ) {
        match self {
            Block::Redstone(_) => {}
            Block::Repeater(v) => v.late_updatable(idx, updatable, blocks),
            Block::RedstoneBlock => {}
            Block::Torch(v) => v.late_updatable(idx, updatable, blocks),
            Block::Comparator(v) => v.late_updatable(idx, updatable, blocks),
        }
    }
}

pub fn redstone_min() -> Block {
    Block::Redstone(Redstone::default())
}

pub fn redstone_max() -> Block {
    Block::Redstone(Redstone::with_signal(15))
}
