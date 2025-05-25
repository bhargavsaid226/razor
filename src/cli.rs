use clap::{Parser, ValueEnum};


#[derive(Parser, Debug)]
#[command(name = "razor")]
pub struct Args{
    // NLP Query
    pub query: String,

    // Search mode: name, content, or both
    #[arg(long, default_value = "name")]
    pub mode: SearchMode,

    // Main Directory to search
    #[arg(short, long, default_value=".")]
    pub dir: String,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum SearchMode{
    Name,
    Content,
}