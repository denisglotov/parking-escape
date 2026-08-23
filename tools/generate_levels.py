#!/usr/bin/env python3
"""
Parking Escape - Standalone Level Generator CLI Tool
Implements Method A: Constraint Placement + Solver Verification (BFS)
Selects and filters puzzle difficulty based on Minimum Optimal Moves (par moves).
"""

import argparse
from collections import deque
import json
import os
import random
import sys
from typing import Dict, List, Optional, Set, Tuple


VEHICLE_KINDS_BY_LENGTH = {
    2: ["car_sedan_blue", "car_taxi_yellow", "car_hatchback_green", "car_police"],
    3: ["truck_delivery", "limo_white", "ambulance"],
    4: ["semi_truck", "bus_transit"],
}

DEFAULT_DIFFICULTIES = {
    "beginner": (4, 7),
    "intermediate": (8, 11),
    "expert": (12, 25),
}


class Vehicle:
    def __init__(
        self,
        vid: str,
        kind: str,
        x: int,
        y: int,
        length: int,
        orientation: str,  # 'horizontal' or 'vertical'
        is_player: bool = False,
    ):
        self.id = vid
        self.kind = kind
        self.x = x
        self.y = y
        self.length = length
        self.orientation = orientation
        self.is_player = is_player

    def cells(self) -> List[Tuple[int, int]]:
        if self.orientation == "horizontal":
            return [(self.x + i, self.y) for i in range(self.length)]
        return [(self.x, self.y + i) for i in range(self.length)]

    def to_dict(self) -> dict:
        return {
            "id": self.id,
            "kind": self.kind,
            "x": self.x,
            "y": self.y,
            "length": self.length,
            "orientation": self.orientation,
            "is_player": self.is_player,
        }


class Obstacle:
    def __init__(
        self,
        oid: str,
        x: int,
        y: int,
        width: int = 1,
        height: int = 1,
        kind: Optional[str] = None,
    ):
        self.id = oid
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.kind = kind

    def cells(self) -> List[Tuple[int, int]]:
        return [
            (self.x + dx, self.y + dy)
            for dy in range(self.height)
            for dx in range(self.width)
        ]

    def to_dict(self) -> dict:
        d = {
            "id": self.id,
            "x": self.x,
            "y": self.y,
            "width": self.width,
            "height": self.height,
        }
        if self.kind:
            d["kind"] = self.kind
        return d


