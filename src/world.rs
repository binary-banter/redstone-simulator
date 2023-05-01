use crate::block::Block;
use crate::schematic::SchemFormat;
use nbt::from_gzip_reader;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::ops::{Index, IndexMut};

pub struct World {
    pub(crate) size_x: usize,
    pub(crate) size_y: usize,
    pub(crate) size_z: usize,
    pub(crate) data: Vec<Vec<Vec<Block>>>,
    pub(crate) triggers: Vec<(usize, usize, usize)>,
    pub(crate) probes: Vec<(usize, usize, usize)>,
    pub(crate) updatable: Vec<(usize, usize, usize)>,
}

impl World {
    pub fn new_empty(size_x: usize, size_y: usize, size_z: usize) -> Self {
        Self {
            size_x,
            size_y,
            size_z,
            data: vec![vec![vec![Block::Air; size_x]; size_y]; size_z],
            triggers: vec![],
            probes: vec![],
            updatable: vec![],
        }
    }

    pub fn from_file(file: &File) -> Self {
        let format: SchemFormat = from_gzip_reader(file).unwrap();
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

        let mut i = 0;
        for y in 0..format.height as usize {
            for z in 0..format.length as usize {
                for x in 0..format.width as usize {
                    let (block, is_trigger, is_probe) = palette[format.block_data[i] as usize];
                    world.data[x][y][z] = block;
                    if is_trigger {
                        world.triggers.push((x, y, z));
                    }
                    if is_probe {
                        world.probes.push((x, y, z));
                    }

                    i += 1;
                }
            }
        }

        world
    }

    pub fn get_probes(&self) -> Vec<bool> {
        self.probes
            .iter()
            .map(|&(x, y, z)| match self.data[x][y][z] {
                Block::Solid(0) => false,
                Block::Solid(_) => true,
                _ => unreachable!(),
            })
            .collect()
    }

    pub fn display_probes(&self) {
        for &(x, y, z) in &self.probes {
            match self.data[x][y][z] {
                Block::Solid(i) => println!("Probe at ({x}, {y}, {z}): {i}"),
                _ => unreachable!(),
            }
        }
    }
}

impl Index<(usize, usize, usize)> for World {
    type Output = Block;

    fn index(&self, (x, y, z): (usize, usize, usize)) -> &Self::Output {
        &self.data[x][y][z]
    }
}

impl IndexMut<(usize, usize, usize)> for World {
    fn index_mut(&mut self, (x, y, z): (usize, usize, usize)) -> &mut Self::Output {
        &mut self.data[x][y][z]
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.data[0].len()).rev() {
            for z in 0..self.data[0][0].len() {
                for x in 0..self.data.len() {
                    write!(f, "{}", self.data[x][y][z])?;
                }
                writeln!(f)?;
            }
            writeln!(f, "-----------------------")?;
        }
        Ok(())
    }
}
