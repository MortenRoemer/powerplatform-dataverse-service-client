use serde::{Serialize, ser::SerializeMap};
use uuid::Uuid;

/// Represents a request to execute the Merge action in Dataverse
#[derive(Debug, Serialize)]
pub struct MergeRequest<'a> {
    #[serde(rename = "Target")]
    pub target: EntityReference<'a>,
    #[serde(rename = "Subordinate")]
    pub subordinate: EntityReference<'a>,
    #[serde(rename = "PerformParentingChecks")]
    pub check_parents: bool,
}

impl<'a> MergeRequest<'a> {
    pub fn new(entity_name: &'a str, target: Uuid, subordinate: Uuid, check_parents: bool) -> Self {
        Self {
            target: EntityReference {
                entity_name,
                entity_id: target,
            },
            subordinate: EntityReference {
                entity_name,
                entity_id: subordinate,
            },
            check_parents
        }
    }
}

#[derive(Debug)]
pub struct EntityReference<'a> {
    pub entity_name: &'a str,
    pub entity_id: Uuid,
}

impl<'a> Serialize for EntityReference<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("@odata.type", &format!("Microsoft.Dynamics.CRM.{}", self.entity_name))?;
        map.serialize_entry(&format!("{}id", self.entity_id), self.entity_id.as_hyphenated())?;
        map.end()
    }
}