def solve_puzzle(
    width: int,
    height: int,
    exit_row: int,
    vehicles: List[Vehicle],
    obstacles: Optional[List[Obstacle]] = None,
    max_states: int = 15000,
) -> Optional[int]:
    """
    Breadth-First Search (BFS) solver for sliding-block puzzle.
    Returns the exact minimum moves to get the player car to the exit gate,
    or None if unsolvable / state limit exceeded.
    """
    try:
        player_idx = next(i for i, v in enumerate(vehicles) if v.is_player)
    except StopIteration:
        return None

    initial_state = tuple((v.x, v.y) for v in vehicles)
    queue = deque([(initial_state, 0)])
    visited = {initial_state}

    v_lens = [v.length for v in vehicles]
    v_orients = [v.orientation for v in vehicles]
    num_v = len(vehicles)
    plen = v_lens[player_idx]

    obstacle_cells: Set[Tuple[int, int]] = set()
    if obstacles:
        for obs in obstacles:
            for cell in obs.cells():
                obstacle_cells.add(cell)

    while queue:
        state, moves = queue.popleft()
        px, py = state[player_idx]

        # Goal check: Player reaches exit gate
        if py == exit_row and px + plen >= width:
            return moves

        if len(visited) >= max_states:
            continue

        # Spatial hash of occupied cells
        grid: Dict[Tuple[int, int], int] = {
            cell: -1 for cell in obstacle_cells}
        for i, (vx, vy) in enumerate(state):
            vl = v_lens[i]
            vo = v_orients[i]
            if vo == "horizontal":
                for k in range(vl):
                    grid[(vx + k, vy)] = i
            else:
                for k in range(vl):
                    grid[(vx, vy + k)] = i

        # Explore all valid single-vehicle moves
        for i, (vx, vy) in enumerate(state):
            vl = v_lens[i]
            vo = v_orients[i]

            if vo == "horizontal":
                # Move Left
                for step in range(1, vx + 1):
                    tx = vx - step
                    if (tx, vy) in grid:
                        break
                    ns = list(state)
                    ns[i] = (tx, vy)
                    tns = tuple(ns)
                    if tns not in visited:
                        visited.add(tns)
                        queue.append((tns, moves + 1))

                # Move Right
                for step in range(1, width - (vx + vl) + 1):
                    fx = vx + vl - 1 + step
                    if (fx, vy) in grid:
                        break
                    tx = vx + step
                    ns = list(state)
                    ns[i] = (tx, vy)
                    tns = tuple(ns)
                    if tns not in visited:
                        visited.add(tns)
                        queue.append((tns, moves + 1))
            else:
                # Move Up
                for step in range(1, vy + 1):
                    ty = vy - step
                    if (vx, ty) in grid:
                        break
                    ns = list(state)
                    ns[i] = (vx, ty)
                    tns = tuple(ns)
                    if tns not in visited:
                        visited.add(tns)
                        queue.append((tns, moves + 1))

                # Move Down
                for step in range(1, height - (vy + vl) + 1):
                    fy = vy + vl - 1 + step
                    if (vx, fy) in grid:
                        break
                    ty = vy + step
                    ns = list(state)
                    ns[i] = (vx, ty)
                    tns = tuple(ns)
                    if tns not in visited:
                        visited.add(tns)
                        queue.append((tns, moves + 1))

    return None


