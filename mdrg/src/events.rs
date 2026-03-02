//! Event system types

use serde::{Deserialize, Serialize};

#[repr(i32)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    None = 0,
    StoryState = 1,
    FungusMessage = 2,
    ItemByDrone = 5,
    Email = 6,
    AddMoney = 9,
    StoryStateBlock = 10,
    AddFlag = 11,
    NewEmail = 12,
    ItemDelivery = 13,
}

/// Raw event payload
///
/// The C# type is `EventHolder` (TypeDefIndex: 1336)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct EventHolder {
    /// Event data
    // TODO: more info?
    #[serde(default)]
    pub data: String,
    /// The event type
    #[serde(rename = "eventEnum")]
    pub event_type: EventType,
}

/// A scheduled game event with its trigger time
///
/// The C# type is `EventManager.NormalEvent` (TypeDefIndex: 1332)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct NormalEvent {
    /// The event payload
    #[serde(rename = "eventHolder")]
    pub event_holder: EventHolder,
    /// In-game minute at which the event was scheduled
    #[serde(rename = "startTime")]
    pub start_time: i32,
}

/// Manages all the in-game events
///
/// The C# type is `EventManager` (TypeDefIndex: 1335)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize, Default)]
pub struct EventsState {
    /// All currently active or pending events
    #[serde(rename = "_events", default)]
    pub events: Vec<NormalEvent>,
}
