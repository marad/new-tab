pub mod token_storage;
mod private_model;

use token_storage::TokenStorage;
use private_model::Token;

use std::error;
use std::fmt;

type AuthResult<T> = Result<T, Box<error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleAuthConfig {
    pub client_id: String,
    pub client_secret: String,
}

pub struct GoogleClient {
    token_storage: Box<dyn TokenStorage>,
    auth_config: GoogleAuthConfig,
}

#[derive(Debug)]
pub struct AuthError {
    details: String,
}

impl AuthError {
    fn new(msg: &str) -> AuthError {
        AuthError { details: msg.to_string() }
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


impl GoogleClient {
    pub fn new(token_storage: Box<dyn TokenStorage>, auth_config: GoogleAuthConfig) -> Self {
        GoogleClient {
            token_storage,
            auth_config,
        }
    }

    pub fn get_access_token(&mut self, scopes: Vec<String>) -> AuthResult<Token> {
        let previous_token = dbg!(self.token_storage.get_token());

        let token = match previous_token {
            Err(_) => self.authenticate(&scopes),
            Ok(token) => self.refresh_token(&token, &scopes),
        };

        token.map(|t| {
            self.token_storage.set_token(&t).unwrap(); // TODO: handle failure
            t
        })
    }

    fn authenticate(&self, scopes: &Vec<String>) -> AuthResult<Token> {
        let oauth_config = self.get_oauth_config(scopes);
        let authenticator = oauth2_noserver::Authenticator::new(oauth_config);
        authenticator.authenticate()
            .map(|t| Token::new( t.access_token, t.refresh_token ))
    }

    fn refresh_token(&self, token: &Token, scopes: &Vec<String>) -> AuthResult<Token> {
        let oauth_config = self.get_oauth_config(scopes);
        match &token.refresh_token {
            Some(refresh_token) => {
                match oauth_config.exchange_refresh_token(refresh_token.clone()) {
                    Ok(response) => Ok(Token::new(response.access_token, Some(refresh_token.clone()))),
                    Err(e) => Err(Box::new(e))
                }

            },
            None => Err(Box::new(AuthError::new("Missing refresh token")))
        }
    }

    fn get_oauth_config(&self, scopes: &Vec<String>) -> oauth2_noserver::Config {
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


