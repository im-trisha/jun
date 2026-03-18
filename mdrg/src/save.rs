use nt_time::chrono::NaiveDateTime;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

pub use flag::StoryFlags;
pub use slot::MDRGSaveSlot;

mod dialogue_chain;
mod filetime_chrono;
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
/// The C# type is `SaveContainer` (`TypeDefIndex`: 1779)
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
    /// Manual saves
    pub saves: Vec<MDRGSaveRecord>,
    /// Automatic saves
    #[serde(rename = "autoSaves", default)]
    pub auto_saves: Vec<MDRGSaveRecord>,
    /// The slot number used by this save file's next autosave
    #[serde(rename = "nextAutoSaveIndex", default)]
    pub next_auto_save_index: i32,
}

#[cfg_attr(feature = "derive-debug", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct MDRGSaveRecord {
    /// The little note on this save record
    pub notes: String,
    // TODO: Is this notes but for backwards compatibility?
    pub description: String,
    /// The time when this savefile was last opened
    #[serde(with = "filetime_chrono", rename = "_time", default)]
    pub time: NaiveDateTime,
    /// In game time in minutes
    #[serde(rename = "ingameTime")]
    pub ingame_time: i32,
    /// The slot of this record
    pub slot: i32,
    /// The save type of this record, either autosave or manual
    #[serde(rename = "_saveType")]
    pub save_type: SaveType,
    #[serde(rename = "savedata")]
    save_data: String,
    #[serde(skip)]
    parsed_data: Option<MDRGSaveSlot>,
}

impl MDRGSaveRecord {
    /// A function that will give you the save data of this record.
    ///
    /// This function parses the internal `save_data` property into a json, and returns you that json
    /// If this function is called more than once, cache will be used, so you can call this function as much as you like
    ///
    /// # Errors
    ///
    /// This will return [`serde_json::Error`] if the parsing wasn't successful
    pub fn save_data(&mut self) -> Result<&mut MDRGSaveSlot, serde_json::Error> {
        if self.parsed_data.is_none() {
            self.parsed_data = Some(serde_json::from_str(&self.save_data)?);
        }

        Ok(self.parsed_data.as_mut().unwrap_or_else(|| unreachable!()))
    }

    /// A function that flushes data obtained from the [`Self::save_data`] function
    /// into the underlying `save_data` property, so the save file possessing this record
    /// will be ready to be saved
    ///
    /// # Errors
    ///
    /// This will return `serde_json::Error` if the jsonification wasn't successful
    pub fn flush_data(&mut self) -> Result<(), serde_json::Error> {
        // If its none, it was never assigned to, meaning it didn't even get edited
        let Some(data) = self.parsed_data.as_ref() else {
            return Ok(());
        };

        self.save_data = serde_json::to_string(data)?;

        Ok(())
    }
}
