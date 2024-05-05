#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};
// use parity_scale_codec::Encode;

// Define the main struct for the BattleShowdown
pub struct BattleShowdown;

// Implementing Metadata for BattleShowdown
impl Metadata for BattleShowdown {
    // Define the type for initialization messages
    type Init = InOut<InitBattleShowdown, String>;
    // Define the type for handle messages
    type Handle = InOut<BattleShowdownAction, BattleShowdownEvent>;
    // Define the type for state messages
    type State = Out<BattleShowdownState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

// Struct for initializing the BattleShowdown
#[derive(Default, Debug, Encode, Decode, TypeInfo)]
pub struct InitBattleShowdown {
    pub player_character_type: CharacterType,
    pub player_name: String,
}

// Struct representing the state of the BattleShowdown
#[derive(Default, Debug, Encode, Decode, TypeInfo)]
pub struct BattleShowdownState {
    pub player_id: ActorId,
    pub player_character_type: CharacterType,
    pub current_level: u32,
    pub player_lives: u32,
    pub player_name: String,
    pub boss_lives: u32,
    pub player_hit_power: u32,
    pub boss_hit_power: u32,
}

// Enum representing different character types
#[derive(Debug, Clone, Copy, Encode, Decode, TypeInfo)]
pub enum CharacterType {
    Warrior,
    Mage,
    Archer,
}

// Enum representing different values for player hit power
#[derive(Debug, Clone, Copy, Encode, Decode, TypeInfo)]
pub enum PlayerHitPowerValue {
    X,
    Y,
    Z,
}

// Implementing Default for PlayerHitPowerValue
impl Default for PlayerHitPowerValue {
    fn default() -> Self {
        PlayerHitPowerValue::X
    }
}

// Implementing Default for CharacterType
impl Default for CharacterType {
    fn default() -> Self {
        CharacterType::Warrior
    }
}

// Enum representing different actions in the BattleShowdown
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum BattleShowdownAction {
    Attack {
        character_hit_power_value: PlayerHitPowerValue,
    },
    Rest {},
}

// Enum representing different events in the BattleShowdown
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum BattleShowdownEvent {
    Attacked {
        id: ActorId,
        character_type: CharacterType,
        name: String,
        player_lives: u32,
        boss_lives: u32,
    },
    PlayerLost {
        id: ActorId,
        character_type: CharacterType,
        boss_lives: u32,
        player_lives: u32,
        message: String,
    },
    BossLost {
        character_type: CharacterType,
        player_lives: u32,
        boss_lives: u32,
        message: String,
    },
    Rested {
        id: ActorId,
        character_type: CharacterType,
        player_lives: u32,
        name: String,
        boss_lives: u32,
        message: String,
    },
}
