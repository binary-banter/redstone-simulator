mod dot;
mod instructions;
pub mod runner;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Schematic file to use as input.
    #[arg(short, long)]
    input: String,

    /// Output file to use for producing a wave file.
    #[arg(short, long)]
    wave: Option<String>,

    /// Simulation to perform on the schematic for producing the wave file.
    #[arg(short, long)]
    simulation: Option<String>,

    /// Output file to use for producing a dot file.
    #[arg(short, long)]
    dot: Option<String>,
}
