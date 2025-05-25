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

    // Filter files after a given date
    #[arg(long)]
    pub after: Option<String>,

    // Filter files before a given date
    #[arg(long)]
    pub before: Option<String>,

    // Filter files less than the size of this param
    #[arg(long)]
    pub max_size: Option<String>,

    // Filter files greater than the size of this param
    #[arg(long)]
    pub min_size: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum SearchMode{
    Name,
    Content,
}