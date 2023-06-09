use crate::blocks::comparator::{CComparator, Comparator};
use crate::blocks::facing::Facing;
use crate::blocks::probe::CProbe;
use crate::blocks::redstone::{CRedstone, Redstone};
use crate::blocks::redstone_block::CRedstoneBlock;
use crate::blocks::repeater::{CRepeater, Repeater};
use crate::blocks::solid::{CSolidStrong, CSolidWeak};
use crate::blocks::srepeater::{CSRepeater, SRepeater};
use crate::blocks::torch::CTorch;
use crate::blocks::trigger::CTrigger;
use crate::world::edge::Edge;
use crate::world::graph::GNode;
use crate::world::UpdatableList;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

pub mod comparator;
pub mod facing;
pub mod probe;
pub mod redstone;
pub mod redstone_block;
pub mod repeater;
pub mod solid;
pub mod srepeater;
pub mod torch;
pub mod trigger;

static SOLID_BLOCKS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| include_str!("../../resources/solid.txt").lines().collect());
static TRANSPARENT_BLOCKS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    include_str!("../../resources/transparent.txt")
        .lines()
        .collect()
});

/// Blocks that end up in the graph structure of the world.
#[derive(Debug)]
pub enum Block {
    Redstone(Redstone),
    Repeater(Repeater),
    Comparator(Comparator),
    SRepeater(SRepeater),
}

/// Blocks used during the creation of the graph structure of the world.
#[derive(Clone, Debug)]
pub enum CBlock {
    Redstone(CRedstone),
    SolidWeak(CSolidWeak),
    SolidStrong(CSolidStrong),
    Trigger(CTrigger),
    Probe(CProbe),
    Repeater(CRepeater),
    SRepeater(CSRepeater),
    RedstoneBlock(CRedstoneBlock),
    Torch(CTorch),
    Comparator(CComparator),
}

pub trait OutputPower {
    fn output_power(&self) -> u8;
}

impl OutputPower for Block {
    fn output_power(&self) -> u8 {
        match self {
            Block::Redstone(v) => v.output_power(),
            Block::Repeater(v) => v.output_power(),
            Block::Comparator(v) => v.output_power(),
            Block::SRepeater(v) => v.output_power(),
        }
    }
}

impl OutputPower for CBlock {
    fn output_power(&self) -> u8 {
        match self {
            CBlock::Redstone(_) => unreachable!(),
            CBlock::Repeater(v) => v.output_power(),
            CBlock::Torch(v) => v.output_power(),
            CBlock::Comparator(v) => v.output_power(),
            CBlock::SRepeater(v) => v.output_power(),
            CBlock::SolidWeak(_) => unreachable!(),
            CBlock::SolidStrong(_) => unreachable!(),
            CBlock::Trigger(_) => 0,
            CBlock::Probe(_) => unreachable!(),
            CBlock::RedstoneBlock(_) => 15,
        }
    }
}

impl Block {
    fn will_lock(&self) -> bool {
        match self {
            Block::Repeater(v) => v.will_lock(),
            Block::Comparator(v) => v.output_power() > 0,
            _ => unreachable!(),
        }
    }
}

pub enum InputSide {
    Rear,
    Side,
}

impl InputSide {
    pub fn to_edge(&self, v: u8) -> Edge {
        match self {
            InputSide::Rear => Edge::Rear(v),
            InputSide::Side => Edge::Side(v),
        }
    }
}

pub trait BlockConnections {
    fn can_output(&self, facing: Facing) -> bool;

    fn can_input(&self, facing: Facing) -> Option<InputSide>;
}

pub trait ToBlock {
    fn to_block(&self, on_inputs: u8) -> Block;
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
    fn can_output(&self, facing: Facing) -> bool {
        match self {
            CBlock::Redstone(v) => v.can_output(facing),
            CBlock::SolidWeak(v) => v.can_output(facing),
            CBlock::SolidStrong(v) => v.can_output(facing),
            CBlock::Trigger(v) => v.can_output(facing),
            CBlock::Probe(v) => v.can_output(facing),
            CBlock::Repeater(v) => v.can_output(facing),
            CBlock::RedstoneBlock(v) => v.can_output(facing),
            CBlock::Torch(v) => v.can_output(facing),
            CBlock::Comparator(v) => v.can_output(facing),
            CBlock::SRepeater(_) => unreachable!(),
        }
    }

