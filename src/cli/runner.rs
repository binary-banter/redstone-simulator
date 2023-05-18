use std::fs::File;
use std::path::Path;
use crate::cli::instructions::{Instruction, InstructionAst, parse};
use crate::world::World;
use crate::cli::Args;
use crate::cli::dot::write_dot;

pub fn run(args: Args){
    let mut world = World::from(File::open(args.input).expect("Could not open schematic file."));

    if let Some(dot) = args.dot {
        write_dot(&world.cblocks, &world.cblock_positions, Path::new(&dot));
    }

    if let Some(wave) = args.wave{
        let Some(simulation) = args.simulation else {
            panic!("No simulation program was provided using the simulation flag!");
        };

        let Some(ast) = parse(&simulation) else {
            panic!("Could not parse simulation program provided!");
        };

        run_ast(&mut world, &ast);
    }
}

fn run_ast(world: &mut World, ast: &InstructionAst) {
    match ast {
        InstructionAst::Instruction(Instruction::Trigger) => {
            world.step_with_trigger();
        }
        InstructionAst::Instruction(Instruction::Step) => {
            world.step();
        }
        InstructionAst::Instruction(Instruction::Probe) => {
            //TODO world.get_probes();
        }
        InstructionAst::Sequence(v) => {
            for i in v {
                run_ast(world, i);
            }
        }
        InstructionAst::Repeat(i, n) => {
            for _ in 0..*n {
                run_ast(world, i);
            }
        }
    }
}