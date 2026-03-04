//! Root save-slot type

use serde::{Deserialize, Serialize};

use crate::common::{Color, KeyedValues, SpecialVariablesHolder};
use crate::email::{LegacyEmail, NewEmail};
use crate::events::EventsState;
use crate::items::ItemsState;
use crate::news::NewsDataState;
use crate::save::dialogue_chain::DialogueChainMemory;
use crate::save::flag::Flag;
use crate::substates::{
    BotStatusAppState, CockTwitchState, CocktractsState, CookingMinigameState, DeliveriesState,
    FishingMinigameState, JoinUsBlogState, StockMarketState,
};

// TODO: not default on some fields important lorewise/game logic?

/// The root save-file object
///
/// The C# type is `GameVariables` (TypeDefIndex: 1329)
///
/// Fields marked `[Obsolete]` in C# are preserved here for round-trip
/// compatibility with older saves (Hopefully, don't put any faith in this, I didn't verify)
#[cfg_attr(feature = "derive-debug", derive(Clone))]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct MDRGSaveSlot {
    // Emails
    /// Legacy email list (obsolete, I suppose its kept for migration/backwards compatibility)
    #[serde(default)]
    pub emails: Vec<LegacyEmail>,
    /// Obsolete serialized-email string list
    #[serde(rename = "serializedEmails", default)]
    pub serialized_emails: Vec<String>,
    /// All current emails (active format)
    #[serde(rename = "_allEmails", default)]
    pub all_emails: Vec<NewEmail>,

    /// Game version in [semver] format
    #[serde(rename = "gameVersion")]
    pub game_version: semver::Version,

    // General game state
    /// Stock-market state
    #[serde(rename = "stockManager")]
    pub stock_market: StockMarketState,
    /// State for anything item-related
    #[serde(rename = "itemManager")]
    pub items: ItemsState,
    /// Ongoing game events
    #[serde(rename = "eventManager")]
    pub events: EventsState,
    /// Streaming state
    #[serde(rename = "cockTwitchManager")]
    pub cock_twitch: CockTwitchState,
    /// Cocktracts state
    #[serde(rename = "cocktractManager")]
    pub cocktracts: CocktractsState,
    /// Fedup deliveries state
    #[serde(rename = "deliveryManager")]
    pub deliveries: DeliveriesState,
    /// Cooking minigame state
    #[serde(rename = "cookingMinigameManager")]
    pub cooking_minigame: CookingMinigameState,
    /// Fishing minigame state
    #[serde(rename = "fishingMinigameManager")]
    pub fishing_minigame: FishingMinigameState,
    /// Bot console state
    #[serde(rename = "botStatusAppManager")]
    pub bot_console: BotStatusAppState,
    /// JoinUs blog state
    #[serde(rename = "joinUsBlogManager")]
    pub join_us_blog: JoinUsBlogState,
    /// Arbitrary custom key-value variables for modding, may be used by the main game too,
    /// check out the documentation of [SpecialVariablesHolder] for more information
    #[serde(rename = "customData")]
    pub custom_data: SpecialVariablesHolder,

    // Stream and church stats
    /// Total number of times the player has attended church
    // TODO: Remove default? This is an important number for the lore, story, event thingies...
    #[serde(rename = "timesWentToChurch", default)]
    pub times_went_to_church: i32,
    /// Total number of streams completed
    #[serde(rename = "streamCount", default)]
    pub stream_count: i32,
    /// Total in-game minutes spent streaming
    #[serde(rename = "streamedFor", default)]
    pub streamed_for: i32,
    /// Total money earned from stream donations
    #[serde(rename = "moneyEarnedFromDonations", default)]
    pub money_earned_from_donations: i32,
    /// Duration (in-game minutes) of the single longest stream
    #[serde(rename = "longestStream", default)]
    pub longest_stream: i32,

    // Intimate stats
    /// How many times the player ejaculated in Jun's vagina
    #[serde(rename = "timesCameInside", default)]
    pub times_came_inside: i32,
    /// How many times the player ejaculated in Jun's anus
    #[serde(rename = "timesCameInsideAnal", default)]
    pub times_came_inside_anal: i32,
    /// How many times the player ejaculated anywhere outside of Jun's orifices
    #[serde(rename = "timesCameOutside", default)]
    pub times_came_outside: i32,
    /// How many times the player ejaculated in Jun's oral cavity
    #[serde(rename = "timesCameInMouth", default)]
    pub times_came_in_mouth: i32,
    /// Volume (mL) ejaculated in Jun's oral cavity
    #[serde(rename = "mlCameInMouth", default)]
    pub ml_came_in_mouth: f32,
    /// Volume (mL) ejaculated anywhere outside of Jun's orifices
    #[serde(rename = "mlOfCumWasted", default)]
    pub ml_of_cum_wasted: f32,

    // Minigame wins / losses
    /// Chess games lost
    #[serde(rename = "timesLostChess", default)]
    pub times_lost_chess: i32,
    /// Chess games won
    #[serde(rename = "timesWonChess", default)]
    pub times_won_chess: i32,
    /// Old Maid rounds lost
    #[serde(rename = "timesLostOldMaid", default)]
    pub times_lost_old_maid: i32,
    /// Old Maid rounds won
    #[serde(rename = "timesWonOldMaid", default)]
    pub times_won_old_maid: i32,
    /// Old Maid rounds forfeited
    #[serde(rename = "timesRanAwayOldMaid", default)]
    pub times_ran_away_old_maid: i32,
    /// Word Chain rounds lost
    #[serde(rename = "timesLostWordChain", default)]
    pub times_lost_word_chain: i32,
    /// Word Chain rounds won
    #[serde(rename = "timesWonWordChain", default)]
    pub times_won_word_chain: i32,

    // Misc state
    /// Saved colour presets for item dyeing
    #[serde(rename = "presetColors", default)]
    pub preset_colors: Vec<Color>,
    /// Whether the bedroom light is currently on
    #[serde(rename = "lightSwitchOn", default)]
    pub light_switch_on: bool,

    // News / dialogue chain data
    /// Current in-game news articles
    #[serde(rename = "newsDataManager")]
    pub news_data: NewsDataState,
    /// Seen dialogue chains, key-value mapped
    #[serde(rename = "dialogueChainData", default)]
    pub seen_dialogue_chains: KeyedValues<String, DialogueChainMemory>,
    /// The status text on the top left, should be basically everytime "Rent in X days" or ""
    #[serde(rename = "statusText", default)]
    pub status_text: String,

    // Story flags
    /// All story and progression flags set so far
    #[serde(default)]
    pub story_flags: Vec<Flag>,

    // Economy / reputation
    /// Search score, which is between 0.0 and 1.0
    ///
    /// This score is related to peoples noticing Jun is a bot,
    /// 0.0 means no one noticed yet, 1.0 means it's bad
    #[serde(default)]
    pub search: f32,
    /// Reputation points with the priest bot
    #[serde(rename = "priestBotPoints", default)]
    pub priest_bot_points: i32,
    /// Reputation points with the Shanice
    #[serde(rename = "nunPoints", default)]
    pub nun_points: i32,
    /// Weekly rent
    #[serde(rename = "weeklyRent", default)]
    pub weekly_rent: i32,

    // Player economy
    /// Player's current balance
    pub money: i32,
    /// Casino token balance
    #[serde(rename = "casinoTokens", default)]
    pub casino_tokens: i32,

    // Player body stats
    /// Maximum volume (mL) of sperm Anon can store inside his testicles
    #[serde(rename = "_maxCum")]
    pub max_cum: f32,
    /// Remaining volume (mL) of sperm Anon has inside his testicles
    ///
    /// [MDRGSaveSlot.max_cum] - [MDRGSaveSlot.remaining_cum] will give you the missing sperm (in mL)
    #[serde(rename = "_remainingCum")]
    pub remaining_cum: f32,
    /// Current stamina level, between 0.0 and 1.0 (1.0 means its full)
    #[serde(rename = "_stamina")]
    pub stamina: f32,
    /// Current mental health level, between 0.0 and 1.0 (1.0 means its full)
    #[serde(rename = "_mentalHealth")]
    pub mental_health: f32,
    /// Temporary mental health modifier
    ///
    /// It's used for handling mental health drugs,
    /// those allow you to quickly increase mental health, but it decreases faster, the formula is:
    ///
    /// ```rust
    /// pub fn get_mental_health() -> f32 {
    ///     real_mental_health + temporary_mental_health
    /// }
    ///
    /// pub fn set_mental_health(new_val: f32) -> f32 {
    ///     let change = new_val - current_val;
    ///     let target = real_mental_health + change;
    ///     if (target <= 0.0) {
    ///         temporary_mental_health += target; // Temporary is always between 0 and 1
    ///         real_mental_health = 0;
    ///     } else {
    ///         real_mental_health = target.clamp(0.0, 1.0);
    ///     }
    /// }
    /// ```
    #[serde(rename = "_mentalHealthTemporary")]
    pub mental_health_temporary: f32,
    /// Current satiation level, between 0.0 and 1.0 (1.0 means you're full)
    #[serde(rename = "_satiation")]
    pub satiation: f32,
    /// Current health level, between 0.0 and 1.0 (1.0 means its full)
    ///
    /// This is used to handle starvation (when you are starving, it goes down and when you reach 0 you die)
    /// It also goes down with some drugs
    #[serde(rename = "_health")]
    pub health: f32,

    // Item effect timers
    /// In-game minutes when the Vinegara effect expires
    #[serde(rename = "vinegaraEffectEnd", default)]
    pub vinegara_effect_end: i32,
    /// In-game minute when the deathgrip debuff expires
    #[serde(rename = "deathGripEffectEnd", default)]
    pub death_grip_effect_end: i32,

    // Jun's stats
    /// Jun's lust level
    #[serde(rename = "_lust")]
    pub lust: i32,
    /// Jun longing / attachment level, between 0.0 and 10.0
    ///
    /// Citing the lead developer (ΩSheep):
    /// >  longing is for accumulated horniness. it goes from 0 to 10.
    /// > It gets converted to horniness when you activate a sex scene. Goes down when she cums.
    #[serde(rename = "_longing")]
    pub longing: f32,
    /// Jun's current arousal level, between 0.0 and 1.0 (1.0 means maximum intensit)
    #[serde(rename = "_currentHorniness")]
    pub current_horniness: f32,
    /// Jun sympathy level
    #[serde(rename = "_sympathy")]
    pub sympathy: i32,
    /// Jun's current mood, between -1 and 1
    #[serde(rename = "_mood")]
    pub mood: f32,
    /// Jun intelligence stat, the game serializes this field as "inteligence"
    #[serde(rename = "inteligence")]
    pub intelligence: i32,
    /// Volume of sperm in Jun's vagina (mL)
    #[serde(rename = "_cumInside")]
    pub cum_inside: f32,
    /// Volume of sperm in Jun's anus (mL)
    #[serde(rename = "_cumInsideAnal")]
    pub cum_inside_anal: f32,
    /// Volume of sperm in Jun's stomach (mL)
    #[serde(rename = "_cumInsideStomach")]
    pub cum_inside_stomach: f32,

    // Names / progression
    /// The player character's chosen name (Normally, its Anon)
    #[serde(rename = "playerName", default)]
    pub player_name: String,
    /// The sexbot's chosen name (Don't like to call her that, its Jun!)
    #[serde(rename = "botName", default)]
    pub bot_name: String,
    /// At what level the player is close to Jun, what "stage" their relationship is at
    ///
    /// This can also be seen from the [DialogueChains], there will be a key for each stage up to the last one, if your stage variable is 3, you will have:
    /// ```
    /// "Dialogue/BotDialogueStage1",
    /// "Dialogue/BotDialogueStage2",
    /// "Dialogue/BotDialogueStage3"
    /// ```
    #[serde(default)]
    pub stage: i32,
    /// Citing the lead developer (ΩSheep):
    ///
    ///  This is used to not allow the player to spam all the dialogues with jun at once.
    ///
    /// When unique_conversations_left is 0 only generic dialogues appear when talking with her.
    ///
    /// It goes down when a unique conversation activates
    #[serde(rename = "_uniqueConversationsLeft", default)]
    pub unique_conversations_left: i32,

    // Time
    /// Current in-game clock (in-game minutes)
    #[serde(default)]
    pub time: i32,

    // Activity timers
    /// In-game day on which the player last worked
    #[serde(rename = "lastWorkedAtDay", default)]
    pub last_worked_at_day: i32,
    /// In-game minutes of the last church visit
    #[serde(rename = "lastWentToChurchAt", default)]
    pub last_went_to_church_at: i32,
    /// In-game minutes of the last cuddle session
    #[serde(rename = "lastCuddledAt", default)]
    pub last_cuddled_at: i32,
    /// Whether the player has ever slept together with the bot
    ///
    /// Achtung: Slept not in the sense of "Sexual intercourse", but in the sense of literally sleeping,
    /// Sleeping with the bot can be unlocked further in the story
    #[serde(rename = "lastSleptWithBot", default)]
    pub last_slept_with_bot: bool,
    /// In-game minutes when the player last woke up
    #[serde(rename = "lastWokeUpAt", default)]
    pub last_woke_up_at: i32,
    /// In-game minutes of the last sexual activity
    #[serde(rename = "lastFuckedAt", default)]
    pub last_fucked_at: i32,
    /// In-game minutes of the last interaction with the Jun
    #[serde(rename = "lastInteractAt", default)]
    pub last_interact_at: i32,
    /// In-game minutes of the last equipment screen visit
    #[serde(rename = "lastEquipmentAt", default)]
    pub last_equipment_at: i32,
    /// In-game minutes of the last time hanging out with Jun
    #[serde(rename = "lastOutsideWithBotAt", default)]
    pub last_outside_with_bot_at: i32,
    /// In-game minute of the last stream session
    #[serde(rename = "lastStreamedAt", default)]
    pub last_streamed_at: i32,
    // TODO: small talk or any talk?
    /// In-game minute of the last small talk with Jun
    #[serde(rename = "lastTalkedAt", default)]
    pub last_talked_at: i32,
    /// In-game minutes when Jun last initiated conversation
    #[serde(rename = "lastBotStartedTalkAt", default)]
    pub last_bot_started_talk_at: i32,
    /// In-game minute of the last headpat, the game serializes this field as "lastHeadpatedAt"
    #[serde(rename = "lastHeadpatedAt", default)]
    pub last_headpatted_at: i32,
    /// In-game minute of the last hunger notification
    #[serde(rename = "lastHungerInfoAt", default)]
    pub last_hunger_info_at: i32,
    /// In-game minute of the last mental health notification
    #[serde(rename = "lastMentalHealthInfoAt", default)]
    pub last_mental_health_info_at: i32,

    // Stream platform stats
    /// Current subscriber count on CockTwitch
    #[serde(default)]
    pub subs: i32,
    /// Current follower count on CockTwitch
    #[serde(default)]
    pub followers: i32,
}
