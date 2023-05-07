mod blocks;
mod schematic;
pub mod world;
mod world_data;
pub mod world_logic;

use crate::world::World;
use std::fs::File;

fn main() {
    let world = World::from(File::open("./schematics/8bit_cpu_1.1.schem").unwrap());

    println!("{:?}", world);
}
