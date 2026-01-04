# Spec: Bootstrap basic 2D game

## 1. Overview

This track covers the initial bootstrap of a 2D grid-based game with a top-down camera. The goal is to implement the basic structure of the game, including a main menu, a simple gameplay state with player movement, and an in-game pause menu.

## 2. Core Features

### 2.1. Main Menu
- The game will start with a main menu screen.
- The menu will have two buttons: "Start" and "Exit".
- "Start" button will transition the game to the gameplay state.
- "Exit" button will close the application.

### 2.2. Gameplay State
- The game will be presented in 2D with a top-down camera.
- The game world will be a simple green screen representing a grassy area.
- A player sprite will be displayed on the screen.
- The player will be able to move the sprite using the WSAD keys.

### 2.3. In-Game Menu
- Pressing the "Escape" key during gameplay will pause the game and show an overlay menu.
- The in-game menu will have two options: "Continue" and "Exit to Main Menu".
- "Continue" will close the menu and resume the game.
- "Exit to Main Menu" will return the player to the main menu screen.

## 3. Technical Requirements

- The game will be built using the Bevy game engine in Rust.
- The game will be 2D with a grid-based layout and a top-down camera.
- The player movement will be restricted to the grid.
