use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::utils::error::ApiError;

#[derive(Serialize, Deserialize, Debug,Clone,Copy,PartialEq)]
pub enum Platform {
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "mobile")]
    Mobile

}

impl FromStr for Platform {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       if s =="web"{
        return Ok(Platform::Web)
       }
        if s =="mobile"{
        return Ok(Platform::Mobile)
       }
       return Err(ApiError::BadRequest("Invalid Platform".to_string()));
    }
}






// impl ToStr for Platform {
//     fn to_str(&self) -> String {
//         match *self {
//             Platform::Web => "web",
//     Platform::Mobile => "mobile",
           
//         }
//     }
// }