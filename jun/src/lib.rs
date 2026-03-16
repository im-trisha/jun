#![feature(trim_prefix_suffix)]

// TODO: CURRENTLY DOESN'T WORK ON WEB BECAUSE OF FILE PICKER, I THINK, AND 100% BECAUSE I READ USING IO FNS
// TODO: ON RELEASE CONVERT UNWRAPS TO ERROR HANDLING

mod app;
mod i18n;
mod state;
mod views;

pub use app::JunApp;
pub use i18n::Language;
pub use state::JunAppState;

#[macro_export]
macro_rules! try_i18n {
    ($app:expr, $expr:expr, $msg:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e.to_string());
                $app.errors.push($msg.into());
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! stat_column {
    ($col:expr, $title:expr, $desc:expr, $val:expr, $range:expr) => {
        $col.vertical(|ui| {
            ui.heading($title);
            ui.label($desc);
            ui.add(egui::DragValue::new($val).range($range));
        });
    };
    ($col:expr, $title:expr, $val:expr, $range:expr) => {
        $col.vertical(|ui| {
            ui.heading($title);
            ui.add(egui::DragValue::new($val).range($range));
        });
    };
}

#[macro_export]
macro_rules! text_column {
    ($col:expr, $title:expr, $desc:expr, $val:expr) => {
        $col.vertical(|ui| {
            ui.heading($title);
            ui.label($desc);
            ui.text_edit_singleline($val);
            ui.add_space(32.0);
        });
    };

    ($col:expr, $title:expr, $val:expr) => {
        $col.vertical(|ui| {
            ui.heading($title);
            ui.text_edit_singleline($val);
            ui.add_space(32.0);
        });
    };
}

#[macro_export]
macro_rules! bool_column {
    ($col:expr, $title:expr, $desc:expr, $val:expr) => {
        $col.vertical(|ui| {
            ui.heading($title);
            ui.horizontal(|ui| {
                ui.label($desc);
                ui.checkbox($val, "")
            });
        });
    };
}

// Emails, game version, managers (substates) and those guys are missing!
//
// /// Saved colour presets for item dyeing
// #[serde(rename = "presetColors", default)]
// pub preset_colors: Vec<Color>,
// // News / dialogue chain data
// /// Current in-game news articles
// #[serde(rename = "newsDataManager")]
// pub news_data: NewsDataState,
// /// Seen dialogue chains, key-value mapped
// #[serde(rename = "dialogueChainData", default)]
// pub seen_dialogue_chains: KeyedValues<String, DialogueChainMemory>,
// // Story flags
// /// All story and progression flags set so far
// #[serde(default)]
// pub story_flags: Vec<Flag>,
