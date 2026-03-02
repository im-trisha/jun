//! Email types, both the legacy format and the current prefab-reference format

use serde::{Deserialize, Serialize};

/// Legacy email format, its not serialized by the game anymore,
/// kept probably for backwards compatibility
///
/// C# types: `EmailBase` + `Email` (TypeDefIndex: 1303)
/// Stored in `GameVariables.emails`
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct LegacyEmail {
    /// Whether the player has opened this email
    pub read: bool,
    /// In-game minute at which the email becomes visible
    #[serde(rename = "visibleAt")]
    pub visible_at: i32,
    /// Display name of the sender
    pub sender: String,
    /// Subject line
    pub topic: String,
    /// Body text
    pub text: String,
}

/// Current email format referencing email prefabs by id
///
/// C# types: `EmailBase` + `NewEmail` (TypeDefIndex: 1304)
/// Stored in `GameVariables._allEmails`
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct NewEmail {
    /// Whether the player has opened this email
    pub read: bool,
    /// In-game minute at which the email becomes visible
    #[serde(rename = "visibleAt")]
    pub visible_at: i32,
    /// Identifier of the email prefab (game asset)
    #[serde(rename = "emailId")]
    pub email_id: String,
    /// Interpolation values injected into the prefab template
    #[serde(rename = "specialValues", default)]
    pub special_values: Vec<String>,
}
