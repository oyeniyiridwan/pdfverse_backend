use bytes::Bytes;
use sea_orm::sqlx::error::BoxDynError;

use crate::utils::{constant::{CHUNK_SIZE, OVERLAP}, functions::{pdf::extract_from_bytes, text::{chunk_text, clean_text}}};


pub fn process_pdf(bytes:Bytes)->Result<Vec<String>,BoxDynError>{
let raw_text =extract_from_bytes(bytes)?;
let text = clean_text(&raw_text);
let chunks = chunk_text(text, CHUNK_SIZE, OVERLAP);
    Ok(chunks)
}