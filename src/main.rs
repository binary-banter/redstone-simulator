use crate::cli::runner::run;
use crate::cli::Args;
use clap::Parser;

mod blocks;
mod cli;
mod world;

fn main() {
    run(Args::parse())
}
