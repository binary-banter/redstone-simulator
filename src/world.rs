use crate::blocks::CBlock;
use crate::blocks::{Block, BlockConnections};
use crate::schematic::SchemFormat;
use crate::world_data::WorldData;
use bimap::BiMap;
use nbt::{from_gzip_reader, Value};
use petgraph::prelude::StableGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
pub struct World {
    pub blocks: RedGraph,
    pub triggers: Vec<NodeIndex>,
    pub probes: BiMap<NodeIndex, String>,
    pub updatable: Vec<NodeIndex>,
}

pub type RedGraph = StableGraph<Block, u8, petgraph::Directed, u32>;

impl World {
    fn create_world(format: &SchemFormat) -> WorldData {
        // Create palette
        let mut palette = vec![CBlock::Air; format.palette_max as usize];
        for (id, i) in &format.palette {
            palette[*i as usize] = CBlock::from(id.as_str());
        }

        let mut world = vec![
            vec![vec![CBlock::Air; format.length as usize]; format.height as usize];
            format.width as usize
        ];

        // construct blocks from palette
        let mut i = 0;
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    let mut ix: usize = 0;
                    for j in 0.. {
                        let next = format.block_data[i];
                        ix |= (next as usize & 0b0111_1111) << (j * 7);
                        i += 1;

                        if next >= 0 {
                            break;
                        }
                    }

                    world[x][y][z] = palette[ix];
                }
            }
        }

        WorldData(world)
    }

    // pub fn get_probe(&self, name: &str) -> bool {
    //     match self.blocks[*self
    //         .probes
    //         .get_by_right(name)
    //         .expect("Probe does not exist.")]
    //     {
    //         Block::Redstone(s) => s > 0,
    //         _ => unreachable!("Probe was not a Solid block. Parsing went wrong."),
    //     }
    // }

    // pub fn get_probes(&self) -> HashMap<&str, bool> {
    //     self.probes
    //         .iter()
    //         .map(|(i, s)| {
    //             (
    //                 &s[..],
    //                 match self.blocks[*i] {
    //                     Block::Redstone(s) => s > 0,
    //                     _ => unreachable!("Probe was not a Solid block. Parsing went wrong."),
    //                 },
    //             )
    //         })
    //         .collect()
    // }
}

impl From<File> for World {
    fn from(file: File) -> Self {
        World::from(from_gzip_reader::<File, SchemFormat>(file).unwrap())
    }
}

impl From<SchemFormat> for World {
    fn from(format: SchemFormat) -> Self {
        let mut world = Self::create_world(&format);
        let mut blocks = StableGraph::<Block, u8, petgraph::Directed, u32>::new();

        let mut triggers = Vec::new();
        let mut probes = BiMap::new();

        let signs: HashMap<_, _> = format
            .block_entities
            .iter()
            .filter_map(|b| {
                if b.id == "minecraft:sign" {
                    if let Some(Value::String(s)) = b.props.get("Text1") {
                        let j: serde_json::Value = serde_json::from_str(s).unwrap();
                        let t = j
                            .as_object()
                            .unwrap()
                            .get("text")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();

                        return Some((
                            (b.pos[0] as usize, b.pos[1] as usize, b.pos[2] as usize),
                            t,
                        ));
                    }
                }
                None
            })
            .collect();

        // construct nodes todo
        // for y in 0..format.height as usize {
        //     for z in 0..format.length as usize {
        //         for x in 0..format.width as usize {
        //             let block = &mut world[(x, y, z)];
        //
        //             match block {
        //                 CBlock::Air => continue,
        //                 CBlock::Redstone { signal, node, .. } => {
        //                     *node = Some(blocks.add_node(Block::Redstone(*signal)));
        //                 }
        //                 CBlock::Trigger { node } => {
        //                     let idx = blocks.add_node(Block::Redstone(0));
        //                     *node = Some(idx);
        //                     triggers.push(idx);
        //                 }
        //                 CBlock::Probe { node } => {
        //                     let idx = blocks.add_node(Block::Redstone(0));
        //                     *node = Some(idx);
        //
        //                     let name: String = world
        //                         .neighbours((x, y, z))
        //                         .into_iter()
        //                         .find_map(|nb| signs.get(&nb).cloned())
        //                         .unwrap_or(format!("{x},{y},{z}"));
        //                     probes.insert(idx, name);
        //                 }
        //                 CBlock::Solid { weak, strong } => {
        //                     *weak = Some(blocks.add_node(Block::Redstone(0)));
        //                     *strong = Some(blocks.add_node(Block::Redstone(0)));
        //                 }
        //                 CBlock::Repeater {
        //                     powered,
        //                     delay,
        //                     node,
        //                     ..
        //                 } => {
        //                     *node = Some(blocks.add_node(Block::Repeater {
        //                         powered: *powered,
        //                         next_powered: *powered,
        //                         delay: *delay,
        //                         count: 0,
        //                     }));
        //                 }
        //                 CBlock::RedstoneBlock { node } => {
        //                     *node = Some(blocks.add_node(Block::RedstoneBlock));
        //                 }
        //                 CBlock::Torch { lit, node, .. } => {
        //                     *node = Some(blocks.add_node(Block::Torch { lit: *lit }))
        //                 }
        //                 CBlock::Comparator {
        //                     signal, mode, node, ..
        //                 } => {
        //                     let rear = blocks.add_node(Block::Redstone(0));
        //                     let side = blocks.add_node(Block::Redstone(0));
        //                     let comp = blocks.add_node(Block::Comparator {
        //                         signal: *signal,
        //                         next_signal: *signal,
        //                         mode: *mode,
        //                         rear,
        //                         side,
        //                     });
        //                     blocks.add_edge(rear, comp, 0);
        //                     blocks.add_edge(side, comp, 0);
        //                     *node = Some(comp)
        //                 }
        //             };
        //         }
        //     }
        // }

        // construct edges
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    let cblock = world[(x, y, z)];
                    for (np, f) in world.neighbours_and_facings((x, y, z)) {
                        cblock.connect(&world[np], f, &mut blocks);
                    }

                    // construct vertical edges for redstone
                    // todo
                }
            }
        }

        let mut world = World {
            blocks,
            triggers,
            probes,
            updatable: vec![],
        };

        // world.prune_graph(); todo

        world.updatable = world.blocks.node_indices().collect();
        world.step();

        world
    }
}
