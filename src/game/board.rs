use super::vehicle::{Orientation, Vehicle};
use crate::audio::SoundTrigger;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExitSide {
    Right,
    Left,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExitPosition {
    pub side: ExitSide,
    #[serde(default)]
    pub row: i32,
    #[serde(default)]
    pub col: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoardSnapshot {
    pub positions: Vec<(i32, i32)>,
    pub move_count: u32,
}

#[derive(Debug, Clone)]
pub struct DragState {
    pub vehicle_index: usize,
    pub start_pos: (f32, f32),
    pub min_offset: f32,
    pub max_offset: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub exit: ExitPosition,
    pub vehicles: Vec<Vehicle>,
    #[serde(skip)]
    pub move_count: u32,
    #[serde(skip)]
    pub history: Vec<BoardSnapshot>,
    #[serde(skip)]
    pub active_drag: Option<DragState>,
    #[serde(skip)]
    pub is_won: bool,
    #[serde(skip)]
    pub exit_animation_progress: f32,
    #[serde(skip)]
    initial_vehicles: Vec<Vehicle>,
}

impl Board {
    pub fn new(width: i32, height: i32, exit: ExitPosition, vehicles: Vec<Vehicle>) -> Self {
        let initial_vehicles = vehicles.clone();
        Self {
            width,
            height,
            exit,
            vehicles,
            move_count: 0,
            history: Vec::new(),
            active_drag: None,
            is_won: false,
            exit_animation_progress: 0.0,
            initial_vehicles,
        }
    }

    /// Checks if a cell is inside the board boundaries.
    pub fn is_inside_board(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    /// Checks if a cell is free of any obstacles, ignoring an optional vehicle index.
    pub fn is_cell_free(&self, x: i32, y: i32, ignore_idx: Option<usize>) -> bool {
        if !self.is_inside_board(x, y) {
            return false;
        }

        self.vehicles
            .iter()
            .enumerate()
            .filter(|(idx, _)| ignore_idx != Some(*idx))
            .all(|(_, v)| !v.contains_cell(x, y))
    }

    /// Computes the valid movement range [min_offset, max_offset] for a vehicle in integer grid units.
    pub fn compute_movement_bounds(&self, vehicle_idx: usize) -> (i32, i32) {
        let v = &self.vehicles[vehicle_idx];
        let (vx, vy, vlen, orient) = (v.x, v.y, v.length, v.orientation);

        match orient {
            Orientation::Horizontal => {
                let steps_left = (1..=vx)
                    .take_while(|step| self.is_cell_free(vx - step, vy, Some(vehicle_idx)))
                    .count() as i32;
                let min_offset = -steps_left;

                let steps_right = (1..=(self.width - (vx + vlen)))
                    .take_while(|step| {
                        self.is_cell_free(vx + vlen - 1 + step, vy, Some(vehicle_idx))
                    })
                    .count() as i32;
                let max_offset = steps_right;

                (min_offset, max_offset)
            }
            Orientation::Vertical => {
                let steps_up = (1..=vy)
                    .take_while(|step| self.is_cell_free(vx, vy - step, Some(vehicle_idx)))
                    .count() as i32;
                let min_offset = -steps_up;

                let steps_down = (1..=(self.height - (vy + vlen)))
                    .take_while(|step| {
                        self.is_cell_free(vx, vy + vlen - 1 + step, Some(vehicle_idx))
                    })
                    .count() as i32;
                let max_offset = steps_down;

                (min_offset, max_offset)
            }
        }
    }

    /// Handles pointer/touch down event. Returns true if a vehicle was selected.
    pub fn handle_touch_down(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        origin_x: f32,
        origin_y: f32,
        cell_size: f32,
    ) -> bool {
        if self.is_won {
            return false;
        }

        let hit = self.vehicles.iter().enumerate().rev().find_map(|(idx, v)| {
            let (px, py, pw, ph) = v.pixel_bounds(origin_x, origin_y, cell_size);
            if mouse_x >= px && mouse_x <= px + pw && mouse_y >= py && mouse_y <= py + ph {
                Some(idx)
            } else {
                None
            }
        });

        if let Some(vehicle_idx) = hit {
            let (min_bound, max_bound) = self.compute_movement_bounds(vehicle_idx);
            self.active_drag = Some(DragState {
                vehicle_index: vehicle_idx,
                start_pos: (mouse_x, mouse_y),
                min_offset: min_bound as f32,
                max_offset: max_bound as f32,
            });
            true
        } else {
            false
        }
    }

    /// Handles continuous pointer movement during drag.
    pub fn handle_touch_move(&mut self, mouse_x: f32, mouse_y: f32, cell_size: f32) {
        if let Some(drag) = &self.active_drag {
            let v = &mut self.vehicles[drag.vehicle_index];
            let raw_delta = match v.orientation {
                Orientation::Horizontal => (mouse_x - drag.start_pos.0) / cell_size,
                Orientation::Vertical => (mouse_y - drag.start_pos.1) / cell_size,
            };

            v.drag_offset = raw_delta.clamp(drag.min_offset, drag.max_offset);
        }
    }

    /// Handles pointer up / touch release. Snaps to grid and checks for move/win.
    pub fn handle_touch_up(&mut self) -> Option<SoundTrigger> {
        let drag = self.active_drag.take()?;
        let offset = self.vehicles[drag.vehicle_index].drag_offset;
        let rounded_offset = offset.round() as i32;

        self.vehicles[drag.vehicle_index].drag_offset = 0.0;

        let has_moved = rounded_offset != 0;
        if has_moved {
            // Save snapshot
            let positions = self.vehicles.iter().map(|veh| (veh.x, veh.y)).collect();
            self.history.push(BoardSnapshot {
                positions,
                move_count: self.move_count,
            });

            let v = &mut self.vehicles[drag.vehicle_index];
            match v.orientation {
                Orientation::Horizontal => v.x += rounded_offset,
                Orientation::Vertical => v.y += rounded_offset,
            }

            self.move_count += 1;

            let (is_player, orientation, vx, vy, vlen) =
                (v.is_player, v.orientation, v.x, v.y, v.length);

            if is_player
                && check_win_condition_raw(
                    self.exit,
                    orientation,
                    vx,
                    vy,
                    vlen,
                    self.width,
                    self.height,
                )
            {
                self.is_won = true;
                return Some(SoundTrigger::Win);
            }

            Some(SoundTrigger::Slide)
        } else if offset.abs() > 0.05 {
            Some(SoundTrigger::Bump)
        } else {
            None
        }
    }

    /// Undoes the last vehicle move. Returns true if successful.
    pub fn undo(&mut self) -> bool {
        if self.is_won || self.history.is_empty() {
            return false;
        }

        if let Some(snapshot) = self.history.pop() {
            for (veh, (saved_x, saved_y)) in self.vehicles.iter_mut().zip(snapshot.positions.iter())
            {
                veh.x = *saved_x;
                veh.y = *saved_y;
                veh.drag_offset = 0.0;
            }
            self.move_count = snapshot.move_count;
            self.active_drag = None;
            true
        } else {
            false
        }
    }

    /// Resets the board to its starting layout.
    pub fn reset(&mut self) {
        self.vehicles = self.initial_vehicles.clone();
        self.history.clear();
        self.move_count = 0;
        self.active_drag = None;
        self.is_won = false;
        self.exit_animation_progress = 0.0;
    }

    /// Updates exit drive-off animation if level is won.
    pub fn update_exit_animation(&mut self, dt: f32) -> bool {
        if self.is_won && self.exit_animation_progress < 1.0 {
            self.exit_animation_progress = (self.exit_animation_progress + dt * 1.5).min(1.0);
            true
        } else {
            false
        }
    }
}

fn check_win_condition_raw(
    exit: ExitPosition,
    orient: Orientation,
    x: i32,
    y: i32,
    length: i32,
    width: i32,
    height: i32,
) -> bool {
    match (exit.side, orient) {
        (ExitSide::Right, Orientation::Horizontal) => y == exit.row && x + length >= width,
        (ExitSide::Left, Orientation::Horizontal) => y == exit.row && x <= 0,
        (ExitSide::Bottom, Orientation::Vertical) => x == exit.col && y + length >= height,
        (ExitSide::Top, Orientation::Vertical) => x == exit.col && y <= 0,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::vehicle::VehicleKind;

    #[test]
    fn test_board_movement_and_undo() {
        let vehicles = vec![
            Vehicle::new(
                "player",
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
        ];

        let exit = ExitPosition {
            side: ExitSide::Right,
            row: 2,
            col: 0,
        };

        let mut board = Board::new(6, 6, exit, vehicles);
        assert_eq!(board.move_count, 0);

        // Compute movement bounds for player
        let (min_b, max_b) = board.compute_movement_bounds(0);
        assert_eq!(min_b, -1);
        assert_eq!(max_b, 0);

        // Move c1 downwards out of the way
        let (c1_min, c1_max) = board.compute_movement_bounds(1);
        assert_eq!(c1_min, -1);
        assert_eq!(c1_max, 3);

        board.handle_touch_down(0.0, 0.0, 0.0, 0.0, 50.0);
        board.active_drag = Some(DragState {
            vehicle_index: 1,
            start_pos: (0.0, 0.0),
            min_offset: c1_min as f32,
            max_offset: c1_max as f32,
        });
        board.vehicles[1].drag_offset = 2.0;
        let trigger = board.handle_touch_up();
        assert_eq!(trigger, Some(SoundTrigger::Slide));
        assert_eq!(board.vehicles[1].y, 3);
        assert_eq!(board.move_count, 1);

        // Player is now unblocked on the right
        let (p_min, p_max) = board.compute_movement_bounds(0);
        assert_eq!(p_min, -1);
        assert_eq!(p_max, 3);

        // Undo move
        assert!(board.undo());
        assert_eq!(board.vehicles[1].y, 1);
        assert_eq!(board.move_count, 0);
    }
}
