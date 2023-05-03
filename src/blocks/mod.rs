use crate::blocks::air::Air;
use crate::blocks::facing::Facing;
use crate::blocks::redstone::Redstone;
use crate::blocks::repeater::Repeater;
use crate::blocks::solid::Solid;
use crate::blocks::trigger::Trigger;
use crate::world_data::WorldData;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub mod air;
pub mod facing;
pub mod redstone;
pub mod repeater;
pub mod solid;
pub mod trigger;

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Solid(Solid),
    Redstone(Redstone),
    Trigger(Trigger),
    Repeater(Repeater),
    Air(Air),
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

impl Block {
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
            "minecraft:redstone_wire" => (Block::Redstone(Redstone { signal: 0 }), false, false),
            "minecraft:air" => (Block::Air(Air {}), false, false),
            "minecraft:stone" => (Block::Solid(Solid { signal: 0 }), false, false),
            "minecraft:gold_block" => (Block::Trigger(Trigger { signal: 0 }), true, false),
            "minecraft:diamond_block" => (Block::Solid(Solid { signal: 0 }), false, true),
            "minecraft:repeater" => match *meta.get("facing").unwrap() {
                "north" => (
                    Block::Repeater(Repeater {
                        signal: 0,
                        facing: Facing::North,
                        count: 0,
                        delay: 1,
                        next_signal: 0,
                    }),
                    false,
                    false,
                ),
                "east" => (
                    Block::Repeater(Repeater {
                        signal: 0,
                        facing: Facing::East,
                        count: 0,
                        delay: 1,
                        next_signal: 0,
                    }),
                    false,
                    false,
                ),
                "south" => (
                    Block::Repeater(Repeater {
                        signal: 0,
                        facing: Facing::South,
                        count: 0,
                        delay: 1,
                        next_signal: 0,
                    }),
                    false,
                    false,
                ),
                "west" => (
                    Block::Repeater(Repeater {
                        signal: 0,
                        facing: Facing::West,
                        count: 0,
                        delay: 1,
                        next_signal: 0,
                    }),
                    false,
                    false,
                ),
                _ => unreachable!(),
            },
            "minecraft:oak_wall_sign" => (Block::Air(Air {}), false, false),
            "minecraft:oak_sign" => (Block::Air(Air {}), false, false),
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
            Block::Redstone(Redstone { signal: s }) => {
                write!(
                    f,
                    "{}",
                    "0123456789ABCDEF".chars().nth(*s as usize).unwrap()
                )
            }
            Block::Air(Air {}) => write!(f, " "),
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
        }
    }
}
