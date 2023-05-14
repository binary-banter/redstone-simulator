use crate::blocks::Block;
use crate::blocks::{CBlock};
use crate::world::data::{neighbours_and_facings, TileMap, WorldData};
use crate::world::schematic::SchemFormat;
use crate::world::{BlockGraph, CBlockGraph, World};
use bimap::BiMap;
use itertools::iproduct;
use nbt::from_gzip_reader;
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

        let world = WorldData::from_format(&format, &tile_map);
        let mut blocks = BlockGraph::new();

        let height = format.height as usize;
        let length = format.length as usize;
        let width = format.width as usize;

        let mut cblocks = CBlockGraph::new();
        let mut indexes = vec![vec![vec![vec![]; length]; height]; width];

        //TODO initialize
        let mut triggers = Vec::new();
        let mut probes = BiMap::new();

        // Construct nodes.
        for (x, y, z) in iproduct!(0..width, 0..height, 0..length) {
            for block in &world[(x, y, z)] {
                indexes[x][y][z].push(cblocks.add_node(block.clone()));
            }
        }

        // Construct edges.
        for (x, y, z) in iproduct!(0..width, 0..height, 0..length) {
            for (block, &idx) in world[(x, y, z)].iter().zip(indexes[x][y][z].iter()) {
                for (np, f) in neighbours_and_facings((x, y, z)) {
                    for (n_block, &n_idx) in world[np].iter().zip(indexes[np.0][np.1][np.2].iter()) {
                        if let Some(edge) = block.get_edge(n_block, f){
                            cblocks.add_edge(idx, n_idx, edge);
                        }
                    }
                }

                // construct vertical edges for redstone
                if let CBlock::Redstone(v) = block {
                    v.add_vertical_edges((x, y, z), &mut blocks, &world)
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
