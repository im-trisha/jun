//! All sub-state types stored inside [MDRGSaveSlot]

mod bot_status_app;
mod cocktracts;
mod cocktwitch;
mod cooking_minigame;
mod deliveries;
mod fishing_minigame;
mod join_us_blog;
mod stock_market;

pub use bot_status_app::*;
pub use cocktracts::*;
pub use cocktwitch::*;
pub use cooking_minigame::*;
pub use deliveries::*;
pub use fishing_minigame::*;
pub use join_us_blog::*;
pub use stock_market::*;

