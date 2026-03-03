// TODO: cooking minigame uses a strange format, I have to check with a real save file...

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use crate::common::GameId;

#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[repr(i32)]
#[derive(Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[serde(from = "i32", into = "i32")]
pub enum CookingGameplayModifiers {
    NoMods = 0,
    SlowMode = 1,
    FastMode = 2,
    UltraFastTimeMode = 4,
    InsaneFastTimeMode = 8,
    AllSpeedMods = 15,
    Practice = 16,
    AutoPlay = 32,
    // TODO: What? OtherMods = 16,
    #[num_enum(catch_all)]
    /// An unknown variant! Please let the developer know if this value ever pops up,
    /// you can safely unwrap, failing fast is probably the best solution
    Unknown(i32),
}

/// Score data from a single cooking gameplay run
///
/// The C# type is `CookingGameplayScoreData` (TypeDefIndex: 997)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct CookingGameplayScoreData {
    /// Number of "Great" hits
    #[serde(rename = "GreatCount", default)]
    pub great_count: i32,
    /// Number of "Good" hits
    #[serde(rename = "GoodCount", default)]
    pub good_count: i32,
    /// Number of "Bad" hits
    #[serde(rename = "BadCount", default)]
    pub bad_count: i32,
    /// Number of missed notes
    #[serde(rename = "MissCount", default)]
    pub miss_count: i32,
    /// Longest combo streak
    #[serde(rename = "MaxCombo", default)]
    pub max_combo: i32,
    /// Raw score value
    #[serde(rename = "RawScore", default)]
    pub raw_score: f64,
    /// Active gameplay modifier
    #[serde(rename = "ActiveModifiers")]
    pub active_modifiers: CookingGameplayModifiers,
    /// Accuracy score (0.0 – 1.0)
    #[serde(rename = "AccuracyScore", default)]
    pub accuracy_score: f64,
    /// Version of the song when this score was recorded
    #[serde(rename = "SongVersion", default)]
    pub song_version: i32,
    /// Whether the player finished the full song
    #[serde(rename = "IsComplete", default)]
    pub is_complete: bool,
}

/// Per-song save data for the cooking minigame
///
/// The C# type is `CookingSongSave` (TypeDefIndex: 1016)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct CookingSongSave {
    /// Unique song identifier
    #[serde(rename = "_key")]
    pub key: GameId,
    /// Top scores for this song (up to 3 remembered)
    #[serde(rename = "_allScores", default)]
    pub all_scores: Vec<CookingGameplayScoreData>,
    /// Total number of times this song was started
    #[serde(rename = "TimesPlayed", default)]
    pub times_played: i32,
    /// Total number of times this song was finished
    #[serde(rename = "TimesFinished", default)]
    pub times_finished: i32,
}

/// State of the cooking minigame
///
/// The C# type is `CookingMinigameManager` (TypeDefIndex: 1014)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct CookingMinigameState {
    /// Per-song save data
    pub saves: serde_json::Value,
}
