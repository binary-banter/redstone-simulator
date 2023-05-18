use crate::world::World;
use std::fs::File;
use std::path::Path;
use crate::cli::dot::write_dot;

mod blocks;
pub mod world;
mod cli;

fn main() {
    let world = World::from(File::open("./schematics/cpu_fib.schem").unwrap());
    let path = Path::new("./output.gv");
    write_dot(&world.cblocks, &world.cblock_positions, path);
}
