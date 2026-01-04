# Fossil

A massively multiplayer game set in a world where dinosaurs still exist.

## Monorepo Structure

This repository is a monorepo containing the following:

-   `/crates`: Shared and game-specific Rust crates.
    -   `/crates/game`: The main game application (client and server).
-   `/docs`: Project documentation.
-   `/services`: Additional backend services.
-   `/web`: Web applications.

## Getting Started

To build and run the game, you can use the following command from the root of the repository:

```sh
cargo run -p game
```
