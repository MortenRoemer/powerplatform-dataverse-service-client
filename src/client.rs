use std::io;

use uuid::Uuid;

use crate::{entity::{Entity, EntityReference}, query::Query};

pub struct Client<'a> {
    pub url: &'a str,
}

impl<'a> Client<'a> {
    pub async fn create<'b>(&self, entity: Entity<'b>) -> Result<Uuid> {
        unimplemented!();
    }

    pub async fn update<'b>(&self, entity: Entity<'b>) -> Result<()> {
        unimplemented!();
    }

    pub async fn delete<'b>(&self, reference: EntityReference<'b>) -> Result<()> {
        unimplemented!();
    }

    pub async fn retrieve<'b>(&self, reference: EntityReference<'b>) -> Result<Entity<'b>> {
        unimplemented!();
    }

    pub async fn retrieve_multiple<'b>(&self, query: Query<'b>) -> Result<Vec<Entity<'b>>> {
        unimplemented!();
    }
}

pub type Result<T> = std::result::Result<T, DataverseError>;

pub enum DataverseError {
    IOError(io::Error)
}