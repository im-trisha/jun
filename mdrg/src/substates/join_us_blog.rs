// TODO: join us uses a strange format, I have to check with a real save file...

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

/// Player interaction type for a blog post or comment
///
/// The C# type is `JoinUsItemSaveData.InteractionTypeEnum`
#[repr(i32)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[serde(from = "i32", into = "i32")]
pub enum InteractionType {
    // TODO: by the player?
    /// Downvoted by the player
    DownVote = -1,
    /// No interaction
    Nothing = 0,
    /// Upvoted by the player
    Upvote = 1,
    #[num_enum(catch_all)]
    /// An unknown variant! Please let the developer know if this value ever pops up,
    /// you can safely unwrap, failing fast is probably the best solution
    Unknown(i32),
}

/// State of the `JoinUs` revolutionary blog
///
/// The C# type is `JoinUsBlogManager` (`TypeDefIndex`: 124)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct JoinUsBlogState {
    /// Per-blog save data
    #[serde(rename = "_blogSaves")]
    pub blog_saves: serde_json::Value,
}
