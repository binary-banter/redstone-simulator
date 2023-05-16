use crate::blocks::{Block, CBlock};
use crate::world::data::{neighbours_and_facings, TileMap, WorldData};
use crate::world::graph::GNode;
use crate::world::prune::prune_graph;
use crate::world::schematic::SchemFormat;
use crate::world::{BlockGraph, CBlockGraph, TickUpdatableList, UpdatableList, World};
use itertools::iproduct;
use nbt::from_gzip_reader;
use std::collections::HashMap;
use std::fs::File;

impl From<File> for World {
    fn from(file: File) -> Self {
        World::from(from_gzip_reader::<File, SchemFormat>(file).unwrap())
    }
}

impl World {
    fn cblock_to_block(
        cblocks: CBlockGraph,
    ) -> (
        BlockGraph,
        Vec<&'static GNode<Block, u8>>,
        HashMap<String, &'static GNode<Block, u8>>,
    ) {
        let mut triggers = Vec::new();
        let mut probes = HashMap::new();
        let blocks = BlockGraph::from_petgraph(&cblocks, |cblock, block_ref| match cblock {
            CBlock::Probe(p) => {
                probes.insert(p.name.clone(), block_ref);
            }
            CBlock::Trigger(_) => {
                triggers.push(block_ref);
            }
            _ => {}
        });

        (blocks, triggers, probes)
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

        let height = format.height as usize;
        let length = format.length as usize;
        let width = format.width as usize;

        let mut cblocks = CBlockGraph::new();
        let mut indexes = vec![vec![vec![vec![]; length]; height]; width];

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
                    let n_idxs = indexes
                        .get(np.0)
                        .and_then(|l| l.get(np.1).and_then(|l| l.get(np.2)))
                        .map(|v| v.iter())
                        .into_iter()
                        .flatten();

                    for (n_block, &n_idx) in world[np].iter().zip(n_idxs) {
                        if let Some(edge) = block.get_edge(n_block, f) {
                            cblocks.add_edge(idx, n_idx, edge);
                        }
                    }
                }

                // construct vertical edges for redstone
                if let CBlock::Redstone(v) = block {
                    v.add_vertical_edges((x, y, z), &mut cblocks, &world, &indexes);
                }
            }
        }

        prune_graph(&mut cblocks);

        // CBlock graph to Block graph
        let (blocks, triggers, probes) = World::cblock_to_block(cblocks);

        let mut world = World {
            blocks,
            triggers,
            probes,
            updatable: UpdatableList::new(),
            tick_updatable: TickUpdatableList::new(),
            tick_counter: 0,
        };

        // Update probes for initial state.
        // Probes ignore the `false`
        world.tick_updatable = world.probes.values().cloned().map(|n| (n, false)).collect();
        world.step();

        world
    }
}
