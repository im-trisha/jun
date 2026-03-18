use serde::{Deserialize, Serialize};

use crate::common::Color32;

/// Console appearance settings
///
/// The C# type is `ConsoleStyleStruct` (`TypeDefIndex`: 287)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ConsoleStyle {
    /// Console text color
    #[serde(rename = "TextColor")]
    pub text_color: Color32,
    /// Console background color
    #[serde(rename = "BackgroundColor")]
    pub background_color: Color32,
}

/// State of the bot's status console app
///
/// The C# type is `BotStatusAppManager` (`TypeDefIndex`: 288)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct BotStatusAppState {
    /// Console appearance settings
    #[serde(rename = "_consoleStyle")]
    pub console_style: ConsoleStyle,
}
