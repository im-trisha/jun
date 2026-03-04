use serde::{Deserialize, Serialize};

use crate::common::SerializableGuid;

#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct ModInfo {
    #[serde(rename = "ModGuid")]
    pub guid: SerializableGuid,
    #[serde(rename = "ModName")]
    pub name: String,
}
