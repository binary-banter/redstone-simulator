mod block;
mod schematic;
mod world;
mod world_logic;

use crate::world::World;
use std::fs::File;

fn main() {
    let file = File::open("./schematics/steady_state.schem").unwrap();
    let mut world = World::from_file(&file);

    world.step_with_trigger();
    world.step();
    world.display_probes();
    println!("{}", world);
}
