use crate::oauth;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppDataFromServer {
    id: String,
    name: String,
    #[serde(rename = "callbackUrl")]
    callback_url: Option<String>,
    permission: Vec<String>,
    pub secret: Option<String>,
    #[serde(rename = "isAuthorized")]
    is_authorized: Option<bool>,
}

impl Into<oauth::AppData> for AppDataFromServer {
    fn into(self) -> oauth::AppData {
        oauth::AppData::new(
            self.id,
            self.name,
            None,
            self.callback_url,
            "".to_string(),
            self.secret.unwrap(),
        )
    }
}

/// Obteined token data from server.
#[derive(Debug, Deserialize, Clone)]
pub struct TokenDataFromServer {
    #[serde(rename = "accessToken")]
    access_token: String,
}

impl Into<oauth::TokenData> for TokenDataFromServer {
    fn into(self) -> oauth::TokenData {
        oauth::TokenData::new(
            self.access_token,
            "Firefish".to_string(),
            None,
            None,
            None,
            None,
        )
    }
}
