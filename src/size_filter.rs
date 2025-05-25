fn parse_size(size_str: &str) -> Option<u64>{
    let lower = size_str.trim().to_lowercase();
    let (num_part, suffix) = lower
        .trim_end_matches(|c: char| c.is_alphabetic() == false)
        .split_at(lower.find(|c: char| c.is_alphabetic())?);
    let value: f64 = num_part.trim().parse().ok()?;

    let multiplier = match suffix.trim() {
        "b" => 1.0,
        "kb" => 1024.0,
        "mb" => 1024.0 * 1024.0,
        "gb" => 1024.0 * 1024.0 * 1024.0,
        _ => panic!("Size needs to be B, KB, MB, or GB in either lowercase or uppercase")
    };
    Some((value * multiplier) as u64)
}