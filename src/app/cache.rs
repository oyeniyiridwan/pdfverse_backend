use std::{sync::{Arc}, time::Instant};

use serde_json::Value;
use tokio::sync::RwLock;


#[derive(Clone)]
pub struct AppCache{
 pub  google: Arc<RwLock<Option<(Value, Instant)>>>,
   pub linkedin: Arc<RwLock<Option<(Value, Instant)>>>


}

impl AppCache{
    pub fn new(    google: Arc<RwLock<Option<(Value, Instant)>>>,
    linkedin: Arc<RwLock<Option<(Value, Instant)>>>

)->Self{
    Self { google, linkedin }
}
}