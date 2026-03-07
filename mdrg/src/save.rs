use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

pub use slot::MDRGSaveSlot;

mod dialogue_chain;
mod flag;
mod slot;

/// Different possible types of saves
#[repr(i32)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[serde(from = "i32", into = "i32")]
pub enum SaveType {
    /// A manual save slot
    Manual = 0,
    /// An auto save
    Auto = 1,
    #[num_enum(catch_all)]
    /// An unknown variant! Please let the developer know if this value ever pops up,
    /// you can safely unwrap, failing fast is probably the best solution
    Unknown(i32),
}

/// A MDRG save file, its the top-most structure in the entire crate
///
/// The C# type is `SaveContainer` (TypeDefIndex: 1779)
#[cfg_attr(feature = "derive-debug", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct MDRGSaveFile {
    /// The websites visited all across the save slots
    #[serde(rename = "visitedWebsites", default)]
    pub visited_websites: Vec<String>,
    /// The steam achievements done, now unused because the game is banned on steam
    pub achievements: serde_json::Value,
    /// Global flags
    // TODO: me ffs
    pub flags: serde_json::Value,
    pub saves: Vec<MDRGSaveRecord>,
    #[serde(rename = "autoSaves", default)]
    pub auto_saves: Vec<MDRGSaveRecord>,
    #[serde(rename = "nextAutoSaveIndex", default)]
    pub next_auto_save_index: i32,
}

#[cfg_attr(feature = "derive-debug", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct MDRGSaveRecord {
    pub notes: String,
    pub description: String,
    // TODO: change according to what sheep said
    #[serde(rename = "_time", default)]
    pub time: i64,
    #[serde(rename = "ingameTime")]
    pub ingame_time: i32,
    pub slot: i32,
    #[serde(rename = "savedata")]
    pub save_data: String, // For now...
    #[serde(rename = "_saveType")]
    pub save_type: SaveType,
}
