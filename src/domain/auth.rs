use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone,Copy)]
pub enum Provider {
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "linkedin")]
    LinkedIn,
    #[serde(rename = "signup")]
    Signup,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "magiclink")]
    MagicLink,
}






impl ToString for Provider {
    fn to_string(&self) -> String {
        match *self {
            Provider::Google => "google".to_string(),
    Provider::MagicLink => "magiclink".to_string(),
            Provider::LinkedIn => "linkedin".to_string(),
            Provider::Signup => "signup".to_string(),
            Provider::Login => "login".to_string(),
        }
    }
}