class ConstraintGenerator:
    """
    Method A: Constraint Placement + Solver Verification.
    Generates layered obstacle configurations and verifies difficulty using BFS minimum optimal moves.
    """

    def __init__(
        self,
        width: int,
        height: int,
        exit_row: int,
        min_moves: int,
        max_moves: int,
        min_vehicles: Optional[int] = None,
        max_vehicles: Optional[int] = None,
    ):
        self.width = width
        self.height = height
        self.exit_row = exit_row
        self.min_moves = min_moves
        self.max_moves = max_moves

        if width <= 6:
            self.min_vehicles = min_vehicles or 4
            self.max_vehicles = max_vehicles or 7
            self.allowed_lengths = [2, 3]
        elif width <= 8:
            self.min_vehicles = min_vehicles or 6
            self.max_vehicles = max_vehicles or 9
            self.allowed_lengths = [2, 3, 4]
        else:
            self.min_vehicles = min_vehicles or 7
            self.max_vehicles = max_vehicles or 11
            self.allowed_lengths = [2, 3, 4]

    def _is_free(
        self,
        occupied: Set[Tuple[int, int]],
        x: int,
        y: int,
        length: int,
        orientation: str,
    ) -> bool:
        if orientation == "horizontal":
            if x < 0 or x + length > self.width or y < 0 or y >= self.height:
                return False
            return not any((x + i, y) in occupied for i in range(length))
        else:
            if x < 0 or x >= self.width or y < 0 or y + length > self.height:
                return False
            return not any((x, y + i) in occupied for i in range(length))

    def _occupy(
        self,
        occupied: Set[Tuple[int, int]],
        x: int,
        y: int,
        length: int,
        orientation: str,
    ):
        if orientation == "horizontal":
            for i in range(length):
                occupied.add((x + i, y))
        else:
            for i in range(length):
                occupied.add((x, y + i))

    def _pick_kind(self, length: int) -> str:
        kinds = VEHICLE_KINDS_BY_LENGTH.get(length, ["car_sedan_blue"])
        return random.choice(kinds)

    def generate_single_attempt(self) -> Optional[Tuple[List[Vehicle], int]]:
        occupied: Set[Tuple[int, int]] = set()
        vehicles: List[Vehicle] = []
        veh_counter = 1

        # -------------------------------------------------------------
        # Step 1: Constraint - Place Player Vehicle (Length 2, Horizontal)
        # -------------------------------------------------------------
        max_player_x = max(0, self.width - 4)
        player_x = random.randint(0, max_player_x)
        player = Vehicle(
            vid="player",
            kind="player_red",
            x=player_x,
            y=self.exit_row,
            length=2,
            orientation="horizontal",
            is_player=True,
        )
        self._occupy(occupied, player.x, player.y,
                     player.length, player.orientation)
        vehicles.append(player)

        # -------------------------------------------------------------
        # Step 2: Constraint - Direct Vertical Blockers (1st degree)
        # -------------------------------------------------------------
        blocker_cols = list(range(player.x + player.length, self.width))
        random.shuffle(blocker_cols)
        desired_blockers = 1 if self.width == 6 else (
            2 if self.min_moves >= 5 and len(blocker_cols) >= 2 else 1)

        selected_cols = blocker_cols[:desired_blockers]

        for col in selected_cols:
            possible_lens = [
                l for l in self.allowed_lengths if l <= self.height]
            random.shuffle(possible_lens)
            for vlen in possible_lens:
                min_y = max(0, self.exit_row - vlen + 1)
                max_y = min(self.height - vlen, self.exit_row)
                valid_ys = [y for y in range(
                    min_y, max_y + 1) if self._is_free(occupied, col, y, vlen, "vertical")]
                if valid_ys:
                    chosen_y = random.choice(valid_ys)
                    v = Vehicle(
                        vid=f"v{veh_counter}",
                        kind=self._pick_kind(vlen),
                        x=col,
                        y=chosen_y,
                        length=vlen,
                        orientation="vertical",
                    )
                    self._occupy(occupied, v.x, v.y, v.length, v.orientation)
                    vehicles.append(v)
                    veh_counter += 1
                    break

        if len(vehicles) == 1:
            return None

        # -------------------------------------------------------------
        # Step 3: Constraint - Secondary Horizontal Blockers (2nd degree)
        # -------------------------------------------------------------
        verticals = [v for v in vehicles if v.orientation == "vertical"]
        for vert in verticals:
            side = random.choice(["top", "bottom"])
            cy = (vert.y - 1) if side == "top" else (vert.y + vert.length)
            if 0 <= cy < self.height and cy != self.exit_row:
                vlen = random.choice(self.allowed_lengths)
                min_x = max(0, vert.x - vlen + 1)
                max_x = min(self.width - vlen, vert.x)
                valid_xs = [x for x in range(
                    min_x, max_x + 1) if self._is_free(occupied, x, cy, vlen, "horizontal")]
                if valid_xs:
                    chosen_x = random.choice(valid_xs)
                    v = Vehicle(
                        vid=f"v{veh_counter}",
                        kind=self._pick_kind(vlen),
                        x=chosen_x,
                        y=cy,
                        length=vlen,
                        orientation="horizontal",
                    )
                    self._occupy(occupied, v.x, v.y, v.length, v.orientation)
                    vehicles.append(v)
                    veh_counter += 1

        # -------------------------------------------------------------
        # Step 4: Density Fill
        # -------------------------------------------------------------
        target_total = random.randint(self.min_vehicles, self.max_vehicles)
        attempts = 0
        while len(vehicles) < target_total and attempts < 20:
            attempts += 1
            orient = random.choice(["horizontal", "vertical"])
            vlen = random.choice(self.allowed_lengths)
            if orient == "horizontal":
                rx = random.randint(0, self.width - vlen)
                ry = random.randint(0, self.height - 1)
                if ry == self.exit_row and rx >= player.x:
                    continue
                if self._is_free(occupied, rx, ry, vlen, orient):
                    v = Vehicle(
                        vid=f"v{veh_counter}",
                        kind=self._pick_kind(vlen),
                        x=rx,
                        y=ry,
                        length=vlen,
                        orientation=orient,
                    )
                    self._occupy(occupied, rx, ry, vlen, orient)
                    vehicles.append(v)
                    veh_counter += 1
            else:
                rx = random.randint(0, self.width - 1)
                ry = random.randint(0, self.height - vlen)
                if rx >= player.x + player.length:
                    continue  # Keep exit corridor clear of static filler obstacles
                if self._is_free(occupied, rx, ry, vlen, orient):
                    v = Vehicle(
                        vid=f"v{veh_counter}",
                        kind=self._pick_kind(vlen),
                        x=rx,
                        y=ry,
                        length=vlen,
                        orientation=orient,
                    )
                    self._occupy(occupied, rx, ry, vlen, orient)
                    vehicles.append(v)
                    veh_counter += 1

        # -------------------------------------------------------------
        # Step 5: Solver Verification (BFS minimum optimal moves)
        # -------------------------------------------------------------
        moves = solve_puzzle(
            self.width,
            self.height,
            self.exit_row,
            vehicles,
            max_states=15000,
        )

        if moves is not None and self.min_moves <= moves <= self.max_moves:
            return vehicles, moves

        return None

    def generate_level(
        self,
        level_id: int,
        name: str,
        max_attempts: int = 10000,
    ) -> dict:
        for attempt in range(1, max_attempts + 1):
            result = self.generate_single_attempt()
            if result is not None:
                vehicles, optimal_moves = result
                return {
                    "id": level_id,
                    "name": name,
                    "width": self.width,
                    "height": self.height,
                    "exit": {
                        "side": "right",
                        "row": self.exit_row,
                    },
                    "vehicles": [v.to_dict() for v in vehicles],
                    "par_moves": optimal_moves,
                }

        raise RuntimeError(
            f"Failed to generate level '{name}' within [{
                self.min_moves}, {self.max_moves}] moves "
            f"after {max_attempts} attempts."
        )


