#!/usr/bin/env python3
"""
Level Generator and BFS Solver Validator for Parking Escape
Generates JSON level packs for 6x6, 8x8, and 10x10 boards, solving each with BFS to find the exact minimal move count.
"""

from collections import deque
import json
import os

class Vehicle:
    def __init__(self, vid, kind, x, y, length, orientation, is_player=False):
        self.id = vid
        self.kind = kind
        self.x = x
        self.y = y
        self.length = length
        self.orientation = orientation  # 'h' or 'v'
        self.is_player = is_player

    def to_dict(self):
        return {
            "id": self.id,
            "kind": self.kind,
            "x": self.x,
            "y": self.y,
            "length": self.length,
            "orientation": "horizontal" if self.orientation == 'h' else "vertical",
            "is_player": self.is_player
        }

def solve_puzzle(width, height, exit_row, vehicles):
    """
    Breadth-First-Search solver for the sliding block puzzle.
    Returns the minimum number of moves to get the player car to the exit.
    """
    player_idx = next(i for i, v in enumerate(vehicles) if v.is_player)
    
    # State representation: tuple of (x, y) for all vehicles
    initial_state = tuple((v.x, v.y) for v in vehicles)
    
    queue = deque([(initial_state, 0)])
    visited = {initial_state}

    while queue:
        state, moves = queue.popleft()
        
        # Check win condition: player reaches rightmost edge
        px, py = state[player_idx]
        plen = vehicles[player_idx].length
        if py == exit_row and px + plen == width:
            return moves + 1 # +1 move to exit the board

        # Build occupied grid
        grid = {}
        for i, (vx, vy) in enumerate(state):
            vlen = vehicles[i].length
            vorient = vehicles[i].orientation
            for k in range(vlen):
                cx = vx + k if vorient == 'h' else vx
                cy = vy if vorient == 'h' else vy + k
                grid[(cx, cy)] = i

        # Try moving each vehicle
        for i, (vx, vy) in enumerate(state):
            vlen = vehicles[i].length
            vorient = vehicles[i].orientation

            if vorient == 'h':
                # Move Left
                for step in range(1, vx + 1):
                    target_x = vx - step
                    if (target_x, vy) in grid:
                        break
                    new_state = list(state)
                    new_state[i] = (target_x, vy)
                    t_state = tuple(new_state)
                    if t_state not in visited:
                        visited.add(t_state)
                        queue.append((t_state, moves + 1))

                # Move Right
                for step in range(1, width - (vx + vlen) + 1):
                    target_x = vx + step
                    front_x = vx + vlen - 1 + step
                    if (front_x, vy) in grid:
                        break
                    new_state = list(state)
                    new_state[i] = (target_x, vy)
                    t_state = tuple(new_state)
                    if t_state not in visited:
                        visited.add(t_state)
                        queue.append((t_state, moves + 1))
            else:
                # Move Up
                for step in range(1, vy + 1):
                    target_y = vy - step
                    if (vx, target_y) in grid:
                        break
                    new_state = list(state)
                    new_state[i] = (vx, target_y)
                    t_state = tuple(new_state)
                    if t_state not in visited:
                        visited.add(t_state)
                        queue.append((t_state, moves + 1))

                # Move Down
                for step in range(1, height - (vy + vlen) + 1):
                    target_y = vy + step
                    front_y = vy + vlen - 1 + step
                    if (vx, front_y) in grid:
                        break
                    new_state = list(state)
                    new_state[i] = (vx, target_y)
                    t_state = tuple(new_state)
                    if t_state not in visited:
                        visited.add(t_state)
                        queue.append((t_state, moves + 1))

    return None

def build_level(lid, name, width, height, exit_row, vehicles):
    moves = solve_puzzle(width, height, exit_row, vehicles)
    if moves is None:
        raise ValueError(f"Level {name} is unsolvable!")
    print(f"Level '{name}' solved in {moves} moves")
    return {
        "id": lid,
        "name": name,
        "width": width,
        "height": height,
        "exit": {
            "side": "right",
            "row": exit_row
        },
        "vehicles": [v.to_dict() for v in vehicles],
        "par_moves": moves
    }

