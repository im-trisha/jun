//! Story flags type
use serde::{Deserialize, Serialize};

/// A story flag set during gameplay
///
/// The C# type is `GameVariables.Flag` (TypeDefIndex: 1320)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct Flag {
    /// Identifier string of the flag
    pub name: String,
    /// In-game minute when the flag was most recently set
    #[serde(rename = "timeAdded")]
    pub time_added: i32,
    /// In-game minute when the flag was set for the very first time
    #[serde(rename = "firstTimeAdded")]
    pub first_time_added: i32,
    /// Total number of times this flag has been set
    pub times: i32,
}
