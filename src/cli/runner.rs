use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use vcd::{IdCode, SimulationCommand, TimescaleUnit};
use crate::cli::instructions::{Instruction, InstructionAst, parse};
use crate::world::World;
use crate::cli::Args;
use crate::cli::dot::write_dot;

pub fn run(args: Args){
    let mut world = World::from(File::open(args.input).expect("Could not open schematic file."));

    if let Some(dot) = args.dot {
        write_dot(&world.cblocks, &world.cblock_positions, Path::new(&dot));
    }

    if let Some(w) = args.wave{
        let Some(simulation) = args.simulation else {
            panic!("No simulation program was provided using the simulation flag!");
        };

        let Some(ast) = parse(&simulation) else {
            panic!("Could not parse simulation program provided!");
        };

        // Write the header
        let w = File::create(w).expect("Could not open wave file.");
        let mut w = vcd::Writer::new(w);
        w.timescale(100, TimescaleUnit::MS).unwrap();
        w.add_module("top").unwrap();
        let mut probe_indices: HashMap<String, IdCode> = HashMap::new();
        for (probe, _) in world.get_probes() {
            let id = w.add_wire(1, probe).unwrap();
            probe_indices.insert(probe.to_string(), id);
        }
        w.upscope().unwrap();
        w.enddefinitions().unwrap();

        // Write the initial values
        w.begin(SimulationCommand::Dumpvars).unwrap();
        for (probe, v) in world.get_probes() {
            w.change_scalar(probe_indices[probe], v).unwrap();
        }
        w.end().unwrap();

        // Write the data values
        let mut i = 0;
        run_ast(&mut world, &ast, &mut |probes| {
            i += 1;
            w.timestamp(i).unwrap();
            for (probe, v) in probes {
                w.change_scalar(probe_indices[probe], v).unwrap();
            }
        });
    }
}

fn run_ast(world: &mut World, ast: &InstructionAst, write: &mut impl FnMut(HashMap<&str, bool>)) {
    match ast {
        InstructionAst::Instruction(Instruction::Trigger) => {
            world.step_with_trigger();
            write(world.get_probes());
        }
        InstructionAst::Instruction(Instruction::Step) => {
            world.step();
            write(world.get_probes());
        }
        InstructionAst::Sequence(v) => {
            for i in v {
                run_ast(world, i, write);
            }
        }
        InstructionAst::Repeat(i, n) => {
            for _ in 0..*n {
                run_ast(world, i, write);
            }
        }
    }
}