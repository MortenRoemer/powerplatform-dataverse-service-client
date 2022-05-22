use uuid::Uuid;

pub trait Reference {
    fn get_reference(&self) -> ReferenceStruct;
}

#[derive(Copy, Clone)]
pub struct ReferenceStruct {
    pub entity_name: &'static str,
    pub entity_id: Uuid,
}

impl ReferenceStruct {
    pub fn new(entity_name: &'static str, entity_id: Uuid) -> Self {
        Self {
            entity_name,
            entity_id,
        }
    }
}

impl Reference for ReferenceStruct {
    fn get_reference(&self) -> ReferenceStruct {
        *self
    }
}
