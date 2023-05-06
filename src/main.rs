mod blocks;
mod schematic;
pub mod world;
mod world_data;
pub mod world_logic;

use crate::world::World;
use std::fs::File;

fn main() {
    let file = File::open("./schematics/8bit_cpu_1.1.schem").unwrap();
    let mut world = World::from_file(&file);

    // println!("{}", world);
    for _ in 0..50000 {
        world.step_with_trigger();
        for _ in 0..10 {
            world.step();
        }
    }

}
