//! In-game news system types

use serde::{Deserialize, Serialize};

use crate::common::GameId;

/// A deprecated, NewsId
///
/// You can check out the variants documentation to
/// understand better what this structure's purpose is
#[repr(i32)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NewsId {
    /// Invalid Id, this means that the news doesn't use this format anymore,
    /// instead it prefers the [GameId] to orchestrate the type news type
    ///
    /// Remember, this property is deprecated and only used in old save files!
    NotValid = -1,
    /// Identifies the main news on the website
    MainNews = 0,
    /// Identifies the side news no. 1
    SideNews1 = 1,
    /// Identifies the side news no. 2
    SideNews2 = 2,
    /// Identifies the side news no. 3
    SideNews3 = 3,
    /// Identifies the opinion news
    OpinionNews = 4,
    /// This was unexpected. Please report to the developer please!
    Unknown(i32),
}

/// A single news article slot with its seed for procedural generation.
///
/// The C# type is `NewsDataManager.NewsData` (TypeDefIndex: 15)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct NewsData {
    /// Numeric id of the news item, deprecated over [NewsData.game_id],
    /// in newer versions it has a default value of -1
    #[serde(rename = "newsId")]
    pub news_id: i32,
    /// Id of this news
    #[serde(rename = "gameId")]
    pub game_id: GameId,
    /// RNG seed used when generating this news item,
    ///
    /// Citing the lead developer (ΩSheep):
    /// > News can contain multiple randomized values, but we want to keep them consistent, so that it doesn't change between reloads of the news website.
    /// > It's easier to just generate the same random numbers using the same seed than save that data.
    #[serde(rename = "newsSeed")]
    pub news_seed: i32,
}

/// Holds all five current news slots shown in the game browser
///
/// The C# type is `NewsDataManager` (TypeDefIndex: 17)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct NewsDataState {
    /// The main headline article.
    #[serde(rename = "mainNews")]
    pub main_news: NewsData,
    /// First secondary article.
    #[serde(rename = "sideNews1")]
    pub side_news1: NewsData,
    /// Second secondary article.
    #[serde(rename = "sideNews2")]
    pub side_news2: NewsData,
    /// Third secondary article.
    #[serde(rename = "sideNews3")]
    pub side_news3: NewsData,
    /// Opinion / editorial article.
    #[serde(rename = "opinionNews")]
    pub opinion_news: NewsData,
}
