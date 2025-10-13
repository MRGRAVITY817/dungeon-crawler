# Dungeon Crawler - A simple roguelike game written in Rust

Hands-on practice project for learning game development in Rust. 
The codebase follows the learning material from the [Rust Roguelike Tutorial](https://bfnightly.bracketproductions.com/rustbook/).

## Short Description

A dungeon crawler with procedurally generated levels, monsters of increasing difficulty, and turn-based movement.

## Story

You are an adventurer exploring a dangerous dungeon filled with monsters and treasures. Your goal is to find a lost amulet (the Amulet of Yendor) and escape the dungeon alive.

## Basic Game Loops

1. Enter the dungeon level.
2. Explore the level, revealing the map as you move.
3. Encounter monsters and engage in turn-based combat or avoid them.
4. Collect items and power-ups to aid your journey.
5. Find the exit to the next level.
6. Repeat until you find the Amulet of Yendor and escape the dungeon.

## Minimum Viable Product (MVP)

1. Create a basic dungeon map.
2. Place the player and let them move around.
3. Spawn monsters, draw them, and let the player attack them by moving into them.
4. Add health and a simple combat system.
5. Add health potions that the player can pick up and use.
6. Display a "Game Over" screen when the player dies.
7. Add the Amulet of Yendor and a win condition.

## Stretch Goals

1. Add Fields of View (FOV) so the player can only see parts of the map they have explored.
2. Add more interesting dungeon generation (e.g., rooms, corridors).
3. Add some dungeon themes (e.g., caves, forests).
4. Add multiple layers to the dungeon, with the amulet on the deepest layer.
5. Add varied weapons and armor with different stats.
6. Move to a data-driven design for items and monsters.
7. Consider some visual effects to make combat more visceral.
8. Consider keeping score.
