use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    pub fn reverse(self) -> Self {
        match self {
            Facing::North => Facing::South,
            Facing::East => Facing::West,
            Facing::South => Facing::North,
            Facing::West => Facing::East,
        }
    }

    pub fn front(self, x: usize, y: usize, z: usize) -> (usize, usize, usize) {
        match self {
            Facing::North => (x, y, z - 1),
            Facing::East => (x + 1, y, z),
            Facing::South => (x, y, z + 1),
            Facing::West => (x - 1, y, z),
        }
    }

    pub fn back(self, x: usize, y: usize, z: usize) -> (usize, usize, usize) {
        self.reverse().front(x, y, z)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Block {
    Solid {
        /// Can be 0 (off), 1 (powered) or 16 (repeater powered)
        signal: u8,
    },

    Redstone {
        /// Ranges from 0 to 15 inclusive.
        signal: u8,
    },

    Trigger {
        /// Can be 0 (off) or 16 (triggered).
        signal: u8,
    },

    Repeater {
        /// Can be 0 (off) or 16 (powered).
        signal: u8,

        /// This is the direction of the input side.
        facing: Facing,
    },

    Air,
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
            "minecraft:redstone_wire" => (Block::Redstone { signal: 0 }, false, false),
            "minecraft:air" => (Block::Air, false, false),
            "minecraft:stone" => (Block::Solid { signal: 0 }, false, false),
            "minecraft:gold_block" => (Block::Trigger { signal: 0 }, true, false),
            "minecraft:diamond_block" => (Block::Solid { signal: 0 }, false, true),
            "minecraft:repeater" => match *meta.get("facing").unwrap() {
                "north" => (
                    Block::Repeater {
                        signal: 0,
                        facing: Facing::North,
                    },
                    false,
                    false,
                ),
                "east" => (
                    Block::Repeater {
                        signal: 0,
                        facing: Facing::East,
                    },
                    false,
                    false,
                ),
                "south" => (
                    Block::Repeater {
                        signal: 0,
                        facing: Facing::South,
                    },
                    false,
                    false,
                ),
                "west" => (
                    Block::Repeater {
                        signal: 0,
                        facing: Facing::West,
                    },
                    false,
                    false,
                ),
                _ => unreachable!(),
            },
            "minecraft:oak_wall_sign" => (Block::Air, false, false),
            "minecraft:oak_sign" => (Block::Air, false, false),
            _ => todo!("Unimplemented identifier: {id}, with meta: {meta:?}."),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Solid { signal: s } => match s {
                0 => write!(f, "□"),
                1 => write!(f, "■"),
                16 => write!(f, "▣"),
                _ => unreachable!(),
            },
            &Block::Redstone { signal: s } => {
                write!(f, "{}", "0123456789ABCDEF".chars().nth(s as usize).unwrap())
            }
            Block::Air => write!(f, " "),
            Block::Trigger { .. } => write!(f, "T"),
            Block::Repeater {
                facing: Facing::North,
                ..
            } => write!(f, "v"),
            Block::Repeater {
                facing: Facing::East,
                ..
            } => write!(f, "<"),
            Block::Repeater {
                facing: Facing::South,
                ..
            } => write!(f, "^"),
            Block::Repeater {
                facing: Facing::West,
                ..
            } => write!(f, ">"),
        }
    }
}
