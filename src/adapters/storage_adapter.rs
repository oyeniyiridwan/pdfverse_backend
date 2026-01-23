use bytes::Bytes;
use aws_sdk_s3::{Client, primitives::ByteStream};
use crate::utils::error::ApiError;

pub trait StorageAdapter {
    async fn upload_file(
        &self,
        bucket_name: &str,
        file_name: &str,
        file_bytes: Bytes,
    ) -> Result<String, ApiError>;
}

pub struct StorageAdapterImpl {
    pub s3_client: Client,
}

impl StorageAdapterImpl {
    pub fn new(s3_client: Client) -> Self {
        Self { s3_client }
    }
}

impl StorageAdapter for StorageAdapterImpl {
    async fn upload_file(
        &self,
        bucket_name: &str,
        file_name: &str,
        file_bytes: Bytes,
    ) -> Result<String, ApiError> {
        let body = ByteStream::from(file_bytes.to_vec());
        let _ = self
            .s3_client
            .put_object()
            .bucket(bucket_name)
            .key(file_name)
            .body(body)
            .content_length(file_bytes.len() as i64)
            .send()
            .await
            .map_err(|e| {
                println!("error is :{:?}",e.raw_response());
                return ApiError::internal_msg(format!("StorageAdapter error: {}",e));})?;
      
        
        Ok("file uploaded successfully".to_string())
    }
}
