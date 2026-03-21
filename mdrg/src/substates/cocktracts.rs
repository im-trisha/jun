use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

/// Lifecycle status of a cocktract contract
///
/// The C# type is `CocktractContract.ContractStatus` (`TypeDefIndex`: 1198)
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[repr(i32)]
#[derive(Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[serde(from = "i32", into = "i32")]
pub enum ContractStatus {
    /// Contract is available for the player to accept
    Available = 0,
    /// Contract has been accepted and is in progress
    Accepted = 100,
    /// Contract expired before the player accepted it
    Expired = 202,
    /// Contract was explicitly rejected
    Rejected = 203,
    /// Contract was successfully completed
    Completed = 300,
    /// Contract was failed
    Failed = 301,
    /// Contract was abandoned by the player
    Abandoned = 302,
    #[num_enum(catch_all)]
    /// An unknown variant! Please let the developer know if this value ever pops up,
    /// you can safely unwrap, failing fast is probably the best solution
    Unknown(i32),
}

/// A cocktract contract
///
/// The C# type is `CocktractContract` (`TypeDefIndex`: 1201)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct CocktractContract {
    /// Localized contract narration text
    // TODO: cry `SimpleLocalizedStringWithBakedArguments`, opaque
    #[serde(rename = "_contractNarration")]
    pub contract_narration: serde_json::Value,
    /// Whether the player has seen this contract in the UI
    #[serde(rename = "<Seen>k__BackingField", default)]
    pub seen: bool,
    /// Whether this is a special golden contract
    #[serde(rename = "<IsGold>k__BackingField", default)]
    pub is_gold: bool,
    /// Identifier of the partner who posted this contract
    #[serde(rename = "_partnerId", default)]
    pub partner_id: String,
    /// Contract requirement
    // TODO: cry `RequirementContainer` polymorphic
    #[serde(rename = "_requirement")]
    pub requirement: serde_json::Value,
    /// Bonus rewards given on acceptance
    // TODO: cry `RewardContainer` list, 3 props, yey
    #[serde(rename = "<AcceptBonusReward>k__BackingField", default)]
    pub accept_bonus_reward: Vec<serde_json::Value>,
    /// Rewards given on successful completion
    #[serde(rename = "<SuccessReward>k__BackingField", default)]
    pub success_reward: Vec<serde_json::Value>,
    /// Penalties applied on failure
    #[serde(rename = "<FailureReward>k__BackingField", default)]
    pub failure_reward: Vec<serde_json::Value>,
    /// Current lifecycle status
    #[serde(rename = "<Status>k__BackingField")]
    pub status: ContractStatus,
    /// In-game time (minutes) when the contract was created
    #[serde(rename = "<CreatedTime>k__BackingField", default)]
    pub created_time: i32,
    /// In-game time (minutes) allowed for accepting
    #[serde(rename = "<TimeLimitForAccepting>k__BackingField", default)]
    pub time_limit_for_accepting: i32,
    /// In-game time (minutes) when the contract was accepted
    #[serde(rename = "<AcceptedTime>k__BackingField", default)]
    pub accepted_time: i32,
    /// In-game time (minutes) when the contract was finished
    #[serde(rename = "<FinishedTime>k__BackingField", default)]
    pub finished_time: i32,
}

/// A cocktract partner the player can work for
///
/// The C# type is `CocktractPartner` (`TypeDefIndex`: 1207)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct CocktractPartner {
    /// Respect level accrued with this partner
    #[serde(rename = "<Respect>k__BackingField", default)]
    pub respect: f32,
    /// Partner identifier
    #[serde(rename = "<Id>k__BackingField")]
    pub id: String,
    /// Whether this partner has been unlocked
    #[serde(rename = "<Unlocked>k__BackingField", default)]
    pub unlocked: bool,
    /// Partner-specific data (`CocktractPartnerSpecialDataContainer`, polymorphic)
    // TODO: cry real hard
    #[serde(rename = "specialData")]
    pub special_data: serde_json::Value,
}

/// State of `CockTwitch` cocktracts
///
/// The C# type is `CocktractManager` (`TypeDefIndex`: 1205)
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize, Default)]
pub struct CocktractsState {
    /// Contracts currently available for the player to accept
    #[serde(rename = "AvailableContracts", default)]
    pub available_contracts: Vec<CocktractContract>,
    /// Contracts the player has accepted and is working on
    #[serde(rename = "CurrentContracts", default)]
    pub current_contracts: Vec<CocktractContract>,
    /// Historical contracts (completed, failed, abandoned, etc.)
    #[serde(rename = "PastContracts", default)]
    pub past_contracts: Vec<CocktractContract>,
    /// Global respect score across all partners
    #[serde(rename = "globalRespect", default)]
    pub global_respect: f32,
    /// All cocktract partners
    #[serde(rename = "<CocktractPartners>k__BackingField", default)]
    pub cocktract_partners: Vec<CocktractPartner>,
}
