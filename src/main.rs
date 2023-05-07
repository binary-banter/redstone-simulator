use std::fs::File;
use crate::world::World;

mod block;
mod construction_block;
mod facing;
mod schematic;
pub mod world;
mod world_data;
mod world_logic;

fn main() {
    let world = World::from(File::open("./schematics/8bit_cpu_1.1.schem").unwrap());

    println!("{:?}", world.blocks.node_count());
    println!("{:?}", world.blocks.edge_count());
}
