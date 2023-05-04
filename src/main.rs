mod blocks;
mod schematic;
pub mod world;
mod world_data;
pub mod world_logic;

use crate::world::World;
use std::fs::File;

fn main() {
    let file = File::open("./schematics/repeater_2t.schem").unwrap();
    let mut world = World::from_file(&file);

    world.step_with_trigger();
    println!("{world}");
    world.step();
    println!("{world}");
    world.step();
    println!("{world}");
    world.step();
    println!("{world}");
    world.step();
    println!("{world}");
}
