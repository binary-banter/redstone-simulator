use crate::block::Block;
use crate::construction_block::CBlock;
use crate::schematic::SchemFormat;
use bimap::BiMap;
use nbt::{from_gzip_reader, Value};
use petgraph::prelude::StableGraph;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;
use std::fs::File;
use crate::facing::Facing;

use crate::world_data::WorldData;

#[derive(Debug)]
pub struct World {
    pub blocks: StableGraph<Block, u8, petgraph::Directed, u32>,
    pub triggers: Vec<NodeIndex>,
    probes: BiMap<NodeIndex, String>,
    pub updatable: Vec<NodeIndex>,
}

impl World {
    fn create_world(format: &SchemFormat) -> WorldData {
        // Create palette
        let mut palette = vec![CBlock::Air; format.palette_max as usize];
        for (id, i) in &format.palette {
            palette[*i as usize] = CBlock::from_id(id);
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

    pub fn get_probe(&self, name: &str) -> bool {
        match self.blocks[*self
            .probes
            .get_by_right(name)
            .expect("Probe does not exist.")]
        {
            Block::Solid(s) => s > 0,
            _ => unreachable!("Probe was not a Solid block. Parsing went wrong."),
        }
    }

    pub fn get_probes(&self) -> HashMap<&str, bool> {
        self.probes
            .iter()
            .map(|(i, s)| {
                (
                    &s[..],
                    match self.blocks[*i] {
                        Block::Solid(s) => s > 0,
                        _ => unreachable!("Probe was not a Solid block. Parsing went wrong."),
                    },
                )
            })
            .collect()
    }
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

        // construct nodes
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    let block = &mut world[(x, y, z)];

                    match block {
                        CBlock::Air => continue,
                        CBlock::Redstone { signal, node, .. } => {
                            *node = Some(blocks.add_node(Block::Redstone(*signal)));
                        }
                        CBlock::Trigger { node } => {
                            let idx = blocks.add_node(Block::Solid(0));
                            *node = Some(idx);
                            triggers.push(idx);
                        }
                        CBlock::Probe { node } => {
                            let idx = blocks.add_node(Block::Solid(0));
                            *node = Some(idx);

                            let name: String = world
                                .neighbours((x, y, z))
                                .into_iter()
                                .find_map(|nb| signs.get(&nb).cloned())
                                .unwrap_or(format!("{x},{y},{z}"));
                            probes.insert(idx, name);
                        }
                        CBlock::Solid { weak, strong } => {
                            *weak = Some(blocks.add_node(Block::Solid(0)));
                            *strong = Some(blocks.add_node(Block::Solid(0)));
                        }
                        CBlock::Repeater {
                            powered,
                            delay,
                            node,
                            ..
                        } => {
                            *node = Some(blocks.add_node(Block::Repeater {
                                powered: *powered,
                                next_powered: *powered,
                                delay: *delay,
                                count: 0,
                            }));
                        }
                        CBlock::RedstoneBlock { node } => {
                            *node = Some(blocks.add_node(Block::RedstoneBlock));
                        }
                        CBlock::Torch { lit, node, .. } => {
                            *node = Some(blocks.add_node(Block::Torch { lit: *lit }))
                        }
                        CBlock::Comparator { signal, mode, node, .. } => {
                            let rear = blocks.add_node(Block::Redstone(0));
                            let side = blocks.add_node(Block::Redstone(0));
                            let comp = blocks.add_node(Block::Comparator {
                                signal: *signal,
                                next_signal: *signal,
                                mode: *mode,
                                rear,
                                side,
                            });
                            blocks.add_edge(rear, comp, 0);
                            blocks.add_edge(side, comp, 0);
                            *node = Some(comp)
                        }
                    };
                }
            }
        }

        // construct edges
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    add_connecting_edges((x, y, z), &world, &mut blocks);
                }
            }
        }

        //TODO find fixpoint
        // for _ in 0..20 {
        //     blocks.retain_nodes(|x, y| {
        //         (x.neighbors_directed(y, Outgoing).count() > 0 && x.neighbors_directed(y, Incoming).count() > 0) || probes.contains_left(&y) || triggers.contains(&y)
        //     });
        // }

        let updatable = blocks.node_indices().collect();

        let mut world = World {
            blocks,
            triggers,
            probes,
            updatable,
        };

        world.step();

        world
    }
}

