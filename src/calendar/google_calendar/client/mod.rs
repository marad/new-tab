pub mod calendar;
mod private_model;
pub mod token_storage;

use private_model::Token;
use token_storage::TokenStorage;

use failure::{Error, Fallible};
use std::sync::{Arc, RwLock};

pub use token_storage::DiskStorage;
pub use token_storage::InMemoryStorage;

type AuthResult<T> = Fallible<T>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAuthConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone)]
pub struct GoogleClient<T: TokenStorage> {
    token_storage: Arc<RwLock<T>>,
    auth_config: GoogleAuthConfig,
}

#[derive(Debug, Fail)]
pub enum AuthError {
    #[fail(display = "Missing refresh token")]
    MissingRefreshToken,
    #[fail(display = "Error while fetching token")]
    TokneError,
}

impl<T: TokenStorage> GoogleClient<T> {
    pub fn new(token_storage: T, auth_config: GoogleAuthConfig) -> Self {
        GoogleClient {
            token_storage: Arc::new(RwLock::new(token_storage)),
            auth_config,
        }
    }

    pub fn get_access_token(&self, scopes: Vec<String>) -> AuthResult<Token> {
        let previous_token = { self.token_storage.read().unwrap().get_token() };

        let token = match previous_token {
            Err(_) => self.authenticate(&scopes),
            Ok(token) => self.refresh_token(&token, &scopes),
        };

        token.map(|t| {
            self.token_storage.write().unwrap().set_token(&t).unwrap(); // TODO: handle failure
            t
        })
    }

    fn authenticate(&self, scopes: &[String]) -> AuthResult<Token> {
        let oauth_config = self.get_oauth_config(scopes);
        let authenticator = oauth2_noserver::Authenticator::new(oauth_config);
        authenticator
            .authenticate()
            .map(|t| Token::new(t.access_token, t.refresh_token))
            .map_err(|_| Error::from(AuthError::TokneError))
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
                    Err(e) => Err(Error::from(e)),
                }
            }
            None => Err(Error::from(AuthError::MissingRefreshToken)),
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
