//! Items related types

use std::collections::HashMap;

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod item;
mod item_condition;
mod item_requests;

pub use item::Item;
pub use item_condition::ItemCondition;
pub use item_requests::{ItemOrder, ItemOrderStatus, ItemRepairOrder};

use crate::{common::KeyedValues, mods::ModInfo};

// Opaque sub-types (fields not exposed by IL2CPP dump)
/// An in-game shop. Field layout not exposed by the dump; all JSON fields are
/// captured for round-trip serialization.
///
/// The C# type is `Shop`.
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct Shop {
    /// All serialized fields
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// A saved equipment preset (Panties, Bra, Dress...)
///
/// The C# type is `EquipmentSet` (`TypeDefIndex`: 1363)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct EquipmentSet {
    /// The name of this equipment set
    #[serde(rename = "Name", default)]
    pub name: String,
    /// The items in this set
    #[serde(rename = "EquippedItems", default)]
    pub equipped_items: Vec<Item>,
    /// The overall mods used by this set (The cumulative mods of the items)
    #[serde(rename = "UsedMods", default)]
    pub used_mods: Vec<ModInfo>,
}

/// Manages all in-game shops.
///
/// The C# type is `ShopManager` (`TypeDefIndex`: 1464)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize, Default)]
pub struct ShopState {
    /// Dynamic shops, an obsolete property
    #[serde(rename = "_dynamicShops", default)]
    pub dynamic_shops: KeyedValues<String, Shop>,
    /// All shops present in the game, this includes the grocery store, ladyparts.ic, Annalie...
    #[serde(rename = "_shops", default)]
    pub shops: Vec<Shop>,
}

/// Root item-system manager: holds the player inventory and related orders.
///
/// The C# type is `Item.ItemManager` (`TypeDefIndex`: 1384)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize, Default)]
pub struct ItemsState {
    /// Saved equipment sets
    #[serde(default)]
    pub sets: Vec<EquipmentSet>,
    /// Pending repair orders to Shanice
    #[serde(rename = "nunRepairOrders", default)]
    pub nun_repair_orders: Vec<ItemRepairOrder>,
    /// Pending clothing orders to Annalie
    #[serde(rename = "clothierOrders", default)]
    pub clothier_orders: Vec<ItemOrder>,
    /// The player's full item inventory
    #[serde(default)]
    pub items: Vec<Item>,
    /// Shop state, contains the state of all the shops,
    /// for instance Annalie, the grocery store...
    #[serde(rename = "shopManager", default)]
    pub shop_state: ShopState,
}

/// The type of a shop item
///
/// The C# type is `ShopItem.ShopItemType` (`TypeDefIndex`: 1459)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[repr(i32)]
#[derive(Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[serde(from = "i32", into = "i32")]
pub enum ShopItemType {
    /// Items like modules in ladyparts.ic,
    /// they appear randomly and disappear from the shop when you buy them
    SingleBuy = 0,
    /// Items like food in the grocery shop,
    /// they just exist no matter how many you buy
    Generic = 1,
    #[num_enum(catch_all)]
    /// An unknown variant! Please let the developer know if this value ever pops up,
    /// you can safely unwrap, failing fast is probably the best solution
    Unknown(i32),
}

/// A shop item
///
/// The C# type is `ShopItem` (`TypeDefIndex`: 1460)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct ShopItem {
    /// The wrapped item
    #[serde(rename = "_item")]
    pub item: Item,
    /// The type of this item
    #[serde(rename = "_itemType")]
    pub item_type: ShopItemType,
}
