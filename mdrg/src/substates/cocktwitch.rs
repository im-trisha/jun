use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

/// Category of CockTwitch streamer
///
/// The C# type is `Streamer.StreamerType`
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[repr(i32)]
#[derive(Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[serde(from = "i32", into = "i32")]
pub enum StreamerType {
    /// Generic / variety streamer
    Generic = 0,
    /// Gay streamer
    Gay = 1,
    /// Gaming streamer
    Gaming = 2,
    /// Female streamer
    Female = 3,
    /// Furry streamer
    Furry = 4,
    /// Disgusting content streamer
    Disgusting = 5,
    /// Inactive streamer
    Frozen = 6,
    /// [XQC](https://en.wikipedia.org/wiki/XQc)
    Xqc = 7,
    #[num_enum(catch_all)]
    /// An unknown variant! Please let the developer know if this value ever pops up,
    /// you can safely unwrap, failing fast is probably the best solution
    Unknown(i32),
}

/// A CockTwitch streamer
///
/// The C# type is `Streamer` (TypeDefIndex: 1510)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct Streamer {
    /// Streamer display name
    #[serde(rename = "<Name>k__BackingField")]
    pub name: String,
    /// Earliest hour of the day (0-23) the streamer goes live
    #[serde(rename = "<EarliestHour>k__BackingField")]
    pub earliest_hour: i32,
    /// Latest hour of the day (0-23) the streamer goes live
    #[serde(rename = "<LatestHour>k__BackingField")]
    pub latest_hour: i32,
    /// Follower count
    #[serde(rename = "<Followers>k__BackingField")]
    pub followers: i32,
    /// Streamer category
    #[serde(rename = "<Type>k__BackingField")]
    pub streamer_type: StreamerType,
    /// Seed for deterministic content generation
    pub seed: i32,
}

/// State of the CockTwitch streaming platform
///
/// The C# type is `CockTwitchManager` (TypeDefIndex: 1515)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct CockTwitchState {
    /// All registered streamers on the platform
    #[serde(rename = "_streamers", default)]
    pub streamers: Vec<Streamer>,
    /// Follower count history
    #[serde(rename = "followerMemorySerialize", default)]
    pub follower_memory: Vec<i32>,
}
