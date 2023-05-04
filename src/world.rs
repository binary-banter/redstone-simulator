use crate::blocks::solid::Solid;
use crate::blocks::Block;
use crate::schematic::SchemFormat;
use crate::world_data::WorldData;
use bimap::BiMap;
use nbt::{from_gzip_reader, Value};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;

pub struct World {
    pub size_x: usize,
    pub size_y: usize,
    pub size_z: usize,
    pub data: WorldData,
    pub triggers: Vec<(usize, usize, usize)>,
    pub probes: BiMap<(usize, usize, usize), String>,
    pub updatable: Vec<(usize, usize, usize)>,
}

impl World {
    pub fn new_empty(size_x: usize, size_y: usize, size_z: usize) -> Self {
        Self {
            size_x,
            size_y,
            size_z,
            data: WorldData(vec![vec![vec![Block::Air; size_z]; size_y]; size_x]),
            triggers: vec![],
            probes: BiMap::new(),
            updatable: vec![],
        }
    }

    pub fn from_file(file: &File) -> Self {
        let format: SchemFormat = from_gzip_reader(file).unwrap();
        println!("{:?}", format.block_data);
        World::from_format(&format)
    }

    fn from_format(format: &SchemFormat) -> Self {
        let mut palette = vec![(Block::Air, false, false); format.palette_max as usize];

        for (id, i) in &format.palette {
            palette[*i as usize] = Block::from_id(id);
        }

        let mut world = World::new_empty(
            format.width as usize,
            format.height as usize,
            format.length as usize,
        );

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

        let mut i = 0;
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    let mut ix: usize = 0;
                    for j in 0.. {
                        let next = format.block_data[i];
                        ix |= (next as usize & 0b0111_1111) << (j*7);
                        i += 1;

                        if next >= 0 {
                            break;
                        }
                    }

                    let (block, is_trigger, is_probe) = &palette[ix];
                    world.data[(x, y, z)] = block.clone();
                    if *is_trigger {
                        world.triggers.push((x, y, z));
                    }
                    if *is_probe {
                        let name: String = world
                            .data
                            .neighbours((x, y, z))
                            .into_iter()
                            .find_map(|nb| signs.get(&nb).cloned())
                            .unwrap_or(format!("{x},{y},{z}"));
                        world.probes.insert((x, y, z), name);
                    }

                    world.updatable.push((x, y, z));


                }
            }
        }

        world.step();

        world
    }

    pub fn get_probe(&self, name: &str) -> bool {
        match self.data[*self
            .probes
            .get_by_right(name)
            .expect("Probe does not exist.")]
        {
            Block::Solid(Solid { signal: s }) => s > 0,
            _ => unreachable!(),
        }
    }

    pub fn get_probes(&self) -> HashMap<&str, bool> {
        self.probes
            .iter()
            .map(|(&(x, y, z), s)| {
                (
                    &s[..],
                    match self.data[(x, y, z)] {
                        Block::Solid(Solid { signal: 0 }) => false,
                        Block::Solid(_) => true,
                        _ => unreachable!(),
                    },
                )
            })
            .collect()
    }

    pub fn display_probes(&self) {
        for (&(x, y, z), name) in &self.probes {
            match self.data[(x, y, z)] {
                Block::Solid(Solid { signal: s }) => println!("Probe '{name}': {s}"),
                _ => unreachable!(),
            }
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
