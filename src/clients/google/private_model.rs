#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl Token {
    pub fn new(access_token: String, refresh_token: Option<String>) -> Self {
        Token { access_token, refresh_token }
    }
}

