# Battle Showdown

## Overview

Battle Showdown is a simple text-based game implemented in Rust, designed for deployment on the Vara Network using the Gear Protocol. In this game, players engage in battles with a boss character, employing various strategies and tactics to emerge victorious.


## Gameplay Description Details

Players choose a character type (e.g., Warrior, Mage, Archer) with varying base attack powers.
The game involves a single battle against a boss with a set number of lives.
Players can attack the boss, dealing damage based on their character type and a random modifier.
The boss attacks back, dealing random damage to the player.
The game continues until either the player or the boss runs out of lives.

## Features

- **Combat System**: Players can initiate attacks against the boss character, with each attack resulting in damage to both parties.
- **Character Customization**: Players can choose from different character types, each with unique abilities and attributes.
- **Game State Management**: The game maintains and updates the state of the battle, including player and boss health points, current level, and game state.
- **Randomized Elements**: Certain aspects of the game, such as hit power and boss actions, are randomized to introduce unpredictability and challenge.
- **Initialization and Reset**: The game supports initialization to set up initial conditions and reset functionality to restore default values.

## Code Functionality

- `get_random_u32` function: Generates a random number between 1 and 3 (inclusive) for in-game randomness.
- `BattleShowdown` struct: Holds the game state information for each player instance, including character details, lives, hit powers, and game state.
- `init` function: Initializes the game state upon receiving a message containing the player's character selection.
- `attack` function (placeholder): Simulates an attack round, calculating player and boss damage based on character type, random modifiers, and remaining lives. Updates the game state accordingly. (Needs further implementation)
- `reset` function (): Resets the game state to default values.
- `handle` function: Handles incoming messages containing attack actions. Decodes the action, calls the attack function, and sends back a response with the updated game state.
- `state` function: Retrieves and returns the current game state for the player.


## Deployment

This project is designed to be deployed on the Vara Network, leveraging the capabilities of the Gear Protocol for secure and efficient communication. The game logic is implemented in Rust, offering high performance and reliability.

To play Battle Showdown:

1. Compile the Rust code targeting the Vara Network platform. 
2. Deploy the compiled contract to the Vara Network using the Gear Protocol.
3. Interact with the deployed contract using the specified interface, initiating attacks and managing game state.


## Technical Details

- **Language:** Rust
- **Dependencies:**
  - gstd: Provides functionalities for writing smart contracts on Gear Network.
  - battle_showdown_io : Contains game-specific logic and data structures.
  - gear-wasm-builder: Used for building the contract for the Gear Network Wasm environment.
- **Deployment Target:** Vara Network (Gear Protocol)

## Usage

To play Battle Showdown:

1. Compile the Rust code targeting the Vara Network platform.
2. Deploy the compiled contract to the Vara Network using the Gear Protocol.
3. Interact with the deployed contract using the specified interface, initiating attacks and managing game state.

## Contributing

Contributions to Battle Showdown are welcome! Whether you're interested in adding new features, optimizing performance, or fixing bugs, your contributions can help enhance the game experience for players.

To contribute:

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Implement your changes, following the project's coding standards and guidelines.
4. Submit a pull request outlining your changes for review and inclusion.

## License

This project is licensed under the [MIT License](LICENSE), allowing for free use, modification, and distribution.

## Acknowledgements

Special thanks to the Vara Network community and the Gear Protocol team for their support in deploying and maintaining this project. Additionally, gratitude to the Rust community for providing a robust and reliable programming language for building decentralized applications.
