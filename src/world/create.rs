use crate::blocks::{CBlock, ToBlock};
use crate::world::data::{neighbours_and_facings, TileMap, WorldData};
use crate::world::prune::prune_graph;
use crate::world::schematic::SchemFormat;
use crate::world::{BlockGraph, CBlockGraph, World};
use bimap::BiHashMap;
use itertools::iproduct;
use nbt::from_gzip_reader;
use petgraph::graph::NodeIndex;
use petgraph::prelude::*;
use petgraph::visit::{IntoEdgeReferences, IntoNodeReferences};
use std::collections::{HashMap, VecDeque};
use std::fs::File;

impl From<File> for World {
    fn from(file: File) -> Self {
        World::from(from_gzip_reader::<File, SchemFormat>(file).unwrap())
    }
}

impl World {
    fn cblock_to_block(
        cblocks: CBlockGraph,
    ) -> (BlockGraph, Vec<NodeIndex>, BiHashMap<NodeIndex, String>) {
        let mut blocks = BlockGraph::with_capacity(cblocks.node_count(), cblocks.edge_count());

        let mut triggers = Vec::new();
        let mut probes = BiHashMap::new();

        let mut nodes_map = HashMap::with_capacity(cblocks.node_count());

        for (idx_old, cblock) in cblocks.node_references() {
            let idx = blocks.add_node(cblock.to_block());
            nodes_map.insert(idx_old, idx);
            match cblock {
                CBlock::Probe(p) => {
                    probes.insert(idx, p.name.clone());
                }
                CBlock::Trigger(_) => {
                    triggers.push(idx);
                }
                _ => {}
            }
        }

        for edge in cblocks.edge_references() {
            blocks.add_edge(nodes_map[&edge.source()], nodes_map[&edge.target()], edge.weight().clone());
        }

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
            updatable: VecDeque::new(),
            tick_updatable: VecDeque::new(),
            tick_counter: 0,
        };

        // Update probes for initial state.
        world.tick_updatable = world.probes.left_values().cloned().collect();
        world.step();

        world
    }
}
