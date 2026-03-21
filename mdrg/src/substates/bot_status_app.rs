use serde::{Deserialize, Serialize};

use crate::common::Color32;

const fn default_text() -> Color32 {
    Color32 {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    }
}

const fn default_background() -> Color32 {
    Color32 {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    }
}

/// Console appearance settings
///
/// The C# type is `ConsoleStyleStruct` (`TypeDefIndex`: 287)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize, Default)]
pub struct ConsoleStyle {
    /// Console text color
    #[serde(rename = "TextColor", default = "default_text")]
    pub text_color: Color32,
    /// Console background color
    #[serde(rename = "BackgroundColor", default = "default_background")]
    pub background_color: Color32,
}

/// State of the bot's status console app
///
/// The C# type is `BotStatusAppManager` (`TypeDefIndex`: 288)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize, Default)]
pub struct BotStatusAppState {
    /// Console appearance settings
    #[serde(rename = "_consoleStyle", default)]
    pub console_style: ConsoleStyle,
}
