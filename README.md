# Guessing Game in Rust

This project is a simple command-line guessing game written in Rust. The program generates a random secret number between 1 and 100, and the player has to guess the number. The program will provide feedback on whether the guess is too small, too big, or correct.

## Features

- Random number generation using the `rand` crate.
- Feedback on the player's guess.
- Colored output using the `colored` crate to enhance user experience.
- Simple and clear code structure.

## Getting Started

### Prerequisites

- **Rust**: You need to have Rust installed on your machine. You can install Rust using `rustup`:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Cargo**: Cargo is the Rust package manager. It comes with Rust, so once Rust is installed, Cargo should be available.

### Running the Game

1. Clone the repository:
   ```sh
   git clone https://github.com/kuslhhh/Guess-num.git
   cd Guess-name
   ```

2. Build and run the project using Cargo:
   ```sh
   cargo run
   ```

3. Follow the on-screen instructions to guess the number.

## Usage

Once the game starts, you will be prompted to input a guess. The game will provide feedback based on your guess:

- **"Too small!"** if your guess is less than the secret number.
- **"Too big!"** if your guess is greater than the secret number.
- **"You win!"** if you guessed the correct number.

The game continues until you guess the correct number.

## Example

```
Please input your guess: 
50
You guessed: 50
Too small!

Please input your guess: 
75
You guessed: 75
Too big!

Please input your guess: 
63
You guessed: 63
You win!
```



## Dependencies

This project uses the following dependencies:

- [`rand`](https://crates.io/crates/rand): For generating random numbers.
- [`colored`](https://crates.io/crates/colored): For colored terminal output.

You can find these dependencies in the `Cargo.toml` file.

## Contributing

Contributions are welcome! If you find any bugs or want to add new features, feel free to open an issue or submit a pull request.

