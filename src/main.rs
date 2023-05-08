use crate::world::World;
use std::fs::File;

mod blocks;
mod schematic;
pub mod world;
mod world_data;
mod world_logic;
mod world_prune;

fn main() {
    let world = World::from(File::open("./schematics/8bit_cpu_1.1.schem").unwrap());

    println!("{:?}", world.blocks.node_count());
    println!("{:?}", world.blocks.edge_count());
}
