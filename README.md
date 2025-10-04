
# The Guessing Game (Rust)

A simple command-line guessing game written in Rust. The program generates a random secret number between 1 and 100, and the player has to guess it.

## Features

- Generates a random secret number between 1 and 100.
- Accepts user input from the terminal.
- Provides feedback whether the guess is too low, too high, or correct.
- Loops until the player guesses the correct number.

## Requirements

- Rust (latest stable version recommended)
- Cargo (Rust’s package manager)

## Installation

1. Clone the repository:
   
   ```bash
   git clone <repository-url>
   cd the-guessing-game

    ```

2. Build the project:

   ```bash
   cargo build
   ```

## Usage

Run the game using Cargo:

```bash
cargo run
```

The game will display:

```
The Guessing Game
Guess The Number...
```

Enter your guesses and the game will provide feedback:

* `Too Low!` – if your guess is less than the secret number
* `Too High!` – if your guess is greater than the secret number
* `You Win!` – when you guess the correct number

## Example

```
The Guessing Game
Guess The Number...
50
Too Low!
75
Too High!
63
You Win!
```

## How It Works

* The program generates a random number using the `rand` crate.
* It reads user input from the terminal and converts it into a number.
* The input is compared to the secret number using Rust’s `Ordering` enum.
* Feedback is displayed until the correct number is guessed.

## Dependencies

* `rand` crate for generating random numbers.

```toml
[dependencies]
rand = "0.9.0"
```

## License

This project is licensed under the MIT License.
