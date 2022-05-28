use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
};

use async_trait::async_trait;
use serde::Deserialize;
use tokio::sync::Mutex;

use super::Authenticate;
use crate::{
    error::DataverseError,
    result::{IntoDataverseResult, Result},
};

/**
Implements the `Authenticate` trait by using OAuth client/secret authentication

It is unlikely you need to use this struct directly. just use the
`Client::with_client_secret_auth(...)` function instead
*/
pub struct ClientSecretAuth {
    http_client: reqwest::Client,
    login_url: String,
    login_data: HashMap<&'static str, String>,
    token_info: Mutex<Option<TokenInfo>>,
}

impl ClientSecretAuth {
    /**
    Creates a new instance for client/secret based authentication

    It is unlikely you need to use this function directly. just use the
    `Client::with_client_secret_auth(...)` function instead
    */
    pub fn new(
        http_client: reqwest::Client,
        login_url: String,
        scope: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            http_client,
            login_url,
            login_data: build_login_data(client_id, client_secret, scope),
            token_info: Mutex::new(None),
        }
    }
}

#[async_trait]
impl Authenticate for ClientSecretAuth {
    async fn get_valid_token(&self) -> Result<Arc<String>> {
        let mut token_info = self.token_info.lock().await;

        if let Some(info) = token_info.as_ref() {
            if info.valid_until > SystemTime::now() {
                return Ok(Arc::clone(&info.key));
            }
        }

        let response = self
            .http_client
            .post(&self.login_url)
            .form(&self.login_data)
            .send()
            .await
            .into_dataverse_result()?;

        if response.status().is_client_error() || response.status().is_server_error() {
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("no error details provided from server"));
            return Err(DataverseError::new(error_message));
        }

        let content = response.bytes().await.into_dataverse_result()?;
        let mut result: TokenResult =
            serde_json::from_slice(content.as_ref()).into_dataverse_result()?;
        let key = Arc::from(result.access_token.take().unwrap());

        *token_info = Some(TokenInfo {
            key: Arc::clone(&key),
            valid_until: SystemTime::now() + Duration::from_secs(900),
        });

        Ok(key)
    }
}

fn build_login_data(
    client_id: String,
    client_secret: String,
    scope: String,
) -> HashMap<&'static str, String> {
    let mut form_data = HashMap::new();
    form_data.insert("grant_type", String::from("client_credentials"));
    form_data.insert("client_id", client_id);
    form_data.insert("client_secret", client_secret);
    form_data.insert("scope", scope);
    form_data
}

struct TokenInfo {
    key: Arc<String>,
    valid_until: SystemTime,
}

#[derive(Deserialize)]
struct TokenResult {
    pub access_token: Option<String>,
}
