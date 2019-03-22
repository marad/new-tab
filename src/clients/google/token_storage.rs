use super::private_model::Token;
use serde_json;
use std::error;
use std::fs;

pub trait TokenStorage: std::marker::Sync + Clone {
    fn get_token(&self) -> Result<Token, Box<error::Error>>;
    fn set_token(&mut self, t: &Token) -> Result<(), Box<error::Error>>;
}

////////////////////////////////////////////////////////////////////////////////
// InMemoryStorage

#[derive(Default, Debug, Clone)]
pub struct InMemoryStorage {
    token: Option<Token>,
}

impl InMemoryStorage {
    #[warn(dead_code)]
    pub fn new() -> Self {
        Self { token: None }
    }
}

impl TokenStorage for InMemoryStorage {
    fn get_token(&self) -> Result<Token, Box<error::Error>> {
        match self.token.clone() {
            Some(t) => Ok(t),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Not set",
            ))),
        }
    }

    fn set_token(&mut self, t: &Token) -> Result<(), Box<error::Error>> {
        self.token = Some((*t).clone());
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// DiskStorage

#[derive(Default, Debug, Clone)]
pub struct DiskStorage {
    file_path: String,
}

impl DiskStorage {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl TokenStorage for DiskStorage {
    fn get_token(&self) -> Result<Token, Box<error::Error>> {
        let data = fs::read_to_string(&self.file_path)?;
        let token: Token = serde_json::from_str(&data)?;
        Ok(token)
    }

    fn set_token(&mut self, token: &Token) -> Result<(), Box<error::Error>> {
        let data = serde_json::to_string(token)?;
        fs::write(&self.file_path, &data)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_in_memory_storage() {
        // given the storage and a token
        let mut storage = InMemoryStorage::new();
        let token = Token::new("access-token".to_string(), None);

        // when updating token
        storage.set_token(&token).unwrap();

        // then token should be available
        let retrieved_token = storage.get_token().unwrap();
        assert_eq!(retrieved_token, token);
    }

    #[test]
    fn test_disk_storage() {
        // given the storage and a token
        let mut storage = DiskStorage::new("test_resources/google_tokens.json".to_owned());
        let token = Token::new(
            "access-token".to_string(),
            Some("refresh-token".to_string()),
        );

        // when updating token
        storage.set_token(&token).unwrap();

        // then token should be available
        let retrieved_token = storage.get_token().unwrap();
        assert_eq!(retrieved_token, token)
    }

}
