use crate::world::World;
use std::fs::File;

mod blocks;
mod schematic;
pub mod world;
mod world_data;
mod world_logic;
mod world_prune;

// sdl 0b1011      // load -5 [0b1111_1011] into adr
// sdh 0b1111
// stl 0b0001      // load 1 [0b0000_0001] into r0
// mov r0 r2       // move r0 into r2 (1)
// mov r1 r0       // .LOOP: move prev to A
// add r2          // add current to prev <--
// mov r2 r1       // move current to prev
// mov r0 r2       // move next to current
// jmp             // jump back to LOOP
fn main() {
    let mut world = World::from(File::open("./schematics/cpu_fib.schem").unwrap());

    println!("nodes: {}, edges: {}", world.blocks.node_count(), world.blocks.edge_count());

    for _ in 0..40 {
        world.step_with_trigger();
        world.step_with_trigger();
        for _ in 0..50 {
            world.step();
        }
        println!(
            "a: {}{}{}{}{}{}{}{}",
            world.get_probe("7") as u8,
            world.get_probe("6") as u8,
            world.get_probe("5") as u8,
            world.get_probe("4") as u8,
            world.get_probe("3") as u8,
            world.get_probe("2") as u8,
            world.get_probe("1") as u8,
            world.get_probe("0") as u8,
        );
        println!(
            "pc: {}{}{}{}{}{}{}{}",
            world.get_probe("pc7") as u8,
            world.get_probe("pc6") as u8,
            world.get_probe("pc5") as u8,
            world.get_probe("pc4") as u8,
            world.get_probe("pc3") as u8,
            world.get_probe("pc2") as u8,
            world.get_probe("pc1") as u8,
            world.get_probe("pc0") as u8,
        );
        println!();
    }
}
