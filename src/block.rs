use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Block {
    Solid(u8),
    Redstone(u8),
    Air,
    Trigger(bool),
}

impl Block {
    /// Returns (Block, is_trigger, is_probe)
    pub fn from_id(id: &str) -> (Self, bool, bool) {
        // minecraft:redstone_wire[east=none,north=side,power=0,south=side,west=none]
        let (id, _meta) = id
            .split_once('[')
            .map(|(x, y)| (x, y.trim_end_matches(']')))
            .unwrap_or((id, ""));

        match id {
            "minecraft:redstone_wire" => (Block::Redstone(0), false, false),
            "minecraft:air" => (Block::Air, false, false),
            "minecraft:stone" => (Block::Solid(0), false, false),
            "minecraft:gold_block" => (Block::Trigger(false), true, false),
            "minecraft:diamond_block" => (Block::Solid(0), false, true),
            _ => todo!("Unimplemented identifier: {id}."),
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
        }
    }
}
