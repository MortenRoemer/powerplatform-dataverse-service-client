use std::time::Duration;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::{client_secret::ClientSecretAuth, Authenticate},
    batch::Batch,
    entity::{ReadableEntity, WritableEntity},
    error::DataverseError,
    query::Query,
    reference::Reference,
    result::{IntoDataverseResult, Result},
};

lazy_static! {
    static ref UUID_REGEX: Regex =
        Regex::new("[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
            .unwrap();
}

pub static VERSION: &str = "9.2";

pub struct Client<A: Authenticate> {
    pub url: &'static str,
    backend: reqwest::Client,
    auth: A,
}

impl Client<ClientSecretAuth> {
    pub fn with_client_secret_auth(
        url: &'static str,
        tenant_id: &'static str,
        client_id: String,
        client_secret: String,
    ) -> Self {
        let client = reqwest::Client::builder()
            .https_only(true)
            .connect_timeout(Duration::from_secs(120))
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap();

        let auth = ClientSecretAuth::new(
            client.clone(),
            format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                tenant_id
            ),
            format!("{}.default", url),
            client_id,
            client_secret,
        );

        Client::new(url, client, auth)
    }
}

impl<A: Authenticate> Client<A> {
    pub fn new(url: &'static str, backend: reqwest::Client, auth: A) -> Self {
        Self { url, backend, auth }
    }

    pub async fn create(&self, entity: &impl WritableEntity) -> Result<Uuid> {
        let token = self.auth.get_valid_token().await?;
        let reference = entity.get_reference();
        let url_path = self.build_simple_url(reference.entity_name);

        let response = self
            .backend
            .post(url_path)
            .bearer_auth(token)
            .header("OData-MaxVersion", "4.0")
            .header("OData-Version", "4.0")
            .header("Content-Type", "application/json; charset=utf-8")
            .header("Accept", "application/json")
            .body(serde_json::to_vec(entity).into_dataverse_result()?)
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

        let header_value = response
            .headers()
            .get("OData-EntityId")
            .ok_or_else(|| DataverseError::new("Dataverse provided no Uuid".to_string()))?;

        let uuid_segment = UUID_REGEX
            .find(header_value.to_str().unwrap_or(""))
            .ok_or_else(|| DataverseError::new("Dataverse provided no Uuid".to_string()))?;

        Uuid::parse_str(uuid_segment.as_str()).into_dataverse_result()
    }

    pub async fn update(&self, entity: &impl WritableEntity) -> Result<()> {
        let token = self.auth.get_valid_token().await?;
        let reference = entity.get_reference();
        let url_path = self.build_targeted_url(reference.entity_name, reference.entity_id);

        let response = self
            .backend
            .patch(url_path)
            .bearer_auth(token)
            .header("OData-MaxVersion", "4.0")
            .header("OData-Version", "4.0")
            .header("Content-Type", "application/json; charset=utf-8")
            .header("If-Match", "*")
            .body(serde_json::to_vec(entity).into_dataverse_result()?)
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

        Ok(())
    }

    pub async fn upsert(&self, entity: &impl WritableEntity) -> Result<()> {
        let token = self.auth.get_valid_token().await?;
        let reference = entity.get_reference();
        let url_path = self.build_targeted_url(reference.entity_name, reference.entity_id);

        let response = self
            .backend
            .patch(url_path)
            .bearer_auth(token)
            .header("OData-MaxVersion", "4.0")
            .header("OData-Version", "4.0")
            .header("Content-Type", "application/json; charset=utf-8")
            .body(serde_json::to_vec(entity).into_dataverse_result()?)
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

        Ok(())
    }

    pub async fn delete(&self, reference: &impl Reference) -> Result<()> {
        let token = self.auth.get_valid_token().await?;
        let reference = reference.get_reference();
        let url_path = self.build_targeted_url(reference.entity_name, reference.entity_id);

        let response = self
            .backend
            .delete(url_path)
            .bearer_auth(token)
            .header("OData-MaxVersion", "4.0")
            .header("OData-Version", "4.0")
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

        Ok(())
    }

    pub async fn retrieve<E: ReadableEntity>(&self, reference: &impl Reference) -> Result<E> {
        let token = self.auth.get_valid_token().await?;
        let reference = reference.get_reference();
        let columns = E::get_columns();
        let url_path = self.build_retrieve_url(reference.entity_name, reference.entity_id, columns);

        let response = self
            .backend
            .get(url_path)
            .bearer_auth(token)
            .header("OData-MaxVersion", "4.0")
            .header("OData-Version", "4.0")
            .header("Accept", "application/json")
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
        serde_json::from_slice(content.as_ref()).into_dataverse_result()
    }

    pub async fn retrieve_multiple<E: ReadableEntity>(&self, query: &Query) -> Result<Vec<E>> {
        let columns = E::get_columns();
        let mut url_path = Some(self.build_query_url(query.logical_name, columns, query));
        let mut entities = Vec::new();

        while url_path.is_some() {
            let response = self
                .backend
                .get(url_path.take().unwrap())
                .bearer_auth(self.auth.get_valid_token().await?.clone())
                .header("OData-MaxVersion", "4.0")
                .header("OData-Version", "4.0")
                .header("Accept", "application/json")
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
            let mut result_entities: EntityCollection<E> =
                serde_json::from_slice(content.as_ref()).into_dataverse_result()?;
            entities.append(&mut result_entities.value);
            url_path = result_entities.next_link
        }

        Ok(entities)
    }

    pub async fn execute(&self, batch: &Batch) -> Result<()> {
        let token = self.auth.get_valid_token().await?;
        let url_path = self.build_simple_url("$batch");

        let response = self
            .backend
            .post(url_path)
            .bearer_auth(token)
            .header("OData-MaxVersion", "4.0")
            .header("OData-Version", "4.0")
            .header(
                "Content-Type",
                format!("multipart/mixed; boundary=batch_{}", batch.get_batch_id()),
            )
            .header("Accept", "application/json")
            .body(batch.to_string())
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

        Ok(())
    }

    fn build_simple_url(&self, table_name: &str) -> String {
        format!("{}api/data/v{}/{}", self.url, VERSION, table_name)
    }

    fn build_targeted_url(&self, table_name: &str, target_id: Uuid) -> String {
        format!(
            "{}api/data/v{}/{}({})",
            self.url,
            VERSION,
            table_name,
            target_id.as_hyphenated()
        )
    }

    fn build_retrieve_url(&self, table_name: &str, target_id: Uuid, columns: &[&str]) -> String {
        let mut select = String::new();
        let mut comma_required = false;

        for column in columns {
            if comma_required {
                select.push(',');
            }

            select.push_str(column);
            comma_required = true;
        }

        format!(
            "{}api/data/v{}/{}({})?$select={}",
            self.url,
            VERSION,
            table_name,
            target_id.as_hyphenated(),
            select
        )
    }

    fn build_query_url(&self, table_name: &str, columns: &[&str], query: &Query) -> String {
        let mut select = String::new();
        let mut comma_required = false;

        for column in columns {
            if comma_required {
                select.push(',');
            }

            select.push_str(column);
            comma_required = true;
        }

        format!(
            "{}api/data/v{}/{}{}&$select={}",
            self.url, VERSION, table_name, query, select
        )
    }
}

#[derive(Deserialize)]
struct EntityCollection<E> {
    value: Vec<E>,
    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>,
}