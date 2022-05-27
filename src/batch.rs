use std::fmt::{Write, Display};

use uuid::Uuid;

use crate::{entity::WritableEntity, reference::Reference, result::{Result, IntoDataverseResult}, client::VERSION};

pub struct Batch {
    url: &'static str,
    batch_id: Uuid,
    dataset_id: Uuid,
    payload: String,
    next_content_id: u16,
}

impl Batch {
    pub fn new(url: &'static str) -> Self {
        Self {
            url,
            batch_id: Uuid::new_v4(),
            dataset_id: Uuid::new_v4(),
            payload: String::new(),
            next_content_id: 1,
        }
    }

    pub fn reset(&mut self) {
        self.batch_id = Uuid::new_v4();
        self.dataset_id = Uuid::new_v4();
        self.payload.clear();
        self.next_content_id = 1;
    }

    pub fn get_batch_id(&self) -> Uuid {
        self.batch_id
    }

    pub fn get_dataset_id(&self) -> Uuid {
        self.dataset_id
    }

    pub fn get_count(&self) -> u16 {
        self.next_content_id - 1
    }

    pub fn create(&mut self, entity: &impl WritableEntity) -> Result<()> {
        let reference = entity.get_reference();
        let entity = serde_json::to_string(entity).into_dataverse_result()?;
        
        write!(
            self.payload, 
            "--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding:binary\nContent-Id: {}\n\nPOST {}api/data/v{}/{} HTTP/1.1\nContent-Type: application/json;type=entry\n\n{}\n", 
            self.dataset_id.as_simple(),
            self.next_content_id,
            self.url,
            VERSION,
            reference.entity_name,
            entity
        ).into_dataverse_result()?;

        self.next_content_id += 1;
        Ok(())
    }

    pub fn update(&mut self, entity: &impl WritableEntity) -> Result<()> {
        let reference = entity.get_reference();
        let entity = serde_json::to_string(entity).into_dataverse_result()?;
        
        write!(
            self.payload, 
            "--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding:binary\nContent-Id: {}\n\nPATCH {}api/data/v{}/{}({}) HTTP/1.1\nContent-Type: application/json;type=entry\nIf-Match: *\n\n{}\n", 
            self.dataset_id.as_simple(),
            self.next_content_id,
            self.url,
            VERSION,
            reference.entity_name,
            reference.entity_id,
            entity
        ).into_dataverse_result()?;

        self.next_content_id += 1;
        Ok(())
    }

    pub fn upsert(&mut self, entity: &impl WritableEntity) -> Result<()> {
        let reference = entity.get_reference();
        let entity = serde_json::to_string(entity).into_dataverse_result()?;
        
        write!(
            self.payload, 
            "--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding:binary\nContent-Id: {}\n\nPATCH {}api/data/v{}/{}({}) HTTP/1.1\nContent-Type: application/json;type=entry\n\n{}\n", 
            self.dataset_id.as_simple(),
            self.next_content_id,
            self.url,
            VERSION,
            reference.entity_name,
            reference.entity_id,
            entity
        ).into_dataverse_result()?;

        self.next_content_id += 1;
        Ok(())
    }

    pub fn delete(&mut self, entity: &impl Reference) -> Result<()> {
        let reference = entity.get_reference();
        
        write!(
            self.payload, 
            "--changeset_{}\nContent-Type: application/http\nContent-Transfer-Encoding:binary\nContent-Id: {}\n\nDELETE {}api/data/v{}/{}({}) HTTP/1.1\n\n", 
            self.dataset_id.as_simple(),
            self.next_content_id,
            self.url,
            VERSION,
            reference.entity_name,
            reference.entity_id
        ).into_dataverse_result()?;

        self.next_content_id += 1;
        Ok(())
    }
}

impl Display for Batch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let batch_id = self.batch_id.as_simple();
        let dataset_id = self.dataset_id.as_simple();
        
        f.write_fmt(
            format_args!(
                "--batch_{}\nContent-Type: multipart/mixed; boundary=changeset_{}\n\n{}--changeset_{}--\n--batch_{}--",
                batch_id,
                dataset_id,
                self.payload,
                dataset_id,
                batch_id,
            )
        )
    }
}