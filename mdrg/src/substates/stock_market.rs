use serde::{Deserialize, Serialize};

// TODO: this should be complete, but the strange type it has on csharp suggests me otherwise...
/// A single company on the stock exchange
///
/// The C# type is `StockCompany` (TypeDefIndex: 1501)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct StockCompany {
    /// Company name
    #[serde(rename = "<Name>k__BackingField")]
    pub name: String,
    /// Number of stocks the player owns in this company
    #[serde(rename = "OwnedStocksCount", default)]
    pub owned_stocks_count: i32,
    /// Total cost basis of the player's owned stocks
    #[serde(rename = "OwnedStocksCost", default)]
    pub owned_stocks_cost: i32,
    /// Target price the stock is trending towards
    #[serde(rename = "<TargetPrice>k__BackingField")]
    pub target_price: f32,
    /// Serialized recent price history (via `LinkedList<float>`)
    #[serde(rename = "priceMemorySerialize", default)]
    pub price_memory: Vec<f32>,
}

/// State of the stock market
///
/// The C# type is `StockManager` (TypeDefIndex: 1502)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct StockMarketState {
    /// All listed stock companies
    #[serde(rename = "<StockCompanies>k__BackingField", default)]
    pub stock_companies: Vec<StockCompany>,
}
