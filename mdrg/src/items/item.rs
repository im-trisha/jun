use std::collections::HashMap;

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::{Color, GameId, KvFloat, KvInt, KvString, SerializableGuid};

/// An active modifier applied to an item
///
/// The C# type is `ItemModifier` (TypeDefIndex: 1406)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct ItemModifier {
    // SpecialVariablesHolder base fields
    /// Integer key-value variables attached to this item.
    #[serde(rename = "I", default)]
    pub ints: Vec<KvInt>,
    /// Float key-value variables attached to this item.
    #[serde(rename = "F", default)]
    pub floats: Vec<KvFloat>,
    /// String key-value variables attached to this item.
    #[serde(rename = "S", default)]
    pub strings: Vec<KvString>,

    // ItemModifier-specific fields
    /// Identifier for this ItemModifier
    #[serde(rename = "_id")]
    pub game_id: GameId,
    #[serde(rename = "_l")]
    pub level: f32,
}

// TODO: explore this further when able to with save files
/// A lazily-loaded dynamic sub-item container
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct DynamicItem {
    /// All serialized fields
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// Where an item is currently stored
///
/// The C# type is `Item.ItemLocationEnum` (TypeDefIndex: 1372)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[repr(i32)]
#[derive(Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[serde(from = "i32", into = "i32")]
pub enum ItemLocation {
    /// The item is stored in Anon's apartment
    AtHome = 0,
    /// The item is in Anon's inventory
    // TODO: items can be carried?
    Carried = 1,
    /// The item is borrowed from Annalie's shop
    BorrowedFromShop = 2,
    #[num_enum(catch_all)]
    /// An unknown variant! Please let the developer know if this value ever pops up,
    /// you can safely unwrap, failing fast is probably the best solution
    Unknown(i32),
}

/// A single inventory item. Extends `SpecialVariablesHolder` (I/F/S bags)
///
/// The C# type is `Item` (TypeDefIndex: 1390)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct Item {
    // SpecialVariablesHolder base fields
    /// Integer key-value variables attached to this item.
    #[serde(rename = "I", default)]
    pub ints: Vec<KvInt>,
    /// Float key-value variables attached to this item.
    #[serde(rename = "F", default)]
    pub floats: Vec<KvFloat>,
    /// String key-value variables attached to this item.
    #[serde(rename = "S", default)]
    pub strings: Vec<KvString>,

    // Item-specific fields
    // TODO: When 0? For example food when finished stays in inv with 0 count?
    /// Stack count, in some cases can be 0
    #[serde(rename = "_count")]
    pub count: i32,
    /// Quality rating (0.4 – 1.0).
    // TODO: Mappings from game qualities to floats?
    #[serde(rename = "_quality")]
    pub quality: f32,
    /// Whether the player has marked this item as a favourite.
    #[serde(rename = "IsFavourite")]
    pub is_favourite: bool,
    /// Legacy integer item id (deprecated, superseded by `game_id` and now maps to -1)
    #[serde(rename = "_id")]
    pub id: i32,
    /// Identifier for this item
    #[serde(rename = "_gameId")]
    pub game_id: GameId,
    /// Obsolete additional data blob
    // TODO: Really obsolete? I'm not entirely sure
    #[serde(rename = "_additionalData", default)]
    pub additional_data: String,
    /// Unique instance GUID for this specific item
    // TODO: Instance or additional identifier? maybe clearer to write clearer doc comments
    #[serde(rename = "UniqueItemGuid")]
    pub unique_item_guid: SerializableGuid,
    /// GUIDs of items that were combined to produce this item
    // TODO: ehm? A game mechanic i dont know or? I deduced it but (?)
    #[serde(rename = "SourceItemsUniqueGuids", default)]
    pub source_items_unique_guids: Vec<SerializableGuid>,
    /// Equipment slot this item is currently equipped in (empty if none)
    // TODO: Present in the dump, make enum
    #[serde(rename = "_equipedSlot", default)]
    pub equiped_slot: String,
    /// Location of the item
    #[serde(rename = "itemLocation")]
    pub item_location: ItemLocation,
    /// Dye / tint colors applied to the item
    #[serde(rename = "_colors", default)]
    pub colors: Vec<Color>,
    /// Dynamic sub-item containers
    #[serde(rename = "DI", default)]
    pub di: Vec<DynamicItem>,
    /// Active modifiers on this item
    #[serde(rename = "IM", default)]
    pub modifiers: Vec<ItemModifier>,
}
