use crate::utils::error::ApiError;
use  dotenvy::var;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "mobile")]
    Mobile,
}

impl FromStr for Platform {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "web" {
            return Ok(Platform::Web);
        }
        if s == "mobile" {
            return Ok(Platform::Mobile);
        }
        return Err(ApiError::BadRequest("Invalid Platform".to_string()));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum StorageProvider {
    #[serde(rename = "minio")]
    Minio,
    #[serde(rename = "supabase")]
    Supabase,
}

impl ToString for StorageProvider {
    fn to_string(&self) -> String {
        match *self {
            StorageProvider::Minio => "minio".to_string(),
            StorageProvider::Supabase => "supabase".to_string(),
        }
    }
}

impl StorageProvider {
    pub fn from_env() -> Self {
     match   var("STORAGE_PROVIDER").as_deref(){
            Ok("minio") => Self::Minio,
            Ok("supabase") => Self::Supabase,
            _ => Self::Minio, // default safe choice
        }
    }
}
