#![allow(dead_code)]

use super::board::ExitPosition;
use super::vehicle::{Orientation, Vehicle};
use std::collections::{HashSet, VecDeque};

/// State of all vehicle coordinates on the board.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoardState(Vec<(i32, i32)>);

/// Solves a parking puzzle using Breadth-First Search to guarantee the optimal minimum moves.
pub fn solve(width: i32, height: i32, exit: ExitPosition, vehicles: &[Vehicle]) -> Option<u32> {
    let player_idx = vehicles.iter().position(|v| v.is_player)?;
    let initial_state = BoardState(vehicles.iter().map(|v| (v.x, v.y)).collect());

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((initial_state.clone(), 0));
    visited.insert(initial_state);

    while let Some((state, moves)) = queue.pop_front() {
        let (px, py) = state.0[player_idx];
        let player = &vehicles[player_idx];

        if exit.is_reached(player.orientation, px, py, player.length, width, height) {
            return Some(moves);
        }

        for next_state in generate_next_states(width, height, &state, vehicles) {
            if !visited.contains(&next_state) {
                visited.insert(next_state.clone());
                queue.push_back((next_state, moves + 1));
            }
        }
    }

    None
}

fn generate_next_states(
    width: i32,
    height: i32,
    current: &BoardState,
    vehicles: &[Vehicle],
) -> Vec<BoardState> {
    let occupied: HashSet<(i32, i32)> = current
        .0
        .iter()
        .zip(vehicles)
        .flat_map(|(&(vx, vy), veh)| veh.orientation.cells(vx, vy, veh.length))
        .collect();

    current
        .0
        .iter()
        .enumerate()
        .flat_map(|(idx, &(vx, vy))| {
            let veh = &vehicles[idx];
            let (vlen, orient) = (veh.length, veh.orientation);

            let own_cells: HashSet<(i32, i32)> = orient.cells(vx, vy, vlen).collect();

            let is_cell_free = |cx: i32, cy: i32| {
                cx >= 0
                    && cx < width
                    && cy >= 0
                    && cy < height
                    && (!occupied.contains(&(cx, cy)) || own_cells.contains(&(cx, cy)))
            };

            let mut valid_moves = Vec::new();

            let (pos, max_dim) = match orient {
                Orientation::Horizontal => (vx, width),
                Orientation::Vertical => (vy, height),
            };

            // Negative direction (backwards)
            for step in 1..=pos {
                let target = pos - step;
                let free = match orient {
                    Orientation::Horizontal => is_cell_free(target, vy),
                    Orientation::Vertical => is_cell_free(vx, target),
                };
                if !free {
                    break;
                }
                let mut next = current.0.clone();
                next[idx] = match orient {
                    Orientation::Horizontal => (target, vy),
                    Orientation::Vertical => (vx, target),
                };
                valid_moves.push(BoardState(next));
            }

            // Positive direction (forwards)
            for step in 1..=(max_dim - (pos + vlen)) {
                let target = pos + step;
                let front = pos + vlen - 1 + step;
                let free = match orient {
                    Orientation::Horizontal => is_cell_free(front, vy),
                    Orientation::Vertical => is_cell_free(vx, front),
                };
                if !free {
                    break;
                }
                let mut next = current.0.clone();
                next[idx] = match orient {
                    Orientation::Horizontal => (target, vy),
                    Orientation::Vertical => (vx, target),
                };
                valid_moves.push(BoardState(next));
            }

            valid_moves
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::board::ExitSide;
    use crate::game::vehicle::VehicleKind;

    #[test]
    fn test_simple_solver() {
        let vehicles = vec![
            Vehicle::new(
                "p",
                VehicleKind::PlayerRed,
                1,
                2,
                2,
                Orientation::Horizontal,
                true,
            ),
            Vehicle::new(
                "c1",
                VehicleKind::CarSedanBlue,
                3,
                1,
                2,
                Orientation::Vertical,
                false,
            ),
            Vehicle::new(
                "t1",
                VehicleKind::TruckDelivery,
                4,
                2,
                3,
                Orientation::Vertical,
                false,
            ),
        ];

        let exit = ExitPosition {
            side: ExitSide::Right,
            row: 2,
            col: 0,
        };

        let result = solve(6, 6, exit, &vehicles);
        assert!(result.is_some(), "Puzzle should be solvable");
        assert!(result.unwrap() >= 2);
    }
}
