mod cli;
mod search;
mod embed;
mod date_filter;
mod size_filter;

use clap::Parser;
use cli::{Args};
use search::search_files;
use env_logger;


fn main() {
    env_logger::init();
    let args = Args::parse();
    search_files(&args);
}
