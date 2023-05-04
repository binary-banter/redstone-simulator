use crate::blocks::facing::Facing;
use crate::blocks::redstone::{Connections, Redstone};
use crate::blocks::repeater::Repeater;
use crate::blocks::solid::Solid;
use crate::blocks::torch::Torch;
use crate::blocks::trigger::Trigger;
use crate::world_data::WorldData;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use once_cell::sync::Lazy;

pub mod facing;
pub mod redstone;
pub mod repeater;
pub mod solid;
pub mod torch;
pub mod trigger;

static SOLID_BLOCKS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    include_str!("../../resources/solid.txt").lines().collect()
});
static TRANSPARENT_BLOCKS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    include_str!("../../resources/transparent.txt").lines().collect()
});

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Solid(Solid),
    Redstone(Redstone),
    RedstoneBlock,
    Trigger(Trigger),
    Repeater(Repeater),
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
    ) -> (Vec<(usize, usize, usize)>, bool);
}

pub trait BlockTraitLate {
    /// Updates after the game tick
    fn update_late(
        &mut self,
        pos: (usize, usize, usize),
        world: &WorldData,
    ) -> Vec<(usize, usize, usize)>;
}

impl Block {
    /// Facing is from the perspective of the updated block, not the powering block
    /// So this is the weak power delivered TO the reverse of f.
    fn weak_power_dir(&self, f: Facing) -> u8 {
        match self {
            Block::Solid(v) => v.signal,
            Block::Redstone(v) if v.connections[f.reverse()] => v.signal,
            Block::Redstone(_) => 0,
            Block::RedstoneBlock => 16,
            Block::Trigger(v) => v.signal,
            Block::Repeater(v) if v.facing == f => v.signal,
            Block::Repeater(_) => 0,
            Block::Torch(v) if v.facing == f => 0, // Torch does not output where it's hanging
            Block::Torch(v) => v.signal,
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
            "minecraft:gold_block" => (Block::Trigger(Trigger { signal: 0 }), true, false),
            "minecraft:diamond_block" => (Block::Solid(Solid { signal: 0 }), false, true),
            "minecraft:repeater" => (
                Block::Repeater(Repeater {
                    signal: 0,
                    facing: Facing::from(meta["facing"]),
                    count: 0,
                    delay: meta["delay"].parse().unwrap(),
                    next_signal: 0,
                }),
                false,
                false,
            ),
            "minecraft:redstone_torch" | "minecraft:redstone_wall_torch"  => {
                let s = meta
                    .get("lit")
                    .map(|&x| if x == "true" { 16 } else { 0 })
                    .unwrap();

                let f = meta
                    .get("facing")
                    .map(|&f| Facing::from(f))
                    .unwrap_or(Facing::Up);

                (
                    Block::Torch(Torch {
                        signal: s,
                        facing: f,
                        next_signal: s,
                    }),
                    false,
                    false,
                )
            }
            "minecraft:redstone_block" => (Block::RedstoneBlock, false, false),

            "minecraft:comparator" => (Block::Air, false, false), //TODO
            "minecraft:stone_button" => (Block::Air, false, false), //TODO
            "minecraft:lever" => (Block::Air, false, false), //TODO
            "minecraft:lightning_rod" => (Block::Air, false, false), //TODO

            id if SOLID_BLOCKS.contains(id) => (Block::Solid(Solid { signal: 0 }), false, false),
            id if TRANSPARENT_BLOCKS.contains(id) => (Block::Air, false, false),
            _ => todo!("Unimplemented identifier: {id}, with meta: {meta:?}."),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Solid(Solid { signal: s }) => match s {
                0 => write!(f, "□"),
                1 => write!(f, "■"),
                16 => write!(f, "▣"),
                _ => unreachable!(),
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
            Block::Torch(v) if v.signal == 0 => write!(f, "*"),
            Block::Torch(_) => write!(f, "+"),
        }
    }
}