    fn can_input(&self, facing: Facing) -> Option<InputSide> {
        match self {
            CBlock::Redstone(v) => v.can_input(facing),
            CBlock::SolidWeak(v) => v.can_input(facing),
            CBlock::SolidStrong(v) => v.can_input(facing),
            CBlock::Trigger(v) => v.can_input(facing),
            CBlock::Probe(v) => v.can_input(facing),
            CBlock::Repeater(v) => v.can_input(facing),
            CBlock::RedstoneBlock(v) => v.can_input(facing),
            CBlock::Torch(v) => v.can_input(facing),
            CBlock::Comparator(v) => v.can_input(facing),
            CBlock::SRepeater(_) => unreachable!(),
        }
    }
}
impl ToBlock for CBlock {
    fn to_block(&self, on_inputs: u8) -> Block {
        match self {
            CBlock::Redstone(v) => v.to_block(on_inputs),
            CBlock::SolidWeak(v) => v.to_block(on_inputs),
            CBlock::SolidStrong(v) => v.to_block(on_inputs),
            CBlock::Trigger(v) => v.to_block(on_inputs),
            CBlock::Probe(v) => v.to_block(on_inputs),
            CBlock::Repeater(v) => v.to_block(on_inputs),
            CBlock::RedstoneBlock(v) => v.to_block(on_inputs),
            CBlock::Torch(v) => v.to_block(on_inputs),
            CBlock::Comparator(v) => v.to_block(on_inputs),
            CBlock::SRepeater(v) => v.to_block(on_inputs),
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
            CBlock::SRepeater { .. } => unreachable!(),
        }
    }

    pub fn get_edge(&self, target: &CBlock, facing: Facing) -> Option<Edge> {
        if matches!(self, CBlock::Redstone(_)) && matches!(target, CBlock::Redstone(_)) {
            return Some(Edge::Rear(1));
        }

        if !self.can_output(facing) {
            return None;
        }

        let Some(input_side) = target.can_input(facing) else {
            return None;
        };

        if !can_connect(self, target, facing) {
            return None;
        }

        Some(input_side.to_edge(0))
    }
}

pub trait Updatable {
    fn update(
        &self,
        idx: &'static GNode<Block, u8>,
        tick_updatable: &mut UpdatableList,
        up: bool,
    ) -> bool;

    fn late_update(
        &self,
        idx: &'static GNode<Block, u8>,
        tick_updatable: &mut UpdatableList,
        tick_counter: usize,
    ) -> Option<(u8, u8)>;
}

impl Updatable for Block {
    #[inline(always)]
    fn update(
        &self,
        idx: &'static GNode<Block, u8>,
        tick_updatable: &mut UpdatableList,
        up: bool,
    ) -> bool {
        match self {
            Block::Repeater(v) => v.update(idx, tick_updatable, up),
            Block::Comparator(v) => v.update(idx, tick_updatable, up),
            Block::Redstone(v) => v.update(idx, tick_updatable, up),
            Block::SRepeater(v) => v.update(idx, tick_updatable, up),
        }
    }

    fn late_update(
        &self,
        idx: &'static GNode<Block, u8>,
        tick_updatable: &mut UpdatableList,
        tick_counter: usize,
    ) -> Option<(u8, u8)> {
        match self {
            Block::Repeater(v) => v.late_update(idx, tick_updatable, tick_counter),
            Block::Comparator(v) => v.late_update(idx, tick_updatable, tick_counter),
            Block::Redstone(_) => unreachable!(),
            Block::SRepeater(v) => v.late_update(idx, tick_updatable, tick_counter),
        }
    }
}
