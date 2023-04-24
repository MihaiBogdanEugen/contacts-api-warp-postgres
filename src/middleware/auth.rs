use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use async_trait::async_trait;
use base64::engine::general_purpose as base64Engine;
use base64::Engine;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;

use crate::models::errors::Error;

const API_USERS_FILE_KEY: &str = "API_USERS_FILE";
const DEFAULT_FILE: &str = "api_users.json";

#[async_trait]
trait AuthMiddleware {
    /// Checks if the authorization header assuming HTTP Basic Auth schema agains a given data store of users.
    async fn http_basic_auth(&self, auth_header: String) -> Result<bool, Error>;
}

struct AuthInMemoryMiddleware {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl AuthInMemoryMiddleware {
    async fn new() -> Self {
        let api_users_file_path: String =
            env::var(API_USERS_FILE_KEY).unwrap_or(DEFAULT_FILE.to_string());
        Self::new_with_file(api_users_file_path).await
    }

    async fn new_with_file(path: String) -> Self {
        let mut file: File = File::open(path).await.unwrap();
        let mut file_contents: Vec<u8> = vec![];
        file.read_to_end(&mut file_contents).await.unwrap();
        let existing_data: HashMap<String, String> =
            serde_json::from_slice(&file_contents).unwrap();
        Self::new_with_data(existing_data).await
    }

    async fn new_with_data(existing_data: HashMap<String, String>) -> Self {
        let new_data: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
        for (username, password) in existing_data.into_iter() {
            new_data.write().await.insert(username, password);
        }
        AuthInMemoryMiddleware { data: new_data }
    }

    fn get_credentials(input: String) -> Result<(String, String), Error> {
        let vec: Vec<u8> = base64Engine::STANDARD.decode(input)?;
        let decoded_credentials: String = String::from_utf8(vec)?;
        if let Some((username, password)) = decoded_credentials.split_once(':') {
            Ok((username.to_string(), password.to_string()))
        } else {
            Err(Error::InvalidAuthHeader)
        }
    }
}

#[async_trait]
impl AuthMiddleware for AuthInMemoryMiddleware {
    async fn http_basic_auth(&self, auth_header: String) -> Result<bool, Error> {
        if let Some((auth_type, encoded_credentials)) = auth_header.split_once(' ') {
            if encoded_credentials.contains(' ') {
                Err(Error::InvalidAuthHeader)
            } else if auth_type.to_lowercase() != "basic" {
                Err(Error::InvalidScheme(auth_type.to_string()))
            } else {
                let (username, password): (String, String) =
                    AuthInMemoryMiddleware::get_credentials(encoded_credentials.to_string())?;
                Ok(self.data.read().await.get(&username) == Some(&password))
            }
        } else {
            Err(Error::InvalidAuthHeader)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_basic_auth() {
        let key: String = "api_username".to_string();
        let value: String = "api_password".to_string();
        let mut existing_data: HashMap<String, String> = HashMap::new();
        existing_data.insert(key.clone(), value.clone());
        let auth_middleware: AuthInMemoryMiddleware =
            AuthInMemoryMiddleware::new_with_data(existing_data).await;

        let credentials: String = format!("{key}:{value}");
        let encoded_credentials: String = base64Engine::STANDARD.encode(credentials);
        let header_value = format!("Basic {encoded_credentials}");

        let actual_result: Result<bool, Error> =
            auth_middleware.http_basic_auth(header_value).await;
        assert!(actual_result.is_ok());
        assert_eq!(Some(true), actual_result.ok());
    }
}
