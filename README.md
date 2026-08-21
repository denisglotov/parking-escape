# 🅿️ Parking Escape (Android Puzzle Game)

A dynamic sliding-block logic puzzle game for Android. Your mission is simple: navigate through chaotic parking lots and get your car to the exit!

## 🎮 Game Mechanics & Rules

The game is inspired by classic parking grid puzzles but features dynamic difficulty and scaling board sizes.

*   **The Grid:** The parking lot size changes depending on the level (e.g., starting at `6x6` for beginners and expanding to `8x8` or `10x10` for advanced stages).
*   **The Goal:** Clear a path and slide the **Player Car** through the parking lot exit.
*   **Movement Rules:** 
    *   Vehicles can only move forward and backward along their parked direction.
    *   Horizontal vehicles slide Left ↔ Right.
    *   Vertical vehicles slide Up ↕ Down.
    *   Vehicles **cannot** turn, move sideways, or pass through other vehicles.

## 🚙 Vehicles

Obstacles in the parking lot come in different sizes, adding layers of complexity to the puzzle:
*   **Cars:** Occupy **2 squares** (1x2 or 2x1).
*   **Limos / Small Trucks:** Occupy **3 squares** (1x3 or 3x1).
*   **Semis / Long Trailers:** Occupy **4 squares** (1x4 or 4x1).

*Note: The player's vehicle is always a 2-square car.*

## 🛠 Features

*   **Dynamic Board Sizes:** The grid expands as you progress, offering more complex traffic jams.
*   **Diverse Obstacles:** Maneuver around standard cars and massive 4-tile semi-trucks.
*   **Data-Driven Level Design:** Puzzles are loaded dynamically via JSON, containing both board dimensions and vehicle coordinates.
