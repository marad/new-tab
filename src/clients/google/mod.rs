pub mod calendar;
mod private_model;
pub mod token_storage;

use private_model::Token;
use token_storage::TokenStorage;

use std::error;
use std::fmt;
use std::cell::RefCell;

type AuthResult<T> = Result<T, Box<error::Error>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAuthConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone)]
pub struct GoogleClient<T: TokenStorage> {
    token_storage: RefCell<T>,
    auth_config: GoogleAuthConfig,
}

#[derive(Debug)]
pub struct AuthError {
    details: String,
}

impl AuthError {
    fn new(msg: &str) -> AuthError {
        AuthError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for AuthError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl<T:TokenStorage> GoogleClient<T> {
    pub fn new(token_storage: T, auth_config: GoogleAuthConfig) -> Self {
        GoogleClient {
            token_storage: RefCell::new(token_storage),
            auth_config,
        }
    }

    pub fn get_access_token(&self, scopes: Vec<String>) -> AuthResult<Token> {
        let previous_token = {
            self.token_storage.borrow_mut().get_token()
        };

        let token = match previous_token {
            Err(_) => self.authenticate(&scopes),
            Ok(token) => self.refresh_token(&token, &scopes),
        };

        token.map(|t| {
            self.token_storage.borrow_mut().set_token(&t).unwrap(); // TODO: handle failure
            t
        })
    }

    fn authenticate(&self, scopes: &[String]) -> AuthResult<Token> {
        let oauth_config = self.get_oauth_config(scopes);
        let authenticator = oauth2_noserver::Authenticator::new(oauth_config);
        authenticator
            .authenticate()
            .map(|t| Token::new(t.access_token, t.refresh_token))
    }

    fn refresh_token(&self, token: &Token, scopes: &[String]) -> AuthResult<Token> {
        let oauth_config = self.get_oauth_config(scopes);
        match &token.refresh_token {
            Some(refresh_token) => {
                match oauth_config.exchange_refresh_token(refresh_token.clone()) {
                    Ok(response) => Ok(Token::new(
                        response.access_token,
                        Some(refresh_token.clone()),
                    )),
                    Err(e) => Err(Box::new(e)),
                }
            }
            None => Err(Box::new(AuthError::new("Missing refresh token"))),
        }
    }

    fn get_oauth_config(&self, scopes: &[String]) -> oauth2_noserver::Config {
        let mut oauth_config = oauth2_noserver::Config::new(
            self.auth_config.client_id.clone(),
            self.auth_config.client_secret.clone(),
            "https://accounts.google.com/o/oauth2/v2/auth",
            "https://www.googleapis.com/oauth2/v3/token",
        );

        for scope in scopes {
            oauth_config = oauth_config.add_scope(scope.to_string());
        }

        oauth_config
    }
}
