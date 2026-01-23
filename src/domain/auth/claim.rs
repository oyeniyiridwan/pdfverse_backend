use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Claim {
    pub sub: Uuid,
    pub exp: Option<usize>,
    pub is_refresh_token: bool
}
