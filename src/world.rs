use crate::blocks::Block;
use crate::schematic::SchemFormat;
use nbt::{from_gzip_reader, Value};
use petgraph::adj::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;

#[derive(Debug, Default)]
pub struct World {
    blocks: Graph<Block, (), petgraph::Directed, u32>,
}

impl From<File> for World {
    fn from(file: File) -> Self {
        World::from(from_gzip_reader::<File, SchemFormat>(file).unwrap())
    }
}

impl From<SchemFormat> for World {
    fn from(format: SchemFormat) -> Self {
        let mut palette = vec![(Block::Air, false, false); format.palette_max as usize];

        let mut temp_world =
            vec![
                vec![vec![Block::Air; format.length as usize]; format.height as usize];
                format.width as usize
            ];

        let mut temp_idx = vec![
            vec![vec![None; format.length as usize]; format.height as usize];
            format.width as usize
        ];

        let mut blocks = Graph::<Block, (), petgraph::Directed, u32>::new();

        // construct palette
        for (id, i) in &format.palette {
            palette[*i as usize] = Block::from_id(id);
        }

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

                    let (block, _is_trigger, _is_probe) = &palette[ix];
                    temp_world[x][y][z] = block.clone();
                }
            }
        }

        // construct nodes
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    let block = temp_world[x][y][z].clone();

                    match block {
                        Block::Air => continue,
                        _ => {
                            temp_idx[x][y][z] = Some(blocks.add_node(block));
                        }
                    };
                }
            }
        }

        let neighbours = |x: usize, y: usize, z: usize| {
            let mut vec = Vec::new();

            if x != 0 {
                vec.push((x.wrapping_sub(1), y, z));
            }
            if x != format.width as usize - 1 {
                (x.wrapping_add(1), y, z);
            }
            if y != 0 {
                vec.push((x, y.wrapping_sub(1), z));
            }
            if y != format.height as usize - 1 {
                vec.push((x, y.wrapping_add(1), z));
            }
            if z != 0 {
                vec.push((x, y, z.wrapping_sub(1)));
            }
            if z != format.length as usize - 1 {
                vec.push((x, y, z.wrapping_add(1)));
            }

            vec.into_iter()
        };

        // construct edges
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    if let Some(idx) = temp_idx[x][y][z] {
                        for n_idx in neighbours(x, y, z).filter_map(|(x, y, z)| temp_idx[x][y][z]) {
                            match (&blocks[idx], &blocks[n_idx]) {
                                (Block::Redstone(_), Block::Redstone(_))
                                | (Block::Redstone(_), Block::Solid(_))
                                | (Block::Redstone(_), Block::Repeater(_))
                                | (Block::Redstone(_), Block::Comparator(_))
                                | (Block::Solid(_), Block::Redstone(_))
                                | (Block::Solid(_), Block::Repeater(_))
                                | (Block::Solid(_), Block::Comparator(_))
                                | (Block::Solid(_), Block::Torch(_))
                                | (Block::Repeater(_), Block::Redstone(_))
                                | (Block::Repeater(_), Block::Solid(_))
                                | (Block::Repeater(_), Block::Repeater(_))
                                | (Block::Repeater(_), Block::Comparator(_))
                                | (Block::Comparator(_), Block::Redstone(_))
                                | (Block::Comparator(_), Block::Solid(_))
                                | (Block::Comparator(_), Block::Repeater(_))
                                | (Block::Comparator(_), Block::Comparator(_))
                                | (Block::Torch(_), Block::Redstone(_))
                                | (Block::Torch(_), Block::Solid(_))
                                | (Block::Torch(_), Block::Repeater(_))
                                | (Block::Torch(_), Block::Comparator(_))
                                | (Block::Trigger(_), Block::Redstone(_))
                                | (Block::Trigger(_), Block::Repeater(_))
                                | (Block::Trigger(_), Block::Comparator(_))
                                | (Block::Trigger(_), Block::Torch(_)) => {
                                    blocks.add_edge(idx, n_idx, ());
                                }
                                (_, _) => {}
                            }
                        }
                    }
                }
            }
        }

        World { blocks }
    }
}

