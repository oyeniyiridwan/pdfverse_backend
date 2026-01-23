pub fn extract_names(names: &Option<String>) -> (Option<String>, Option<String>) {
    match names {
        Some(names) => {
            let parts: Vec<&str> = names.split_whitespace().collect();
            if parts.is_empty() {
                (None, None)
            } else {
                if parts.len() > 1 {
                    (Some((parts[0]).to_string()), Some((parts[1]).to_string()))
                } else {
                    (Some((parts[0]).to_string()), None)
                }
            }
        }
        None => (None, None),
    }
}

pub fn option_pg_vector_from_option_string(value: Option<String>) -> Option<Vec<f32>> {
    value.map(|s| serde_json::from_str(&s).unwrap())
}

pub fn option_string_from_option_pg_vector(value: Option<Vec<f32>>) -> Option<String> {
    value.map(|v| serde_json::to_string(&v).unwrap())
}


pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (norm_a * norm_b)
}
