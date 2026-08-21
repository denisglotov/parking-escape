#![allow(dead_code)]

use super::board::{ExitPosition, ExitSide};
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
        let player_len = vehicles[player_idx].length;

        let is_at_exit = match (exit.side, vehicles[player_idx].orientation) {
            (ExitSide::Right, Orientation::Horizontal) => {
                py == exit.row && px + player_len >= width
            }
            (ExitSide::Left, Orientation::Horizontal) => py == exit.row && px <= 0,
            (ExitSide::Bottom, Orientation::Vertical) => {
                px == exit.col && py + player_len >= height
            }
            (ExitSide::Top, Orientation::Vertical) => px == exit.col && py <= 0,
            _ => false,
        };

        if is_at_exit {
            return Some(moves + 1);
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
        .zip(vehicles.iter())
        .flat_map(|(&(vx, vy), veh)| {
            let (len, orient) = (veh.length, veh.orientation);
            (0..len).map(move |i| match orient {
                Orientation::Horizontal => (vx + i, vy),
                Orientation::Vertical => (vx, vy + i),
            })
        })
        .collect();

    current
        .0
        .iter()
        .enumerate()
        .flat_map(|(idx, &(vx, vy))| {
            let veh = &vehicles[idx];
            let (vlen, orient) = (veh.length, veh.orientation);

            let own_cells: HashSet<(i32, i32)> = (0..vlen)
                .map(|k| match orient {
                    Orientation::Horizontal => (vx + k, vy),
                    Orientation::Vertical => (vx, vy + k),
                })
                .collect();

            let is_cell_free = |cx: i32, cy: i32| {
                cx >= 0
                    && cx < width
                    && cy >= 0
                    && cy < height
                    && (!occupied.contains(&(cx, cy)) || own_cells.contains(&(cx, cy)))
            };

            let mut valid_moves = Vec::new();

            match orient {
                Orientation::Horizontal => {
                    for step in 1..=vx {
                        let target_x = vx - step;
                        if is_cell_free(target_x, vy) {
                            let mut next = current.0.clone();
                            next[idx] = (target_x, vy);
                            valid_moves.push(BoardState(next));
                        } else {
                            break;
                        }
                    }
                    for step in 1..=(width - (vx + vlen)) {
                        let target_x = vx + step;
                        let front_x = vx + vlen - 1 + step;
                        if is_cell_free(front_x, vy) {
                            let mut next = current.0.clone();
                            next[idx] = (target_x, vy);
                            valid_moves.push(BoardState(next));
                        } else {
                            break;
                        }
                    }
                }
                Orientation::Vertical => {
                    for step in 1..=vy {
                        let target_y = vy - step;
                        if is_cell_free(vx, target_y) {
                            let mut next = current.0.clone();
                            next[idx] = (vx, target_y);
                            valid_moves.push(BoardState(next));
                        } else {
                            break;
                        }
                    }
                    for step in 1..=(height - (vy + vlen)) {
                        let target_y = vy + step;
                        let front_y = vy + vlen - 1 + step;
                        if is_cell_free(vx, front_y) {
                            let mut next = current.0.clone();
                            next[idx] = (vx, target_y);
                            valid_moves.push(BoardState(next));
                        } else {
                            break;
                        }
                    }
                }
            }

            valid_moves
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
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
