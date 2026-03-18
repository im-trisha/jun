use serde::{Deserialize, Serialize};

use crate::common::GameId;

/// Per-fish save data recording catch statistics
///
/// The C# type is `FishSave` (`TypeDefIndex`: 1116)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct FishSave {
    /// Unique fish identifier
    #[serde(rename = "_key")]
    pub key: GameId,
    /// Total number of times this fish has been caught
    #[serde(rename = "_timesCaught", default)]
    pub times_caught: i32,
    /// Lightest recorded catch weight
    #[serde(rename = "_minWeightCaught", default)]
    pub min_weight_caught: f32,
    /// Heaviest recorded catch weight
    #[serde(rename = "_maxWeightCaught", default)]
    pub max_weight_caught: f32,
    /// Id for the location where this fish was last caught
    #[serde(rename = "_lastCaughtLocationDataKey")]
    pub last_caught_location_data_key: GameId,
}

/// State of the fishing minigame
///
/// The C# type is `FishingMinigameManager` (`TypeDefIndex`: 1114)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct FishingMinigameState {
    /// All per-fish catch records
    #[serde(rename = "_serializedSaves", default)]
    pub saves: Vec<FishSave>,
    /// Seed for the daily fishing tip
    #[serde(rename = "_fishingTipSeed", default)]
    pub fishing_tip_seed: i32,
}
