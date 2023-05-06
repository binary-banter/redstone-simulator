use crate::blocks::comparator::{Comparator, ComparatorMode};
use crate::blocks::facing::Facing;
use crate::blocks::redstone::{Connections, Redstone};
use crate::blocks::repeater::Repeater;
use crate::blocks::solid::{Solid, SolidPower};
use crate::blocks::torch::Torch;
use crate::blocks::trigger::Trigger;
use crate::world_data::WorldData;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

pub mod comparator;
pub mod facing;
pub mod redstone;
pub mod repeater;
pub mod solid;
pub mod torch;
pub mod trigger;

static SOLID_BLOCKS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| include_str!("../../resources/solid.txt").lines().collect());
static TRANSPARENT_BLOCKS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    include_str!("../../resources/transparent.txt")
        .lines()
        .collect()
});

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Solid(Solid),
    Redstone(Redstone),
    RedstoneBlock,
    Trigger(Trigger),
    Repeater(Repeater),
    Comparator(Comparator),
    Torch(Torch),
    Air,
}

pub trait BlockTrait {
    /// Updates the block using its neighbours.
    /// Returns the neighbours that need to be updated and whether the block needs to be updated next tick.
    fn update(
        &mut self,
        pos: (usize, usize, usize),
        world: &WorldData,
        updates: &mut Vec<(usize, usize, usize)>,
    ) -> bool;

    // fn signal(&self, b: &Block, f: Facing) -> u8;
    //
    // fn output_signal(&self, f: Facing) -> u8;
}

pub trait BlockTraitLate {
    /// Updates after the game tick
    fn update_late(
        &mut self,
        pos: (usize, usize, usize),
        world: &WorldData,
        updates: &mut Vec<(usize, usize, usize)>,
    );
}

impl Block {
    pub fn output_power(&self, f: Facing) -> u8 {
        match self {
            Block::Solid(v) => v.output_signal().into(),
            Block::Redstone(v) => v.output_signal(f),
            Block::RedstoneBlock => 15,
            Block::Trigger(v) => v.output_signal(),
            Block::Repeater(v) => v.output_signal(f),
            Block::Comparator(v) => v.output_signal(f),
            Block::Torch(v) => v.output_signal(f),
            Block::Air => 0,
        }
    }

    pub fn is_transparent(&self) -> bool {
        match self {
            Block::Solid(_) => false,
            Block::Redstone(_) => true,
            Block::RedstoneBlock => false,
            Block::Trigger(_) => false,
            Block::Repeater(_) => true,
            Block::Comparator(_) => true,
            Block::Torch(_) => true,
            Block::Air => true,
        }
    }

    /// Returns (`Block`, `is_trigger`, `is_probe`)
    pub fn from_id(id: &str) -> (Self, bool, bool) {
        let (id, meta) = id
            .split_once('[')
            .map_or((id, ""), |(x, y)| (x, y.trim_end_matches(']')));

        let meta = meta
            .split(',')
            .filter(|v| !v.is_empty())
            .map(|key_value| key_value.split_once('=').unwrap())
            .collect::<HashMap<&str, &str>>();

        match id {
            "minecraft:redstone_wire" => (
                Block::Redstone(Redstone {
                    signal: meta["power"].parse().unwrap(),
                    connections: Connections {
                        north: meta["north"] != "none",
                        east: meta["east"] != "none",
                        south: meta["south"] != "none",
                        west: meta["west"] != "none",
                    },
                }),
                false,
                false,
            ),
            "minecraft:gold_block" | "minecraft:lightning_rod" => {
                (Block::Trigger(Trigger { powered: false }), true, false)
            }
            "minecraft:diamond_block" => (
                Block::Solid(Solid {
                    signal: SolidPower::Weak(0),
                }),
                false,
                true,
            ),
            "minecraft:repeater" => (
                Block::Repeater(Repeater {
                    powered: false,
                    facing: Facing::from(meta["facing"]),
                    count: 0,
                    delay: meta["delay"].parse().unwrap(),
                    next_powered: false,
                }),
                false,
                false,
            ),
            "minecraft:redstone_torch" | "minecraft:redstone_wall_torch" => {
                let s = meta.get("lit").map(|&x| x == "true").unwrap();

                let f = meta
                    .get("facing")
                    .map(|&f| Facing::from(f))
                    .unwrap_or(Facing::Up);

                (
                    Block::Torch(Torch {
                        powered: s,
                        facing: f,
                    }),
                    false,
                    false,
                )
            }
            "minecraft:redstone_block" => (Block::RedstoneBlock, false, false),

            "minecraft:comparator" => (
                Block::Comparator(Comparator {
                    signal: 0,
                    next_signal: 0,
                    facing: Facing::from(meta["facing"]),
                    mode: ComparatorMode::from(meta["mode"]),
                }),
                false,
                false,
            ), //TODO

            id if SOLID_BLOCKS.contains(id) => (
                Block::Solid(Solid {
                    signal: SolidPower::Weak(0),
                }),
                false,
                false,
            ),
            id if TRANSPARENT_BLOCKS.contains(id) => (Block::Air, false, false),
            _ => todo!("Unimplemented identifier: {id}, with meta: {meta:?}."),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Solid(Solid { signal: s }) => match s {
                SolidPower::Weak(0) | SolidPower::Strong(0) => write!(f, "□"),
                SolidPower::Weak(_) => write!(f, "■"),
                SolidPower::Strong(_) => write!(f, "▣"),
            },
            Block::Redstone(Redstone { signal: s, .. }) => {
                write!(
                    f,
                    "{}",
                    "0123456789ABCDEF".chars().nth(*s as usize).unwrap()
                )
            }
            Block::RedstoneBlock => write!(f, "R"),
            Block::Air => write!(f, " "),
            Block::Trigger(Trigger { .. }) => write!(f, "T"),
            Block::Repeater(Repeater {
                facing: Facing::North,
                ..
            }) => write!(f, "v"),
            Block::Repeater(Repeater {
                facing: Facing::East,
                ..
            }) => write!(f, "<"),
            Block::Repeater(Repeater {
                facing: Facing::South,
                ..
            }) => write!(f, "^"),
            Block::Repeater(Repeater {
                facing: Facing::West,
                ..
            }) => write!(f, ">"),
            Block::Repeater(Repeater { .. }) => unreachable!(),
            Block::Comparator(Comparator { .. }) => write!(f, "-"),
            Block::Torch(v) if !v.powered => write!(f, "*"),
            Block::Torch(_) => write!(f, "+"),
        }
    }
}