impl World {
    // fn from_format(format: &SchemFormat) -> Self {
    //     let mut palette = vec![(Block::Air, false, false); format.palette_max as usize];
    //
    //     for (id, i) in &format.palette {
    //         palette[*i as usize] = Block::from_id(id);
    //     }
    //
    //     let mut world = World::new_empty(
    //         format.width as usize,
    //         format.height as usize,
    //         format.length as usize,
    //     );
    //
    //     let signs: HashMap<_, _> = format
    //         .block_entities
    //         .iter()
    //         .filter_map(|b| {
    //             if b.id == "minecraft:sign" {
    //                 if let Some(Value::String(s)) = b.props.get("Text1") {
    //                     let j: serde_json::Value = serde_json::from_str(s).unwrap();
    //                     let t = j
    //                         .as_object()
    //                         .unwrap()
    //                         .get("text")
    //                         .unwrap()
    //                         .as_str()
    //                         .unwrap()
    //                         .to_string();
    //
    //                     return Some((
    //                         (b.pos[0] as usize, b.pos[1] as usize, b.pos[2] as usize),
    //                         t,
    //                     ));
    //                 }
    //             }
    //             None
    //         })
    //         .collect();
    //
    //     let mut i = 0;
    //     for y in 0..format.height as usize {
    //         for z in 0..format.length as usize {
    //             for x in 0..format.width as usize {
    //                 let mut ix: usize = 0;
    //                 for j in 0.. {
    //                     let next = format.block_data[i];
    //                     ix |= (next as usize & 0b0111_1111) << (j * 7);
    //                     i += 1;
    //
    //                     if next >= 0 {
    //                         break;
    //                     }
    //                 }
    //
    //                 let (block, is_trigger, is_probe) = &palette[ix];
    //                 world.data[(x, y, z)] = block.clone();
    //                 if *is_trigger {
    //                     world.triggers.push((x, y, z));
    //                 }
    //                 if *is_probe {
    //                     let name: String = world
    //                         .data
    //                         .neighbours((x, y, z))
    //                         .into_iter()
    //                         .find_map(|nb| signs.get(&nb).cloned())
    //                         .unwrap_or(format!("{x},{y},{z}"));
    //                     world.probes.insert((x, y, z), name);
    //                 }
    //
    //                 world.updatable.push((x, y, z));
    //             }
    //         }
    //     }
    //
    //     world.step();
    //
    //     world
    // }
    //
    // pub fn get_probe(&self, name: &str) -> bool {
    //     match self.data[*self
    //         .probes
    //         .get_by_right(name)
    //         .expect("Probe does not exist.")]
    //     {
    //         Block::Solid(Solid { signal: s }) => <SolidPower as Into<u8>>::into(s) > 0,
    //         _ => unreachable!(),
    //     }
    // }
    //
    // pub fn get_probes(&self) -> HashMap<&str, bool> {
    //     self.probes
    //         .iter()
    //         .map(|(&(x, y, z), s)| {
    //             (
    //                 &s[..],
    //                 match self.data[(x, y, z)] {
    //                     Block::Solid(Solid {
    //                         signal: SolidPower::Weak(0) | SolidPower::Strong(0),
    //                     }) => false,
    //                     Block::Solid(_) => true,
    //                     _ => unreachable!(),
    //                 },
    //             )
    //         })
    //         .collect()
    // }
    //
    // pub fn display_probes(&self) {
    //     for (&(x, y, z), name) in &self.probes {
    //         match self.data[(x, y, z)] {
    //             Block::Solid(Solid { signal: s }) => {
    //                 println!("Probe '{name}': {}", <SolidPower as Into<u8>>::into(s))
    //             }
    //             _ => unreachable!(),
    //         }
    //     }
    // }
}

// impl Display for World {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.data)
//     }
// }
