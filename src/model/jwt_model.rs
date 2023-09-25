use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Clone)]
pub struct AuthenticatedClaims {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtCustomClaims {
    pub user_id: i32,
}