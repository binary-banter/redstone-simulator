use crate::blocks::Block;
use crate::blocks::{BlockConnections, CBlock};
use crate::world::data::{neighbours, neighbours_and_facings, TileMap, WorldData};
use crate::world::schematic::SchemFormat;
use crate::world::{BlockGraph, World};
use bimap::BiMap;
use nbt::{from_gzip_reader, Value};
use petgraph::stable_graph::NodeIndex;
use std::collections::VecDeque;
use std::fs::File;

impl From<File> for World {
    fn from(file: File) -> Self {
        World::from(from_gzip_reader::<File, SchemFormat>(file).unwrap())
    }
}

impl From<SchemFormat> for World {
    fn from(format: SchemFormat) -> Self {
        let tile_map: TileMap = format
            .block_entities
            .iter()
            .map(|b| ((b.pos[0] as usize, b.pos[1] as usize, b.pos[2] as usize), b))
            .collect();

        let mut world = WorldData::from_format(&format, &tile_map);
        let mut blocks = BlockGraph::new();

        let mut triggers = Vec::new();
        let mut probes = BiMap::new();

        let get_sign = |p| {
            tile_map.get(&p).and_then(|b| {
                if b.id == "minecraft:sign" {
                    if let Some(Value::String(s)) = b.props.get("Text1") {
                        let j: serde_json::Value = serde_json::from_str(&s).unwrap();
                        Some(
                            j.as_object()
                                .unwrap()
                                .get("text")
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .to_string(),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        };

        // construct nodes
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    let mut add_probe = |idx: NodeIndex| {
                        let name = neighbours((x, y, z))
                            .find_map(get_sign)
                            .unwrap_or(format!("{x},{y},{z}"));

                        probes.insert(idx, name);
                    };

                    let mut add_trigger = |idx: NodeIndex| {
                        triggers.push(idx);
                    };

                    for block in &mut world[(x, y, z)] {
                        block.add_node(&mut blocks, &mut add_probe, &mut add_trigger);
                    }
                }
            }
        }

        // construct edges
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    for block in &world[(x, y, z)] {
                        for (np, f) in neighbours_and_facings((x, y, z)) {
                            for n_block in &world[np] {
                                block.add_edge(n_block, f, &mut blocks);
                            }
                        }

                        // construct vertical edges for redstone
                        if let CBlock::Redstone(v) = block {
                            v.add_vertical_edges((x, y, z), &mut blocks, &world)
                        }
                    }
                }
            }
        }

        let mut world = World {
            blocks,
            triggers,
            probes,
            updatable: VecDeque::new(),
            tick_updatable: VecDeque::new(),
            tick_counter: 0,
        };

        world.prune_graph();

        world.tick_updatable = world
            .blocks
            .node_indices()
            .filter(|x| matches!(world.blocks[*x], Block::Redstone(_)))
            .collect();
        world.step();

        world
    }
}
