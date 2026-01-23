use std::fs;
use bytes::Bytes;
use sea_orm::sqlx::error::BoxDynError;
use tempfile::NamedTempFile;
use lopdf::Document;
use std::error::Error;
use pdf_oxide::{PdfDocument,pipeline::{MarkdownOutputConverter, TextPipeline, TextPipelineConfig}};

// pub fn extract_from_bytes(bytes: Bytes)->Result<String,BoxDynError>{
    // let  temp = NamedTempFile::new()?;
    // let path = temp.path();
    // fs::write(path, bytes)?;
//    let text = pdf_extract::extract_text(path)?;
// Ok(text)
// }


pub fn extract_from_bytes(bytes: Bytes) -> Result<String, BoxDynError> {
    let doc = Document::load_mem(&bytes)?;
    let text = doc.extract_text(&doc.get_pages().keys().cloned().collect::<Vec<_>>())?;
    Ok(text)
}






// pub fn extract_from_bytes(bytes: Bytes) -> Result<String,BoxDynError> {
//     let  temp = NamedTempFile::new()?;
//     let path = temp.path();
//         fs::write(path, bytes)?;

//     let mut doc = PdfDocument::open(path)?;
    
//     let mut full_text = String::new();


//     for page_num in 0..doc.page_count()? {
//             let text = doc.extract_text(page_num)?;

      
//         full_text.push_str(&text);
//         full_text.push('\n');
//     }
    
//     Ok(full_text)

// }