def main():
    parser = argparse.ArgumentParser(
        description="Parking Escape - Level Generator CLI (Method A: Constraint Placement + BFS Verification)"
    )
    parser.add_argument(
        "--grid-size",
        type=int,
        default=6,
        choices=[6, 8, 10],
        help="Board grid dimensions (width & height). Default: 6",
    )
    parser.add_argument(
        "--difficulty",
        type=str,
        choices=["beginner", "intermediate", "expert"],
        default=None,
        help="Predefined difficulty tier ('beginner', 'intermediate', 'expert')",
    )
    parser.add_argument(
        "--min-moves",
        type=int,
        default=None,
        help="Minimum optimal moves (overrides --difficulty)",
    )
    parser.add_argument(
        "--max-moves",
        type=int,
        default=None,
        help="Maximum optimal moves (overrides --difficulty)",
    )
    parser.add_argument(
        "--exit-row",
        type=int,
        default=None,
        help="Target exit row (default: height // 2 - 1)",
    )
    parser.add_argument(
        "--count",
        type=int,
        default=1,
        help="Number of levels to generate. Default: 1",
    )
    parser.add_argument(
        "--start-id",
        type=int,
        default=None,
        help="Starting integer ID for generated levels. Defaults to auto (1 or max existing id + 1).",
    )
    parser.add_argument(
        "--name-prefix",
        type=str,
        default=None,
        help="Level name prefix (e.g. 'Level' or 'Stage')",
    )
    parser.add_argument(
        "--output",
        "-o",
        type=str,
        default=None,
        help="Output JSON file path. If omitted, prints to stdout.",
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=None,
        help="Random seed for deterministic generation",
    )
    parser.add_argument(
        "--max-attempts",
        type=int,
        default=10000,
        help="Maximum candidate attempts per level. Default: 10000",
    )

    args = parser.parse_args()

    if args.seed is not None:
        random.seed(args.seed)

    grid_size = args.grid_size
    exit_row = args.exit_row if args.exit_row is not None else (
        grid_size // 2 - 1)

    if args.min_moves is not None or args.max_moves is not None:
        min_moves = args.min_moves if args.min_moves is not None else 4
        max_moves = args.max_moves if args.max_moves is not None else 50
    elif args.difficulty:
        min_moves, max_moves = DEFAULT_DIFFICULTIES[args.difficulty]
    else:
        if grid_size == 6:
            min_moves, max_moves = DEFAULT_DIFFICULTIES["beginner"]
        elif grid_size == 8:
            min_moves, max_moves = DEFAULT_DIFFICULTIES["intermediate"]
        else:
            min_moves, max_moves = DEFAULT_DIFFICULTIES["expert"]

    # Read and validate existing levels if file exists
    existing_levels = []
    if args.output and os.path.exists(args.output):
        try:
            with open(args.output, "r") as f:
                data = json.load(f)
                if not isinstance(data, list):
                    print(
                        f"Error: Existing file '{
                            args.output}' does not contain a valid JSON array of levels.",
                        file=sys.stderr,
                    )
                    sys.exit(1)
                existing_levels = data
        except Exception as e:
            print(
                f"Error: Failed to read existing JSON file '{
                    args.output}': {e}",
                file=sys.stderr,
            )
            sys.exit(1)

        # Validate that existing levels match requested grid size and difficulty
        for lvl in existing_levels:
            if not isinstance(lvl, dict):
                continue
            lw = lvl.get("width")
            lh = lvl.get("height")
            if lw != grid_size or lh != grid_size:
                print(
                    f"Error: Existing level '{lvl.get('name', lvl.get('id'))}' in '{
                        args.output}' has grid size "
                    f"{lw}x{
                        lh}, which does not match requested --grid-size {grid_size}x{grid_size}.",
                    file=sys.stderr,
                )
                sys.exit(1)

            par = lvl.get("par_moves")
            if par is not None and (par < min_moves or par > max_moves):
                diff_label = f"'{
                    args.difficulty}'" if args.difficulty else "custom range"
                print(
                    f"Error: Existing level '{lvl.get('name', lvl.get('id'))}' in '{
                        args.output}' has {par} par moves, "
                    f"which does not match requested difficulty {
                        diff_label} (moves: {min_moves}..{max_moves}).",
                    file=sys.stderr,
                )
                sys.exit(1)

        print(
            f"Found existing file '{args.output}' with {
                len(existing_levels)} compatible level(s). "
            f"Appending new levels.",
            file=sys.stderr,
        )

    # Determine start ID
    if args.start_id is not None:
        start_id = args.start_id
    elif existing_levels:
        max_id = max((lvl.get("id", 0)
                     for lvl in existing_levels if isinstance(lvl, dict)), default=0)
        start_id = max_id + 1
    else:
        start_id = 1

    name_prefix = args.name_prefix or f"{grid_size}x{grid_size} Stage"

    generator = ConstraintGenerator(
        width=grid_size,
        height=grid_size,
        exit_row=exit_row,
        min_moves=min_moves,
        max_moves=max_moves,
    )

    generated_levels = []
    for i in range(args.count):
        lid = start_id + i
        lname = f"{name_prefix} {lid}"
        print(
            f"Generating [{i + 1}/{args.count}] '{
                lname}' ({grid_size}x{grid_size}, target moves: {min_moves}..{max_moves})...",
            file=sys.stderr,
        )
        lvl = generator.generate_level(
            lid, lname, max_attempts=args.max_attempts)
        print(f" -> Generated in {lvl['par_moves']} optimal moves ({
              len(lvl['vehicles'])} vehicles)", file=sys.stderr)
        generated_levels.append(lvl)

    all_levels = existing_levels + generated_levels
    output_json = json.dumps(all_levels, indent=2)

    if args.output:
        out_dir = os.path.dirname(os.path.abspath(args.output))
        if out_dir:
            os.makedirs(out_dir, exist_ok=True)
        with open(args.output, "w") as f:
            f.write(output_json)
            f.write("\n")
        if existing_levels:
            print(
                f"Successfully appended {len(generated_levels)} levels to {
                    args.output} (total: {len(all_levels)} levels)",
                file=sys.stderr,
            )
        else:
            print(f"Successfully saved {len(generated_levels)} levels to {
                  args.output}", file=sys.stderr)
    else:
        print(output_json)


if __name__ == "__main__":
    main()
