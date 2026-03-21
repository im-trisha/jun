use serde::{Deserialize, Serialize};

use crate::items::Item;

/// A pending `FedUp` delivery package
///
/// The C# type is `DeliveryManager.Delivery` (`TypeDefIndex`: 1296)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct Delivery {
    /// Package tracking number
    #[serde(rename = "<TrackingNumber>k__BackingField")]
    pub tracking_number: String,
    /// Items contained in this delivery
    #[serde(rename = "<DeliveryItems>k__BackingField", default)]
    pub delivery_items: Vec<Item>,
    /// In-game time (minutes) when the delivery was sent
    #[serde(rename = "<SentTime>k__BackingField", default)]
    pub sent_time: i32,
    /// Duration (in-game minutes) for the delivery to arrive
    #[serde(rename = "<DeliveryDuration>k__BackingField", default)]
    pub delivery_duration: i32,
    /// Whether premium (faster) delivery was purchased
    #[serde(rename = "<PremiumDeliveryBought>k__BackingField", default)]
    pub premium_delivery_bought: bool,
    /// Sender name
    // TODO: `SimpleLocalizedString`, a bit obscure... I need to check with my save file
    #[serde(rename = "<Sender>k__BackingField")]
    pub sender: serde_json::Value,
}

/// State of the `FedUp` deliveries
///
/// The C# type is `DeliveryManager` (`TypeDefIndex`: 1299)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize, Default)]
pub struct DeliveriesState {
    /// All pending and completed deliveries
    #[serde(rename = "<Deliveries>k__BackingField", default)]
    pub deliveries: Vec<Delivery>,
}
