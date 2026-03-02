use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::GameId;

/// Records which dialogue chains a character has already played through,
/// preventing repetition
///
/// The C# type is `DialogueChainMemory` (TypeDefIndex: 824)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct DialogueChainMemory {
    /// This is kept for backwards compatibility, it was used in place of [DialogueChainMemory.ignored_game_ids], now deprecated
    ///
    /// Confirmed by ΩSheep, the lead developer
    #[serde(rename = "<ignored>k__BackingField", default)]
    pub ignored: Vec<i32>,
    /// Current [GameId]-keyed ignored chains.
    #[serde(rename = "ignoredGameIds", default)]
    pub ignored_game_ids: Vec<GameId>,
}

/// A proxy type containing a key-value pair of [String]->[DialogueChainMemory]
///
/// In the config files, the json is like such:
/// ```json
/// {
///     "keys": ["KeyA"]
///     "values": {dialogueRepresentingKeyA}
/// }
/// ```
/// So this has been mapped through this proxy class, having a more rusty interface
/// I wasn't, however, able to detect a list of possible keys, so we can't get an enum unfortunately
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(from = "DialogueChainMemoryProxy", into = "DialogueChainMemoryProxy")]
pub struct DialogueChains {
    pub memories: HashMap<String, DialogueChainMemory>,
}

#[derive(Serialize, Deserialize)]
struct DialogueChainMemoryProxy {
    keys: Vec<String>,
    values: Vec<DialogueChainMemory>,
}

impl From<DialogueChainMemoryProxy> for DialogueChains {
    fn from(proxy: DialogueChainMemoryProxy) -> Self {
        Self {
            memories: proxy.keys.into_iter().zip(proxy.values).collect(),
        }
    }
}

impl From<DialogueChains> for DialogueChainMemoryProxy {
    fn from(model: DialogueChains) -> Self {
        let (keys, values): (Vec<String>, Vec<DialogueChainMemory>) =
            model.memories.into_iter().unzip();

        Self { keys, values }
    }
}
