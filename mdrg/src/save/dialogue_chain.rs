use serde::{Deserialize, Serialize};

use crate::common::GameId;

/// Records which dialogue chains a character has already played through,
/// preventing repetition
///
/// In the actual save file, this chain memory is mapped through a String-ChainMemory pair
/// I wasn't, however, able to detect a list of possible keys, so we can't get an enum unfortunately
///
/// The C# type is `DialogueChainMemory` (`TypeDefIndex`: 824)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct DialogueChainMemory {
    /// This is kept for backwards compatibility, it was used in place of [`DialogueChainMemory.ignored_game_ids`], now deprecated
    ///
    /// Confirmed by `ΩSheep`, the lead developer
    #[serde(rename = "<ignored>k__BackingField", default)]
    pub ignored: Vec<i32>,
    /// Current [`GameId`]-keyed ignored chains.
    #[serde(rename = "ignoredGameIds", default)]
    pub ignored_game_ids: Vec<GameId>,
}
