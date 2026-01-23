use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, Config, config::Credentials};

use crate::utils::{config::StorageConfig, enums::StorageProvider};

pub  fn get_s3_client()->Client{
    let storage_config = StorageConfig::new();
   
    let  builder = Config::builder().behavior_version(BehaviorVersion::v2025_08_07())
     .region(Region::new(storage_config.region)).endpoint_url(storage_config.endpoint)
     .force_path_style(true)
    .credentials_provider(Credentials::new(storage_config.access_key.clone(), storage_config.secret_key.clone(),
     None, None, "supabase"));
        Client::from_conf(builder.build())
}