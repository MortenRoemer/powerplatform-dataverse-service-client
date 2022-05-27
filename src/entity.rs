use serde::{de::DeserializeOwned, Serialize};

use crate::{reference::Reference, select::Select};

pub trait ReadableEntity: DeserializeOwned + Select {}

pub trait WritableEntity: Serialize + Reference {}
