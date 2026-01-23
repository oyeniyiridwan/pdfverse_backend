use crate::{
    adapters::storage_adapter::StorageAdapter,
    utils::{
        config::StorageConfig, constant::MAX_PDF_SIZE, enums::StorageProvider, error::ApiError,
    },
};
use axum::extract::Multipart;
use bytes::Bytes;
use std::sync::Arc;
use uuid::Uuid;

pub trait StorageService {
    async fn upload_file(&self, multipart: Multipart) -> Result<(String, String, Bytes), ApiError>;
}

pub struct StorageServiceImpl<S: StorageAdapter + Send + Sync> {
    pub storage_adapter: Arc<S>,
}

impl<S: StorageAdapter + Send + Sync> StorageServiceImpl<S> {
    pub fn new(storage_adapter: Arc<S>) -> Self {
        Self { storage_adapter }
    }
}

impl<S: StorageAdapter + Send + Sync> StorageService for StorageServiceImpl<S> {
    async fn upload_file(
        &self,
        mut multipart: Multipart,
    ) -> Result<(String, String, Bytes), ApiError> {
        let mut file_name = None;
        let mut file_bytes = None;
        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|e| ApiError::BadGateway(format!("error:{e}")))?
        {
            let name = field
                .name()
                .ok_or(ApiError::BadRequest("invalid key".to_string()))?;
            if name == "file" {
                let main_file_name = field
                    .file_name()
                    .ok_or(ApiError::BadRequest("invalid key".to_string()))?
                    .to_string();
                file_name = Some(main_file_name);
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::BadGateway(format!("error:{e}")))?;
                let max_size = MAX_PDF_SIZE;
                if bytes.len() > max_size {
                    return Err(ApiError::BadRequest(
                        "file too large (max 10MB)".to_string(),
                    ));
                }
                file_bytes = Some(bytes);
            }
        }
        let (file_name, file_bytes) = match (file_name, file_bytes) {
            (Some(name), Some(bytes)) => (name, bytes),

            _ => {
                return Err(ApiError::BadGateway("Invalid type".to_string()));
            }
        };
        let storage = StorageConfig::new();
        let bucket = storage.bucket;
        println!(
            "storage info {},{},{}",
            &bucket,
            &file_name,
            file_bytes.len()
        );
        let file_path = format!("upload/{}-{}", Uuid::new_v4(), file_name);
        self.storage_adapter
            .upload_file(&bucket, &file_path, file_bytes.clone())
            .await?;

        let file_path = if matches!(storage.provider, StorageProvider::Supabase) {
            let base_endpoint = storage
                .endpoint
                .strip_suffix("/s3/")
                .unwrap_or(&storage.endpoint);
            format!("{}/object/public/{}/{}", base_endpoint, bucket, file_path)
        } else {
            format!("{}/{}/{}", storage.endpoint, bucket, file_path)
        };

        Ok((file_name, file_path, file_bytes))
    }
}
