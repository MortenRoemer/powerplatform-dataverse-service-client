use serde::{Serialize, de::DeserializeOwned};

use crate::{reference::Reference, select::Select};

pub trait ReadableEntity: DeserializeOwned + Select { }

pub trait WritableEntity: Serialize + Reference { }