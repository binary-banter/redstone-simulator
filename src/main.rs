use clap::Parser;
use crate::cli::Args;
use crate::cli::runner::run;

mod blocks;
mod cli;
mod world;

fn main() {
    run(Args::parse())
}
