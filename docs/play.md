# Play

## Goal
Implement Tic Tac Toe game to play against an algorithm

## Use Cases
- Tic tac toe can be selected from main menu
- Select a grid square places the players token.
- A player move is followed up with an AI move
- Game state can be saved, and loaded
- Game state and be reset
- Display a message when the game is over, stating who won

## In scope
- Above cases

## Out of Scope
- Multiple saves
- PvP

## Overview
- AI algorithm will be minimax
- Game state can be save to existing persistence setup

### Schema
| Field | Type   |
|-------|--------|
| Cell1 | Enum   |
| Cell2 | Enum   |
| ...   | ...    |
| Cell9 | Enum   |

### Keyboard commands
| Key | Command                     |
|-----|-----------------------------|
| q   | quit to main menu           |
| 1-9 | select the respective cell  |
| s   | saves the game              |
| l   | loads the game              |
| r   | resets the game             |


### Implementation
- Db migration and seed
- Tic Tac Toe ViewModel
- Tic Tac Toe View
- Minimax Algorithm