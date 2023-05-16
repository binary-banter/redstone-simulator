use crate::world::World;
use std::fs::File;

mod blocks;
pub mod world;

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

    // let mut f = File::create("output.gv").expect("Unable to create file");
    // writeln!(f, "{:?}", Dot::with_config(&world.blocks, &[])).unwrap();
    //
    // println!(
    //     "nodes: {}, edges: {}",
    //     world.blocks.node_count(),
    //     world.blocks.edge_count()
    // );
    // println!(
    //     "Repeaters: {}",
    //     world
    //         .blocks
    //         .node_weights()
    //         .filter(|b| matches!(b, Block::Repeater(_)))
    //         .count()
    // );
    // println!(
    //     "Torches: {}",
    //     world
    //         .blocks
    //         .node_weights()
    //         .filter(|b| matches!(b, Block::Torch(_)))
    //         .count()
    // );
    // println!(
    //     "Comps: {}",
    //     world
    //         .blocks
    //         .node_weights()
    //         .filter(|b| matches!(b, Block::Comparator(_)))
    //         .count()
    // );
    // println!(
    //     "Redstone: {}",
    //     world
    //         .blocks
    //         .node_weights()
    //         .filter(|b| matches!(b, Block::Redstone(_)))
    //         .count()
    // );

    for _ in 0..400000 {
        world.step_with_trigger();
        world.step_with_trigger();
        for _ in 0..50 {
            world.step();
        }
        // println!(
        //     "a: {}{}{}{}{}{}{}{}",
        //     world.get_probe("7").unwrap() as u8,
        //     world.get_probe("6").unwrap() as u8,
        //     world.get_probe("5").unwrap() as u8,
        //     world.get_probe("4").unwrap() as u8,
        //     world.get_probe("3").unwrap() as u8,
        //     world.get_probe("2").unwrap() as u8,
        //     world.get_probe("1").unwrap() as u8,
        //     world.get_probe("0").unwrap() as u8,
        // );
        // println!(
        //     "pc: {}{}{}{}{}{}{}{}",
        //     world.get_probe("pc7").unwrap() as u8,
        //     world.get_probe("pc6").unwrap() as u8,
        //     world.get_probe("pc5").unwrap() as u8,
        //     world.get_probe("pc4").unwrap() as u8,
        //     world.get_probe("pc3").unwrap() as u8,
        //     world.get_probe("pc2").unwrap() as u8,
        //     world.get_probe("pc1").unwrap() as u8,
        //     world.get_probe("pc0").unwrap() as u8,
        // );
        // println!();
    }
}
