//! Item types

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize, Default)]
pub struct ItemsState {}
