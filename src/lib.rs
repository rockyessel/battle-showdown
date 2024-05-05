#![no_std]

use gstd::{exec, msg, prelude::*, ActorId};

use battle_showdown_io::*;

// Function to generate random number between 1 and 3
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    (u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]]) % 3) + 1 // Generate random number between 1 and 3
}

#[derive(Debug, Default)]
pub struct BattleShowdown {
    pub player_id: ActorId,
    pub player_character_type: CharacterType,
    pub current_level: u32,
    pub player_lives: u32,
    pub player_name: String,
    pub boss_lives: u32,
    pub character_hit_power_value: PlayerHitPowerValue,
    pub player_hit_power: u32,
    pub boss_hit_power: u32,
    pub game_state: String,
}

static mut BATTLESHOWNDOWN: Option<BattleShowdown> = None;

/// Initializes the game contract and sets up the initial game state.
///
/// This function initializes the game contract by loading initialization data from a message.
/// The initialization data includes the player's chosen character type and name.
/// Based on this data, a `BattleShowdown` instance is created with initial values.
/// The created instance represents the state of the game, including player and boss stats, current level, and game state.
/// The initialized game state is stored in a static variable `BATTLESHOWNDOWN` to be accessed by game logic.
/// Finally, a reply message is sent back to indicate successful initialization of the game contract.
#[no_mangle]
unsafe extern "C" fn init() {
    // Load initialization data
    let init: InitBattleShowdown = msg::load().expect("Unable to decode InitBattleShowdown");

    // Create a BattleShowdown instance with initial values
    let battle_showdown = BattleShowdown {
        player_id: msg::source(),
        player_character_type: init.player_character_type,
        player_name: init.player_name,
        boss_lives: 10,
        player_lives: 10,
        game_state: "Started.".to_string(),
        ..Default::default()
    };

    // Store the BattleShowdown instance
    BATTLESHOWNDOWN = Some(battle_showdown);

    // Reply to signal successful initialization
    msg::reply_bytes("Successfully initialized", 0).expect("Failed to initialize successfully.");
}

impl Encode for BattleShowdown {
    fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();

        // Encode each field of BattleShowdown struct
        encoded.extend_from_slice(&self.player_id.encode());
        encoded.extend_from_slice(&self.player_character_type.encode());
        encoded.extend_from_slice(&self.current_level.encode());
        encoded.extend_from_slice(&self.player_lives.encode());
        encoded.extend_from_slice(&self.player_name.encode());
        encoded.extend_from_slice(&self.boss_lives.encode());
        encoded.extend_from_slice(&self.character_hit_power_value.encode());
        encoded.extend_from_slice(&self.player_hit_power.encode());
        encoded.extend_from_slice(&self.boss_hit_power.encode());
        encoded.extend_from_slice(&self.game_state.encode());

        encoded
    }
}

impl BattleShowdown {
    /// Simulates a combat encounter between the player and the boss character.
    /// Calculates hit power for both entities based on character type and randomness.
    /// Manages health points and generates game events to reflect the outcome of the encounter.
    fn attack(&mut self, _character_hit_power_value: PlayerHitPowerValue) -> BattleShowdownEvent {
        // Calculate total hit power for player based on character type and random values
        let character_hit_power = match &self.player_character_type {
            CharacterType::Warrior => 4,
            CharacterType::Mage => 3,
            CharacterType::Archer => 2,
        };

        let player_hit_power = match &self.character_hit_power_value {
            PlayerHitPowerValue::X => character_hit_power + get_random_u32(),
            PlayerHitPowerValue::Y => character_hit_power + get_random_u32(),
            PlayerHitPowerValue::Z => character_hit_power + get_random_u32(),
        };

        // Placeholder for boss attack logic
        // Update boss hit power to a random value for each attack
        self.boss_hit_power = get_random_u32();

        self.player_hit_power = player_hit_power;

        // Reduce boss's lives based on player's hit power
        self.boss_lives = self.boss_lives.saturating_sub(self.player_hit_power);
        // Reduce player's lives based on boss's hit power
        self.player_lives = self.player_lives.saturating_sub(self.boss_hit_power);

        // Check if player or boss has lost
        if self.player_lives == 0 {
            // Player lost
            self.game_state = "Player lost.".to_string();
            return BattleShowdownEvent::PlayerLost {
                id: self.player_id,
                boss_lives: self.boss_lives,
                character_type: self.player_character_type,
                message: "".to_string(),
                player_lives: self.player_lives,
            };
        } else if self.boss_lives == 0 {
            // Boss lost
            self.game_state = "Boss lost.".to_string();
            return BattleShowdownEvent::BossLost {
                boss_lives: self.boss_lives,
                character_type: self.player_character_type,
                player_lives: self.player_lives,
                message: "You've defeated the boss".to_string(),
            };
        }

        self.game_state = "The game continues.".to_string();
        // Return event indicating attack occurred
        BattleShowdownEvent::Attacked {
            boss_lives: self.boss_lives,
            character_type: self.player_character_type,
            id: self.player_id,
            name: self.player_name.clone(),
            player_lives: self.player_lives,
        }
    }

    /// Resets the game state to default values.
    /// This method is called when a Rest action is received.
    fn reset(&mut self) -> BattleShowdownEvent {
        // Reset various game state attributes
        self.boss_hit_power = 0;
        self.player_hit_power = 0;
        self.current_level = 0;
        self.boss_lives = 10;
        self.player_lives = 10;

        // Generate a Rested event with updated game state information
        BattleShowdownEvent::Rested {
            id: self.player_id,
            character_type: self.player_character_type,
            player_lives: self.player_lives,
            name: self.player_name.clone(),
            boss_lives: self.boss_lives,
            message: "Reset was successful.".to_string(),
        }
    }
}

/// Handles incoming messages and performs corresponding actions.
///
/// This function is responsible for processing incoming messages, specifically `BattleShowdownAction`.
/// It loads the action from the message, retrieves the current game state, and executes the appropriate action on the game state.
/// The result of the action is then sent back as a reply message in the form of `BattleShowdownEvent`.
#[no_mangle]
extern "C" fn handle() {
    // Load the action from the message
    let battle_showdown_action: BattleShowdownAction =
        msg::load().expect("Could not load BattleShowdownAction");

    // Retrieve the current game state
    let battle_showdown = unsafe {
        BATTLESHOWNDOWN
            .as_mut()
            .expect("`BattleShowdown` is not initialized.")
    };

    // Match the action and execute corresponding method, generating a game event
    let result: BattleShowdownEvent = match battle_showdown_action {
        // If the action is an Attack, call the attack method
        BattleShowdownAction::Attack {
            character_hit_power_value,
        } => battle_showdown.attack(character_hit_power_value),
        // If the action is Rest, call the reset method
        BattleShowdownAction::Rest {} => battle_showdown.reset(),
    };

    // Send back the result as a reply message
    msg::reply_bytes(result.encode(), 0)
        .expect("Failed to encode or reply with `BattleShowdownEvent`.");
}

/// The state function retrieves the current game state represented by BattleShowdown
/// and sends it as a reply.
#[no_mangle]
extern "C" fn state() {
    // Retrieve the BattleShowdown instance, consuming it in the process
    let battle_showdown = unsafe {
        BATTLESHOWNDOWN
            .take()
            .expect("Unexpected error in taking state")
    };

    // Reply with the game state
    msg::reply(battle_showdown, 0).expect("Unable to share the state");
}
