mod cli;
mod search;
mod embed;

use clap::Parser;
use cli::{Args};
use search::search_files;

fn main() {
    let args = Args::parse();
    search_files(&args);
}
