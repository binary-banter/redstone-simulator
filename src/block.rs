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
    /// Solid block can be 0 (off), 1 (powered) or 16 (repeater powered)
    Solid(u8),

    /// Redstone ranges from 0 to 15 inclusive
    Redstone(u8),

    Air,

    /// Trigger
    Trigger(bool),

    /// Facing = Direction of INPUT of repeater
    Repeater(bool, Facing), // delay: 4, count: 1

                            // minecraft:repeater, with meta: "delay=1,facing=north,locked=false,powered=false"
}

impl Block {
    /// Returns (Block, is_trigger, is_probe)
    pub fn from_id(id: &str) -> (Self, bool, bool) {
        let (id, meta) = id
            .split_once('[')
            .map(|(x, y)| (x, y.trim_end_matches(']')))
            .unwrap_or((id, ""));

        let meta = meta
            .split(',')
            .filter(|v| !v.is_empty())
            .map(|key_value| key_value.split_once('=').unwrap())
            .collect::<HashMap<&str, &str>>();

        match id {
            "minecraft:redstone_wire" => (Block::Redstone(0), false, false),
            "minecraft:air" => (Block::Air, false, false),
            "minecraft:stone" => (Block::Solid(0), false, false),
            "minecraft:gold_block" => (Block::Trigger(false), true, false),
            "minecraft:diamond_block" => (Block::Solid(0), false, true),
            "minecraft:repeater" => match *meta.get("facing").unwrap() {
                "north" => (Block::Repeater(false, Facing::North), false, false),
                "east" => (Block::Repeater(false, Facing::East), false, false),
                "south" => (Block::Repeater(false, Facing::South), false, false),
                "west" => (Block::Repeater(false, Facing::West), false, false),
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
            Block::Solid(i) => match *i {
                0 => write!(f, "□"),
                1 => write!(f, "■"),
                16 => write!(f, "▣"),
                _ => unreachable!(),
            },
            Block::Redstone(i) => write!(
                f,
                "{}",
                "0123456789ABCDEF".chars().nth(*i as usize).unwrap()
            ),
            Block::Air => write!(f, " "),
            Block::Trigger(_) => write!(f, "T"),
            Block::Repeater(_, Facing::North) => write!(f, "v"),
            Block::Repeater(_, Facing::East) => write!(f, "<"),
            Block::Repeater(_, Facing::South) => write!(f, "^"),
            Block::Repeater(_, Facing::West) => write!(f, ">"),
        }
    }
}