fn add_connecting_edges(
    p: (usize, usize, usize),
    world: &WorldData,
    blocks: &mut StableGraph<Block, u8, petgraph::Directed, u32>,
) {
    let cblock = world[p];
    for (np, f) in world.neighbours_and_facings(p) {
        let n_cblock = world[np];

        // regular connections
        #[rustfmt::skip]
        match (cblock, n_cblock) {
            (CBlock::Redstone { node: Some(idx), .. }, CBlock::Redstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, n_idx, 1);
            }
            (CBlock::Redstone { node: Some(idx), connections, .. }, CBlock::Solid { weak: Some(n_idx), .. }) if connections[f] => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Redstone { node: Some(idx), connections, .. }, CBlock::Probe { node: Some(n_idx), .. }) if connections[f] => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Redstone { node: Some(idx), .. }, CBlock::Repeater { node: Some(n_idx), facing, .. }) if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Redstone { node: Some(idx), .. }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if n_facing == f.reverse() => {
                let Block::Comparator { rear, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(idx, rear, 0);
            }
            (CBlock::Redstone { node: Some(idx), .. }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if n_facing == f.rotate_right() || n_facing == f.rotate_left() => {
                let Block::Comparator { side, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(idx, side, 0);
            }

            (CBlock::Trigger { node: Some(idx), .. }, CBlock::Redstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Trigger { node: Some(idx), .. }, CBlock::Repeater { node: Some(n_idx), facing, .. }) if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Trigger { node: Some(idx), ..}, CBlock::Torch { node: Some(n_idx), facing, .. }) if facing == f => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Trigger { node: Some(idx), ..}, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if n_facing == f.reverse() => {
                let Block::Comparator { rear, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(idx, rear, 0);
            }

            (CBlock::Solid { strong: Some(idx), .. }, CBlock::Redstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Solid { weak: Some(w_idx), strong: Some(s_idx), .. }, CBlock::Repeater { node: Some(n_idx), facing, .. }) if facing == f.reverse() => {
                blocks.add_edge(w_idx, n_idx, 0);
                blocks.add_edge(s_idx, n_idx, 0);
            }
            (CBlock::Solid {weak: Some(w_idx), strong: Some(s_idx), ..}, CBlock::Torch { node: Some(n_idx), facing, .. }) if facing == f => {
                blocks.add_edge(w_idx, n_idx, 0);
                blocks.add_edge(s_idx, n_idx, 0);
            }
            (CBlock::Solid {weak: Some(w_idx), strong: Some(s_idx), ..}, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if n_facing == f.reverse() => {
                let Block::Comparator { rear, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(w_idx, rear, 0);
                blocks.add_edge(s_idx, rear, 0);
            }

            (CBlock::Repeater { node: Some(idx), facing, .. }, CBlock::Redstone { node: Some(n_idx), .. }) if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Repeater { node: Some(idx), facing, .. }, CBlock::Solid { strong: Some(n_idx), .. }) if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Repeater { node: Some(idx), facing, .. }, CBlock::Probe { node: Some(n_idx), .. }) if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Repeater { node: Some(idx), facing, .. }, CBlock::Repeater { node: Some(n_idx), facing: n_facing, .. }) if facing == f.reverse() && n_facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Repeater { node: Some(idx), facing, .. }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if facing == f.reverse() && n_facing == f.reverse() => {
                let Block::Comparator { rear, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(idx, rear, 0);
            }
            (CBlock::Repeater { node: Some(idx), facing, .. }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if facing == f.reverse() && (n_facing == f.rotate_right() || n_facing == f.rotate_left()) => {
                let Block::Comparator { side, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(idx, side, 0);
            }

            (CBlock::RedstoneBlock { node: Some(idx) }, CBlock::Redstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::RedstoneBlock { node: Some(idx) }, CBlock::Repeater { node: Some(n_idx), facing, .. })  if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::RedstoneBlock { node: Some(idx) }, CBlock::Torch { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::RedstoneBlock { node: Some(idx) }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if n_facing == f.reverse() => {
                let Block::Comparator { rear, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(idx, rear, 0);
            }
            (CBlock::RedstoneBlock { node: Some(idx) }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if n_facing == f.rotate_right() || n_facing == f.rotate_left() => {
                let Block::Comparator { side, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(idx, side, 0);
            }

            (CBlock::Torch { node: Some(idx), .. }, CBlock::Redstone { node: Some(n_idx), .. }) => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Torch { node: Some(idx), .. }, CBlock::Solid { strong: Some(s_idx), .. }) if f == Facing::Up => {
                blocks.add_edge(idx, s_idx, 0);
            }
            (CBlock::Torch { node: Some(idx), .. }, CBlock::Probe { node: Some(n_idx), .. }) if f == Facing::Up => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Torch { node: Some(idx), .. }, CBlock::Repeater { node: Some(n_idx), facing, .. }) if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Torch { node: Some(idx), .. }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if n_facing == f.reverse() => {
                let Block::Comparator { rear, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(idx, rear, 0);
            }

            (CBlock::Comparator { node: Some(idx), facing, .. }, CBlock::Redstone { node: Some(n_idx), .. }) if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Comparator { node: Some(idx), facing, .. }, CBlock::Solid { strong: Some(n_idx), .. }) if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Comparator { node: Some(idx), facing, .. }, CBlock::Probe { node: Some(n_idx), .. }) if facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Comparator { node: Some(idx), facing, .. }, CBlock::Repeater { node: Some(n_idx), facing: n_facing, .. }) if facing == f.reverse() && n_facing == f.reverse() => {
                blocks.add_edge(idx, n_idx, 0);
            }
            (CBlock::Comparator { node: Some(idx), facing, .. }, CBlock::Comparator { node: Some(n_idx), facing: n_facing, .. }) if facing == f.reverse() && n_facing == f.reverse() => {
                let Block::Comparator { rear, ..} = blocks[n_idx] else {
                    unreachable!();
                };
                blocks.add_edge(idx, rear, 0);
            }

            _ => {}
        };

        // redstone up/down connections
        if let CBlock::Redstone { node: Some(idx), .. } = cblock {
            let top = (p.0, p.1.wrapping_add(1), p.2);
            let bottom = (p.0, p.1.wrapping_sub(1), p.2);
            for f in [Facing::North, Facing::East, Facing::South, Facing::West] {
                let side = f.front(p);
                let side_down = (side.0, side.1.wrapping_sub(1), side.2);
                let side_up = (side.0, side.1.wrapping_add(1), side.2);

                match [side_down, side, side_up, bottom, top].map(|n| &world[n]) {
                    //Down
                    [CBlock::Redstone{ node: Some(n_idx) , ..}, b1, _, b2, _] if b1.is_transparent() && !b2.is_transparent() => {
                        blocks.add_edge(idx, *n_idx, 1);
                    },
                    //Up
                    [_, _, CBlock::Redstone { node: Some(n_idx), .. }, _, b2] if b2.is_transparent() => {
                        blocks.add_edge(idx, *n_idx, 1);
                    },
                    _ => {}
                }
            }
        }

    }
}
