use std::collections::HashMap;

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    common::{Color, GameId, KvFloat, KvInt, KvString, SerializableGuid},
    items::ItemCondition,
};

/// An active modifier applied to an item
///
/// The C# type is `ItemModifier` (`TypeDefIndex`: 1406)
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
    /// Identifier for this `ItemModifier`
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
/// The C# type is `Item.ItemLocationEnum` (`TypeDefIndex`: 1372)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[repr(i32)]
#[derive(Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[serde(from = "i32", into = "i32")]
pub enum ItemLocation {
    /// The item is stored in Anon's apartment
    AtHome = 0,
    /// The item is being carried
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
/// The C# type is `Item` (`TypeDefIndex`: 1390)
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
    /// Stack count
    #[serde(rename = "_count")]
    count: i32,
    /// Quality rating
    ///
    /// You can have a simpler understanding of the meaning of it
    /// by using [`Item.item_condition`]
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
    #[serde(rename = "_additionalData", default)]
    pub additional_data: String,
    //TODO: AdditionalDataSlots is there and obsolete too, put it?
    /// Unique instance GUID for this specific item
    // TODO: Instance or additional identifier? maybe clearer to write clearer doc comments
    #[serde(rename = "UniqueItemGuid")]
    pub unique_item_guid: SerializableGuid,
    /// GUIDs of items "related to each other", citing the lead developer (ΩSheep):
    /// > Item.SourceItemsUniqueGuids is also related to handling the changing room.
    /// > Specifically handling packages in borrowed items.
    /// > Like a package of socks.
    /// > So that when you return one sock it automatically returns both the socks.
    #[serde(rename = "SourceItemsUniqueGuids", default)]
    pub source_items_unique_guids: Vec<SerializableGuid>,
    /// Equipment slot this item is currently equipped in (empty if none)
    // TODO: Present in the dump, make enum
    #[serde(rename = "_equipedSlot", default)]
    pub equiped_slot: String,
    /// Location of the item, only used at runtime
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

impl Item {
    #[must_use] 
    pub fn item_condition(&self) -> ItemCondition {
        ItemCondition::from(self.quality)
    }

    /// Get the item count
    ///
    ///
    /// # Notes
    /// The reason why this is behind a getter/setter, is because
    /// the actual underlying value is really misleading, for backwards compatibility:
    /// ```
    /// -1 = 0
    /// 0 = 1
    /// 1 = 2
    /// ```
    /// In game, if the 0-indexed value gets to -1, it will be removed from the inventory
    ///
    /// As of preventing breaking the save file and prevent confusion, this is behind a getter/setter
    ///
    /// You should treat this value as a normal value though,
    /// because this function does the job for you and is a transpilation of the C# code
    #[must_use] 
    pub const fn get_count(&self) -> i32 {
        self.count + 1
    }

    /// Sets the item count
    ///
    /// # Notes
    /// The reason why this is behind a getter/setter, is because
    /// the actual underlying value is really misleading, for backwards compatibility:
    /// ```
    /// -1 = 0
    /// 0 = 1
    /// 1 = 2
    /// ```
    /// In game, if the 0-indexed value gets to -1, it will be removed from the inventory
    ///
    /// As of preventing breaking the save file and prevent confusion, this is behind a getter/setter
    ///
    /// You should treat this value as a normal value though,
    /// because this function does the job for you and is a transpilation of the C# code
    pub const fn set_count(&mut self, count: i32) {
        let count = count - 1;

        if count < 0 {
            self.count = -1;
            // TODO: Is stackable here, add it RE
        } else if false {
            self.count = count;
        } else {
            self.count = 1;
        }
    }
}
