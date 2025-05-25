pub fn embed_text(text: &str) -> Vec<f32> {
    // Basic Bag of alphabets Model
    // ToDo: Implement a Pre-Trained Context Aware Embedding Model
    let mut vec = vec![0.0; 26];
    for c in text.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            let idx = (c as u8  - b'a') as usize;
            vec[idx] += 1.0;
        }
    }

    normalize(&vec)

}

fn normalize(vec: &[f32]) -> Vec<f32> {
    let total  = norm(vec);
    if total == 0.0 {
        vec.to_vec()
    }
    else{
        vec.iter().map(|x| x / total).collect()
    }
}

fn norm(a: &[f32]) -> f32{
    a.iter().map(|x| x * x).sum::<f32>().sqrt()
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>()
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32{
    let similarity = dot(a, b);
    let norm_a: f32 = norm(a);
    let norm_b: f32 = norm(b);
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    }
    else{
        similarity / (norm_a * norm_b)
    }
}
