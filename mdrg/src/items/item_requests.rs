use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use super::Item;

/// A pending item repair order placed to Shanice
///
/// The C# type is `ItemRepairOrder` (`TypeDefIndex`: 1456)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct ItemRepairOrder {
    /// The item that was asked to be repaired (Thanks shanice~ <3)
    pub repaired_item: Item,
    /// In-game minutes of when the order was started
    #[serde(rename = "OrderStartedTime")]
    pub order_started_at: i32,
}

/// The status for an Annalie clothes order
///
/// The C# type is `ItemOrder` (`TypeDefIndex`: 1454)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[repr(i32)]
#[derive(Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[serde(from = "i32", into = "i32")]
pub enum ItemOrderStatus {
    /// It's added to the custom orders shop, but not paid for
    InCreation = 0,
    /// It's paid for and awaiting the completion. The order is "Created"
    Created = 1,
    /// The order is done and ready to be taken from Annalie
    Done = 2,
    #[num_enum(catch_all)]
    /// An unknown variant! Please let the developer know if this value ever pops up,
    /// you can safely unwrap, failing fast is probably the best solution
    Unknown(i32),
}

/// A pending clothing order given to Annalie
///
/// The C# type is `ItemOrder` (`TypeDefIndex`: 1455)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct ItemOrder {
    /// The item ordered
    #[serde(rename = "Item")]
    pub item: Item,
    /// In-game minute when the order will be finished
    #[serde(rename = "OrderFinishedTime")]
    pub order_finished_at: i32,
    /// The status of the order, check out [`ItemOrderStatus`] variants for more informations
    pub status: ItemOrderStatus,
}
