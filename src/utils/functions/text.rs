
use regex::Regex;

pub fn clean_text(raw: &str) -> String {
    let white_space = Regex::new(r"\s+").unwrap();
    white_space.replace_all(raw, " ").trim().replace('\0', "").to_string()
}

pub fn chunk_text(text: String, chunk_size: usize, overlap: usize) -> Vec<String> {
    let words = text.split_whitespace().collect::<Vec<&str>>();
    let mut chunks: Vec<String> = Vec::new();
    let mut start = 0;
    while start < words.len() {
        let end = (start + chunk_size).min(words.len());
        let chunk = words[start..end].join(" ");
        chunks.push(chunk);
        if end == words.len() {
            break;
        }
        start +=chunk_size - overlap;
    }
  chunks
}
