//! Serde deserialization models for MDRG game save files.
//! Currently, only supports 0.95, in the future may support
//! older and newer versions
//!
//! Types are extracted from the IL2CPP disassembly
//! each module groups related C# classes
//!
//! As a sidenote, I don't know a lot about unity games RE,
//! neither I know if the TypeDefIndex is constant between
//! IL2CPP disassembly runs, but I kept it for at least helping myself
//! going back and forth through the dump code
//!
//! # Usage
//!
//! ```rust,no_run
//! let json = std::fs::read_to_string("save.mdrg").unwrap();
//! let save: mdrg::MDRGSaveFile = serde_json::from_str(&json).unwrap();
//! println!("{}", save.player_name);
//! ```

pub mod common;
pub mod email;
pub mod events;
pub mod items;
pub mod mods;
pub mod news;
pub mod save;
pub mod substates;

pub use save::MDRGSaveFile;
pub use save::MDRGSaveSlot;
