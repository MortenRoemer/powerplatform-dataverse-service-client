use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/**
trait for getting a reference to an entity record from a struct
*/
pub trait Reference {
    /// creates a Reference structure for association purposes in Microsoft Dataverse
    fn get_reference(&self) -> ReferenceStruct;
}

/**
default implementation for the `Reference` trait
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceStruct {
    pub entity_name: &'static str,
    pub entity_id: Uuid,
}

impl ReferenceStruct {
    /// creates a new Reference struct
    pub fn new(entity_name: &'static str, entity_id: Uuid) -> Self {
        Self {
            entity_name,
            entity_id,
        }
    }
}

impl Display for ReferenceStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:({})",
            self.entity_name,
            self.entity_id.as_simple()
        ))
    }
}

impl Reference for ReferenceStruct {
    fn get_reference(&self) -> ReferenceStruct {
        *self
    }
}
