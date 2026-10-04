# Tetris CLI

A terminal-based Tetris game written in **Rust**.

This project was built as a learning project to get hands-on experience with Rust by building a complete, interactive CLI application from scratch.

## Features

* Classic Tetris gameplay
* 7-bag randomizer
* Tetromino rotation and collision detection
* Line clearing
* Score and level system
* NES-inspired gravity and controls
* Terminal rendering with `crossterm`
* Installable as a CLI application

## What I Learned

This project gave me practical experience with:

* **Structs and `impl` blocks** for modeling game objects
* **Enums** for representing Tetromino types
* **Ownership and borrowing** when passing and modifying game state
* **`Result` and error handling** for invalid moves and collisions
* **Vectors and indexing** for representing the game board
* **Signed vs. unsigned integers** when working with coordinates and array indices
* **Modules** for structuring a larger Rust project
* **Pattern matching** with `match`
* **Traits** such as `Copy` and `Clone`
* **Randomness and external crates** with `rand`
* **Terminal input/output** and event handling with `crossterm`
* **Time-based game logic** using `Instant` and `Duration`
* Designing and separating **game state, board logic, input handling, and rendering**

## Installation

Requires [Rust](https://www.rust-lang.org/).

```bash
cargo install --path .
```

Run the game with:

```bash
tetris-cli
```

Readme generated with the help of AI
