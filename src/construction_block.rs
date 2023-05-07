use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use petgraph::prelude::NodeIndex;

static SOLID_BLOCKS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| include_str!("../resources/solid.txt").lines().collect());
static TRANSPARENT_BLOCKS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    include_str!("../resources/transparent.txt")
        .lines()
        .collect()
});

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CBlock {
    Redstone { signal: u8, connections: [bool; 4], node: Option<NodeIndex> },
    Solid { weak: Option<NodeIndex>, strong: Option<NodeIndex> },
    Trigger { node: Option<NodeIndex> },
    Probe { node: Option<NodeIndex> },
    Air,
}

impl CBlock {
    /// Returns (`Block`, `is_trigger`, `is_probe`)
    pub fn from_id(id: &str) -> Self {
        let (id, meta) = id
            .split_once('[')
            .map_or((id, ""), |(x, y)| (x, y.trim_end_matches(']')));

        let meta = meta
            .split(',')
            .filter(|v| !v.is_empty())
            .map(|key_value| key_value.split_once('=').unwrap())
            .collect::<HashMap<&str, &str>>();

        match id {
            "minecraft:redstone_wire" => CBlock::Redstone {
                signal: meta["power"].parse().unwrap(),
                connections: [
                    meta["north"] != "none",
                    meta["east"] != "none",
                    meta["south"] != "none",
                    meta["west"] != "none",
                ],
                node: None,
            },
            "minecraft:gold_block" | "minecraft:lightning_rod" => CBlock::Trigger {
                node: None,
            },
            "minecraft:diamond_block" => CBlock::Probe {
                node: None,
            },

            id if SOLID_BLOCKS.contains(id) => CBlock::Solid {
                weak: None,
                strong: None,
            },
            id if TRANSPARENT_BLOCKS.contains(id) => CBlock::Air,

            _ => panic!("Undefined block with id {id}."),
        }
    }
}
