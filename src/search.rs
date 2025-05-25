use crate::cli::{SearchMode, Args};
use crate::embed::{cosine_similarity, embed_text};
use walkdir::WalkDir;
use std::fs;


pub fn search_files(args: &Args){
    let query_vec = embed_text(&args.query);

    for entry in WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let result_text = match args.mode{
                SearchMode::Name => path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                SearchMode::Content => fs::read_to_string(path).unwrap_or_default()
            };
        
        let file_vec = embed_text(&result_text);
        let score = cosine_similarity(&query_vec, &file_vec);
        
        if score > 0.7{
            println!("{:.2} -> {}", score, path.display());
        }
        //"Make Score configurable from CLI to let user define what value they want"

    }

}