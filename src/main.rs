use nbt::{from_gzip_reader, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::mem;
use std::ops::{Index, IndexMut};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SchemFormat {
    #[serde(serialize_with = "nbt::i8_array")]
    block_data: Vec<i8>,
    block_entities: Vec<SchemBlockEntity>,
    data_version: i32,
    height: i16,
    length: i16,
    metadata: Metadata,
    #[serde(serialize_with = "nbt::i32_array")]
    offset: Vec<i32>,
    palette: HashMap<String, i32>,
    palette_max: i32,
    version: i32,
    width: i16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SchemBlockEntity {
    id: String,

    #[serde(serialize_with = "nbt::i32_array")]
    pos: Vec<i32>,

    #[serde(flatten)]
    props: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct Metadata {
    #[serde(rename = "WEOffsetX")]
    offset_x: i32,
    #[serde(rename = "WEOffsetY")]
    offset_y: i32,
    #[serde(rename = "WEOffsetZ")]
    offset_z: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Block {
    Solid(u8),
    Redstone(u8),
    Air,
    Trigger(bool),
}

impl Block {
    /// Returns (Block, is_trigger, is_probe)
    pub fn from_id(id: &str) -> (Self, bool, bool) {
        // minecraft:redstone_wire[east=none,north=side,power=0,south=side,west=none]
        let (id, meta) = id
            .split_once('[')
            .map(|(x, y)| (x, y.trim_end_matches(']')))
            .unwrap_or((id, ""));

        match id {
            "minecraft:redstone_wire" => (Block::Redstone(0), false, false),
            "minecraft:air" => (Block::Air, false, false),
            "minecraft:stone" => (Block::Solid(0), false, false),
            "minecraft:gold_block" => (Block::Trigger(false), true, false),
            "minecraft:diamond_block" => (Block::Solid(0), false, true),
            _ => todo!("Unimplemented identifier: {id}."),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Solid(i) => match *i {
                0 => write!(f, "□"),
                1 => write!(f, "■"),
                16 => write!(f, "▣"),
                _ => unreachable!(),
            }
            Block::Redstone(i) => write!(f, "{}", "0123456789ABCDEF".chars().nth(*i as usize).unwrap()),
            Block::Air => write!(f, " "),
            Block::Trigger(_) => write!(f, "T"),
        }
    }
}

struct World {
    size_x: usize,
    size_y: usize,
    size_z: usize,
    data: Vec<Vec<Vec<Block>>>,
    triggers: Vec<(usize, usize, usize)>,
    probes: Vec<(usize, usize, usize)>,
    updatable: Vec<(usize, usize, usize)>,
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

    pub fn from_format(format: &SchemFormat) -> Self {
        let mut palette = vec![(Block::Air, false, false); format.palette_max as usize];
        for (id, i) in &format.palette {
            palette[*i as usize] = Block::from_id(&id);
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

    pub fn step_with_trigger(&mut self) {
        // put redstone power on triggers
        for &(tx, ty, tz) in &self.triggers {
            self.data[tx][ty][tz] = Block::Trigger(true);
            for (nx, ny, nz) in self.neighbours(tx, ty, tz) {
                self.updatable.push((nx, ny, nz));
            }
        }

        self.step();

        // take redstone power off triggers
        for &(tx, ty, tz) in &self.triggers {
            self.data[tx][ty][tz] = Block::Trigger(false);
            for (nx, ny, nz) in self.neighbours(tx, ty, tz) {
                self.updatable.push((nx, ny, nz));
            }
        }
    }

    fn neighbours(
        &self,
        x: usize,
        y: usize,
        z: usize,
    ) -> impl Iterator<Item = (usize, usize, usize)> {
        let mut vec: heapless::Vec<(usize, usize, usize), 6> = heapless::Vec::new();

        if x != 0 {
            vec.push((x - 1, y, z)).unwrap();
        }
        if x != self.size_x - 1 {
            vec.push((x + 1, y, z)).unwrap();
        }
        if y != 0 {
            vec.push((x, y - 1, z)).unwrap();
        }
        if y != self.size_y - 1 {
            vec.push((x, y + 1, z)).unwrap();
        }
        if z != 0 {
            vec.push((x, y, z - 1)).unwrap();
        }
        if z != self.size_z - 1 {
            vec.push((x, y, z + 1)).unwrap();
        }

        vec.into_iter()
    }

    pub fn step(&mut self) {
        let mut tick_updatable = mem::replace(&mut self.updatable, Vec::new());

        while let Some((x, y, z)) = tick_updatable.pop() {
            let block = self.data[x][y][z];

            match block {
                Block::Redstone(s) => {
                    // find biggest signal strength around this block
                    let s_new = self
                        .neighbours(x, y, z)
                        .map(|(nx, ny, nz)| {
                            let n_block = self.data[nx][ny][nz];
                            match n_block {
                                Block::Redstone(ns) => ns.saturating_sub(1),
                                Block::Trigger(true) => 15,
                                _ => 0,
                            }
                        })
                        .max()
                        .unwrap_or(0);

                    // if signal strength has changed, update neighbours
                    if s != s_new {
                        for (nx, ny, nz) in self.neighbours(x, y, z) {
                            tick_updatable.push((nx, ny, nz));
                        }
                        self.data[x][y][z] = Block::Redstone(s_new);
                    }
                }
                Block::Solid(s) => {
                    // find biggest signal strength around this block
                    let s_new = self
                        .neighbours(x, y, z)
                        .map(|(nx, ny, nz)| {
                            let n_block = self.data[nx][ny][nz];
                            match n_block {
                                Block::Redstone(1..) => 1,
                                _ => 0,
                            }
                        })
                        .max()
                        .unwrap_or(0);

                    // if signal strength has changed, update neighbours
                    if s != s_new {
                        for (nx, ny, nz) in self.neighbours(x, y, z) {
                            tick_updatable.push((nx, ny, nz));
                        }
                        self.data[x][y][z] = Block::Solid(s_new);
                    }
                }
                _ => {}
            }
        }

        print!("");
    }

    pub fn display_probes(&self) {
        for &(x,y,z) in &self.probes {
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

fn main() {
    let file = File::open("./schematics/steady_state.schem").unwrap();
    let format: SchemFormat = from_gzip_reader(file).unwrap();

    let mut world = World::from_format(&format);
    world.step_with_trigger();
    world.step();
    world.display_probes();
    println!("{}", world);
}
