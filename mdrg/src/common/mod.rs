//! Primitive value types shared across the save model
//!
//! Contains:
//! `GameId`, `SerializableGuid`, Unity `Color`/`Color32`,
//! `PrefabSaves`, and the `SpecialVariablesHolder` key-value bag

mod keyed_values;

pub use keyed_values::KeyedValues;
use serde::{Deserialize, Serialize};

/// Opaque game-object identifier composed of an integer id and a legacy GUID
///
/// Citing the lead developer (ΩSheep):
/// > GameId contains guid and id, so that there's not any conflicts between mods
///
/// > When guid is empty it means the item is vanillla, part of the main game
///
/// > If guid has a value it's an item from a mod with that guid
///
/// The C# type is not present in the dump, I'm guessing it has been inlined,
/// it was easy to derive from the save files though
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Serialize, Deserialize)]
pub struct GameId {
    /// GUID, if non-empty, the item is a modded item from the mod having that GUID
    #[serde(rename = "_guid")]
    pub guid: SerializableGuid,
    /// Integer id
    #[serde(rename = "_id")]
    pub id: i32,
}

/// 128-bit GUID stored as a hex string
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Serialize, Deserialize)]
pub struct SerializableGuid {
    /// String representation of the 16-byte GUID
    #[serde(rename = "serializedGuid")]
    pub serialized_guid: String,
}

/// RGBA color matching `UnityEngine.Color` (each component is between 0.0 and 1.0)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct Color {
    /// Red channel
    pub r: f32,
    /// Green channel
    pub g: f32,
    /// Blue channel
    pub b: f32,
    /// Alpha channel
    pub a: f32,
}

/// RGBA color matching `UnityEngine.Color32` (each component is between 0 and 255)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Color32 {
    /// Red channel
    pub r: u8,
    /// Green channel
    pub g: u8,
    /// Blue channel
    pub b: u8,
    /// Alpha channel
    pub a: u8,
}

/// Wrapper for `PrefabManagerDataOneToOneLazy.Serialized<TKey, TPrefab, TSerializedPart>`
///
/// Only the `_s` list is serialized; the runtime dictionary and prefab references
/// are reconstructed by the game on load.
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct PrefabSaves<T> {
    /// The serialized save entries
    #[serde(rename = "_s", default)]
    pub saves: Vec<T>,
}

/// A key-value pair storing an integer variable
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct KvInt {
    /// Variable name
    pub key: String,
    /// Integer value
    pub value: i32,
}

/// A key-value pair storing a float variable
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct KvFloat {
    /// Variable name
    pub key: String,
    /// Float value
    pub value: f32,
}

/// A key-value pair storing a string variable
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct KvString {
    /// Variable name
    pub key: String,
    /// String value
    pub value: String,
}

/// Generic key-value bag attached to `Item` and the root [MDRGSaveSlot]
///
/// Citing the lead developer (ΩSheep):
///
/// > It can be used for mods, but it is also reused for items.
/// > It stores data about certain items here and there. For example FishWeights.
///
/// > This more flexible system was added later, so you can see that some of variables like quality are still just stored directly.
///
/// The C# type is `SpecialVariablesHolder` (TypeDefIndex: 1362)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize, Default)]
pub struct SpecialVariablesHolder {
    /// Integer variables
    #[serde(rename = "I", default)]
    pub ints: Vec<KvInt>,
    /// Float variables
    #[serde(rename = "F", default)]
    pub floats: Vec<KvFloat>,
    /// String variables
    #[serde(rename = "S", default)]
    pub strings: Vec<KvString>,
}
