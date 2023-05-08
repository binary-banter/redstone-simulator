// use crate::facing::Facing;
// use once_cell::sync::Lazy;
// use petgraph::prelude::NodeIndex;
// use std::collections::{HashMap, HashSet};
// use std::ops::Index;
//
// static SOLID_BLOCKS: Lazy<HashSet<&'static str>> =
//     Lazy::new(|| include_str!("../resources/solid.txt").lines().collect());
// static TRANSPARENT_BLOCKS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
//     include_str!("../resources/transparent.txt")
//         .lines()
//         .collect()
// });
//
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Connections {
//     pub north: bool,
//     pub east: bool,
//     pub south: bool,
//     pub west: bool,
// }
//
// impl Index<Facing> for Connections {
//     type Output = bool;
//
//     fn index(&self, index: Facing) -> &Self::Output {
//         match index {
//             Facing::North => &self.north,
//             Facing::East => &self.east,
//             Facing::South => &self.south,
//             Facing::West => &self.west,
//             Facing::Up => &false,
//             Facing::Down => &true,
//         }
//     }
// }
//
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum CBlock {
//     Redstone {
//         signal: u8,
//         connections: Connections,
//         node: Option<NodeIndex>,
//     },
//     Solid {
//         weak: Option<NodeIndex>,
//         strong: Option<NodeIndex>,
//     },
//     Trigger {
//         node: Option<NodeIndex>,
//     },
//     Probe {
//         node: Option<NodeIndex>,
//     },
//     Repeater {
//         powered: bool,
//         delay: u8,
//         node: Option<NodeIndex>,
//         facing: Facing,
//     },
//     RedstoneBlock {
//         node: Option<NodeIndex>,
//     },
//     Torch {
//         lit: bool,
//         facing: Facing,
//         node: Option<NodeIndex>,
//     },
//     Comparator {
//         signal: u8,
//         facing: Facing,
//         mode: ComparatorMode,
//         node: Option<NodeIndex>,
//     },
//     Air,
// }
//
// impl CBlock {
//     pub fn is_transparent(&self) -> bool {
//         match self {
//             CBlock::Solid { .. } => false,
//             CBlock::Redstone { .. } => true,
//             CBlock::RedstoneBlock { .. } => false,
//             CBlock::Trigger { .. } => false,
//             CBlock::Repeater { .. } => true,
//             CBlock::Comparator { .. } => true,
//             CBlock::Torch { .. } => true,
//             CBlock::Air => true,
//             CBlock::Probe { .. } => false,
//         }
//     }
//
//     /// Returns (`Block`, `is_trigger`, `is_probe`)
//     pub fn from_id(id: &str) -> Self {
//         let (id, meta) = id
//             .split_once('[')
//             .map_or((id, ""), |(x, y)| (x, y.trim_end_matches(']')));
//
//         let meta = meta
//             .split(',')
//             .filter(|v| !v.is_empty())
//             .map(|key_value| key_value.split_once('=').unwrap())
//             .collect::<HashMap<&str, &str>>();
//
//         match id {
//             "minecraft:redstone_wire" => CBlock::Redstone {
//                 signal: meta["power"].parse().unwrap(),
//                 connections: Connections {
//                     north: meta["north"] != "none",
//                     east: meta["east"] != "none",
//                     south: meta["south"] != "none",
//                     west: meta["west"] != "none",
//                 },
//                 node: None,
//             },
//             "minecraft:gold_block" | "minecraft:lightning_rod" => CBlock::Trigger { node: None },
//             "minecraft:diamond_block" => CBlock::Probe { node: None },
//             "minecraft:redstone_block" => CBlock::RedstoneBlock { node: None },
//             "minecraft:redstone_torch" | "minecraft:redstone_wall_torch" => {
//                 let s = meta.get("lit").map(|&x| x == "true").unwrap();
//
//                 let f = meta
//                     .get("facing")
//                     .map(|&f| Facing::from(f))
//                     .unwrap_or(Facing::Up);
//
//                 CBlock::Torch {
//                     lit: s,
//                     facing: f,
//                     node: None,
//                 }
//             }
//             "minecraft:comparator" => CBlock::Comparator {
//                 signal: 0,
//                 facing: Facing::from(meta["facing"]),
//                 mode: ComparatorMode::from(meta["mode"]),
//                 node: None,
//             },
//             "minecraft:repeater" => CBlock::Repeater {
//                 powered: false,
//                 facing: Facing::from(meta["facing"]),
//                 delay: meta["delay"].parse().unwrap(),
//                 node: None,
//             },
//             id if SOLID_BLOCKS.contains(id) => CBlock::Solid {
//                 weak: None,
//                 strong: None,
//             },
//             id if TRANSPARENT_BLOCKS.contains(id) => CBlock::Air,
//
//             _ => panic!("Undefined block with id {id}."),
//         }
//     }
// }