def generate_packs():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    out_dir = os.path.join(base_dir, "assets", "levels")
    os.makedirs(out_dir, exist_ok=True)

    # === 6x6 Beginner Pack ===
    p6_levels = []
    
    # 6x6 Level 1: Warmup
    p6_levels.append(build_level(1, "First Escape", 6, 6, 2, [
        Vehicle("player", "player_red", 1, 2, 2, 'h', is_player=True),
        Vehicle("c1", "car_sedan_blue", 3, 1, 2, 'v'),
        Vehicle("c2", "car_taxi_yellow", 0, 0, 2, 'h'),
        Vehicle("t1", "truck_delivery", 4, 2, 3, 'v'),
    ]))

    # 6x6 Level 2: The Blockade
    p6_levels.append(build_level(2, "The Blockade", 6, 6, 2, [
        Vehicle("player", "player_red", 1, 2, 2, 'h', is_player=True),
        Vehicle("c1", "car_taxi_yellow", 3, 0, 2, 'v'),
        Vehicle("c2", "car_sedan_blue", 4, 1, 2, 'h'),
        Vehicle("c3", "car_police", 0, 3, 2, 'v'),
        Vehicle("t1", "limo_white", 3, 2, 3, 'v'),
        Vehicle("c4", "car_hatchback_green", 1, 5, 2, 'h'),
    ]))

    # 6x6 Level 3: Crossroads
    p6_levels.append(build_level(3, "Crossroads", 6, 6, 2, [
        Vehicle("player", "player_red", 1, 2, 2, 'h', is_player=True),
        Vehicle("c1", "car_sedan_blue", 0, 0, 2, 'v'),
        Vehicle("c2", "car_taxi_yellow", 1, 0, 2, 'h'),
        Vehicle("t1", "truck_delivery", 3, 0, 3, 'v'),
        Vehicle("c3", "car_police", 4, 2, 2, 'v'),
        Vehicle("t2", "ambulance", 0, 4, 3, 'h'),
        Vehicle("c4", "car_hatchback_green", 4, 4, 2, 'v'),
    ]))

    # 6x6 Level 4: Twin Obstacles
    p6_levels.append(build_level(4, "Twin Obstacles", 6, 6, 2, [
        Vehicle("player", "player_red", 0, 2, 2, 'h', is_player=True),
        Vehicle("c1", "car_taxi_yellow", 0, 0, 2, 'h'),
        Vehicle("c2", "car_police", 2, 0, 2, 'v'),
        Vehicle("t1", "truck_delivery", 3, 1, 3, 'v'),
        Vehicle("c3", "car_sedan_blue", 4, 0, 2, 'h'),
        Vehicle("c4", "car_hatchback_green", 2, 3, 2, 'v'),
        Vehicle("t2", "ambulance", 0, 5, 3, 'h'),
        Vehicle("c5", "car_taxi_yellow", 4, 4, 2, 'v'),
    ]))

    # 6x6 Level 5: Gridlock Rush
    p6_levels.append(build_level(5, "Gridlock Rush", 6, 6, 2, [
        Vehicle("player", "player_red", 1, 2, 2, 'h', is_player=True),
        Vehicle("c1", "car_police", 0, 1, 2, 'v'),
        Vehicle("c2", "car_sedan_blue", 0, 0, 2, 'h'),
        Vehicle("c3", "car_taxi_yellow", 3, 1, 2, 'v'),
        Vehicle("t1", "limo_white", 4, 0, 3, 'v'),
        Vehicle("t2", "ambulance", 0, 4, 3, 'h'),
        Vehicle("c4", "car_hatchback_green", 3, 3, 2, 'v'),
        Vehicle("c5", "car_sedan_blue", 1, 5, 2, 'h'),
    ]))

    with open(os.path.join(out_dir, "pack_6x6.json"), "w") as f:
        json.dump(p6_levels, f, indent=2)
    print(f"Saved pack_6x6.json ({len(p6_levels)} levels)")

    # === 8x8 Intermediate Pack ===
    p8_levels = []

    # 8x8 Level 1: Avenue Jam
    p8_levels.append(build_level(1, "Avenue Jam", 8, 8, 3, [
        Vehicle("player", "player_red", 1, 3, 2, 'h', is_player=True),
        Vehicle("s1", "semi_truck", 3, 0, 4, 'v'),
        Vehicle("b1", "bus_transit", 4, 2, 4, 'v'),
        Vehicle("c1", "car_taxi_yellow", 0, 1, 2, 'v'),
        Vehicle("c2", "car_sedan_blue", 1, 1, 2, 'h'),
        Vehicle("t1", "truck_delivery", 5, 0, 3, 'v'),
        Vehicle("c3", "car_police", 0, 4, 2, 'v'),
        Vehicle("t2", "ambulance", 1, 5, 3, 'h'),
        Vehicle("c4", "car_hatchback_green", 5, 4, 2, 'v'),
        Vehicle("c5", "car_sedan_blue", 6, 2, 2, 'v'),
    ]))

    # 8x8 Level 2: Heavy Haul
    p8_levels.append(build_level(2, "Heavy Haul", 8, 8, 3, [
        Vehicle("player", "player_red", 0, 3, 2, 'h', is_player=True),
        Vehicle("s1", "semi_truck", 2, 0, 4, 'v'),
        Vehicle("b1", "bus_transit", 4, 0, 4, 'v'),
        Vehicle("t1", "limo_white", 0, 1, 3, 'h'),
        Vehicle("c1", "car_sedan_blue", 0, 4, 2, 'v'),
        Vehicle("c2", "car_police", 1, 5, 2, 'h'),
        Vehicle("t2", "truck_delivery", 3, 5, 3, 'h'),
        Vehicle("c3", "car_taxi_yellow", 6, 2, 2, 'v'),
        Vehicle("c4", "car_hatchback_green", 6, 5, 2, 'v'),
    ]))

    # 8x8 Level 3: Central Plaza
    p8_levels.append(build_level(3, "Central Plaza", 8, 8, 3, [
        Vehicle("player", "player_red", 2, 3, 2, 'h', is_player=True),
        Vehicle("b1", "bus_transit", 4, 0, 4, 'v'),
        Vehicle("t1", "ambulance", 0, 2, 3, 'v'),
        Vehicle("c1", "car_sedan_blue", 1, 0, 2, 'h'),
        Vehicle("c2", "car_taxi_yellow", 2, 1, 2, 'v'),
        Vehicle("c3", "car_police", 0, 5, 2, 'h'),
        Vehicle("t2", "truck_delivery", 2, 5, 3, 'h'),
        Vehicle("c4", "car_hatchback_green", 6, 1, 2, 'v'),
        Vehicle("c5", "car_sedan_blue", 6, 4, 2, 'v'),
    ]))

    with open(os.path.join(out_dir, "pack_8x8.json"), "w") as f:
        json.dump(p8_levels, f, indent=2)
    print(f"Saved pack_8x8.json ({len(p8_levels)} levels)")

    # === 10x10 Expert Pack ===
    p10_levels = []

    # 10x10 Level 1: Terminal Congestion
    p10_levels.append(build_level(1, "Terminal Congestion", 10, 10, 4, [
        Vehicle("player", "player_red", 1, 4, 2, 'h', is_player=True),
        Vehicle("s1", "semi_truck", 3, 0, 4, 'v'),
        Vehicle("b1", "bus_transit", 4, 2, 4, 'v'),
        Vehicle("s2", "semi_truck", 6, 0, 4, 'v'),
        Vehicle("t1", "limo_white", 0, 1, 3, 'h'),
        Vehicle("t2", "ambulance", 0, 2, 3, 'v'),
        Vehicle("c1", "car_police", 0, 5, 2, 'v'),
        Vehicle("c2", "car_taxi_yellow", 1, 6, 2, 'h'),
        Vehicle("t3", "truck_delivery", 3, 6, 3, 'h'),
        Vehicle("c3", "car_sedan_blue", 8, 2, 2, 'v'),
        Vehicle("c4", "car_hatchback_green", 8, 5, 2, 'v'),
    ]))

    # 10x10 Level 2: Mega Depot
    p10_levels.append(build_level(2, "Logistics Hub", 10, 10, 4, [
        Vehicle("player", "player_red", 0, 4, 2, 'h', is_player=True),
        Vehicle("s1", "semi_truck", 2, 0, 4, 'v'),
        Vehicle("b1", "bus_transit", 3, 2, 4, 'v'),
        Vehicle("s2", "semi_truck", 5, 0, 4, 'v'),
        Vehicle("t1", "truck_delivery", 0, 1, 3, 'h'),
        Vehicle("c1", "car_police", 0, 2, 2, 'v'),
        Vehicle("c2", "car_sedan_blue", 0, 6, 2, 'v'),
        Vehicle("t2", "ambulance", 1, 7, 3, 'h'),
        Vehicle("c3", "car_taxi_yellow", 4, 7, 2, 'h'),
        Vehicle("c4", "car_hatchback_green", 9, 1, 2, 'v'),
    ]))

    with open(os.path.join(out_dir, "pack_10x10.json"), "w") as f:
        json.dump(p10_levels, f, indent=2)
    print(f"Saved pack_10x10.json ({len(p10_levels)} levels)")

if __name__ == "__main__":
    generate_packs()
