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

impl ExitPosition {
    /// Checks if a vehicle at the given position reaches the exit.
    pub fn is_reached(
        &self,
        orient: Orientation,
        x: i32,
        y: i32,
        length: i32,
        width: i32,
        height: i32,
    ) -> bool {
        match (self.side, orient) {
            (ExitSide::Right, Orientation::Horizontal) => y == self.row && x + length >= width,
            (ExitSide::Left, Orientation::Horizontal) => y == self.row && x <= 0,
            (ExitSide::Bottom, Orientation::Vertical) => x == self.col && y + length >= height,
            (ExitSide::Top, Orientation::Vertical) => x == self.col && y <= 0,
            _ => false,
        }
    }
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
    pub last_pos: (f32, f32),
    pub last_time: f64,
    pub velocity: f32,
    pub has_bumped_min: bool,
    pub has_bumped_max: bool,
}

impl DragState {
    pub fn new(
        vehicle_index: usize,
        start_pos: (f32, f32),
        min_offset: f32,
        max_offset: f32,
        time: f64,
    ) -> Self {
        Self {
            vehicle_index,
            start_pos,
            min_offset,
            max_offset,
            last_pos: start_pos,
            last_time: time,
            velocity: 0.0,
            has_bumped_min: false,
            has_bumped_max: false,
        }
    }
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

        let (pos, max_dim) = match orient {
            Orientation::Horizontal => (vx, self.width),
            Orientation::Vertical => (vy, self.height),
        };

        let is_free = |p: i32| match orient {
            Orientation::Horizontal => self.is_cell_free(p, vy, Some(vehicle_idx)),
            Orientation::Vertical => self.is_cell_free(vx, p, Some(vehicle_idx)),
        };

        let min_offset = -((1..=pos).take_while(|&step| is_free(pos - step)).count() as i32);
        let max_offset = (1..=(max_dim - (pos + vlen)))
            .take_while(|&step| is_free(pos + vlen - 1 + step))
            .count() as i32;

        (min_offset, max_offset)
    }

    /// Triggers a bump effect on a vehicle, setting its BumpState and returning the corresponding sound trigger.
    pub fn trigger_bump(
        &mut self,
        vehicle_idx: usize,
        impact_dir: f32,
        velocity: f32,
    ) -> Option<SoundTrigger> {
        let v = &mut self.vehicles[vehicle_idx];
        let is_emergency = v.kind.is_emergency();
        v.bump_state = Some(crate::game::vehicle::BumpState::new(
            impact_dir,
            velocity,
            is_emergency,
        ));

        if is_emergency {
            Some(SoundTrigger::Siren)
        } else {
            Some(SoundTrigger::Alarm)
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
            let cur_time = macroquad::time::get_time();
            self.active_drag = Some(DragState::new(
                vehicle_idx,
                (mouse_x, mouse_y),
                min_bound as f32,
                max_bound as f32,
                cur_time,
            ));
            true
        } else {
            false
        }
    }

    /// Handles continuous pointer movement during drag.
    /// Returns Some(SoundTrigger) if a fast slide bump occurred against an obstacle.
    pub fn handle_touch_move(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        cell_size: f32,
    ) -> Option<SoundTrigger> {
        let cur_time = macroquad::time::get_time();
        let mut do_bump = None;

        let v_idx = {
            let drag = self.active_drag.as_mut()?;
            let v_idx = drag.vehicle_index;
            let orient = self.vehicles[v_idx].orientation;

            let (cur_axis_pos, last_axis_pos, start_axis_pos) = match orient {
                Orientation::Horizontal => (mouse_x, drag.last_pos.0, drag.start_pos.0),
                Orientation::Vertical => (mouse_y, drag.last_pos.1, drag.start_pos.1),
            };

            let dt = ((cur_time - drag.last_time) as f32).clamp(0.001, 0.2);
            let instant_vel = ((cur_axis_pos - last_axis_pos) / cell_size) / dt;
            drag.velocity = drag.velocity * 0.35 + instant_vel * 0.65;
            drag.last_pos = (mouse_x, mouse_y);
            drag.last_time = cur_time;

            let raw_delta = (cur_axis_pos - start_axis_pos) / cell_size;
            let min_bound = drag.min_offset;
            let max_bound = drag.max_offset;

            let clamped_offset = raw_delta.clamp(min_bound, max_bound);
            self.vehicles[v_idx].drag_offset = clamped_offset;

            // Reset bump latch when pulling away from barriers
            if raw_delta > min_bound + 0.25 {
                drag.has_bumped_min = false;
            }
            if raw_delta < max_bound - 0.25 {
                drag.has_bumped_max = false;
            }

            // Hard slide threshold in cells/sec (only trigger on very fast slides / hard bumps)
            const HARD_BUMP_SPEED_THRESHOLD: f32 = 8.0;

            if raw_delta <= min_bound
                && !drag.has_bumped_min
                && drag.velocity < -HARD_BUMP_SPEED_THRESHOLD
            {
                drag.has_bumped_min = true;
                do_bump = Some((-1.0, drag.velocity));
            } else if raw_delta >= max_bound
                && !drag.has_bumped_max
                && drag.velocity > HARD_BUMP_SPEED_THRESHOLD
            {
                drag.has_bumped_max = true;
                do_bump = Some((1.0, drag.velocity));
            }

            v_idx
        };

        if let Some((dir, vel)) = do_bump {
            self.trigger_bump(v_idx, dir, vel)
        } else {
            None
        }
    }

    /// Handles pointer up / touch release. Snaps to grid and checks for move/win.
    pub fn handle_touch_up(&mut self) -> Option<SoundTrigger> {
        let drag = self.active_drag.take()?;
        let v_idx = drag.vehicle_index;
        let offset = self.vehicles[v_idx].drag_offset;
        let rounded_offset = offset.round() as i32;

        self.vehicles[v_idx].drag_offset = 0.0;

        const HARD_BUMP_SPEED_THRESHOLD: f32 = 8.0;

        let has_moved = rounded_offset != 0;
        if has_moved {
            // Save snapshot
            let positions = self.vehicles.iter().map(|veh| (veh.x, veh.y)).collect();
            self.history.push(BoardSnapshot {
                positions,
                move_count: self.move_count,
            });

            let v = &mut self.vehicles[v_idx];
            match v.orientation {
                Orientation::Horizontal => v.x += rounded_offset,
                Orientation::Vertical => v.y += rounded_offset,
            }

            self.move_count += 1;

            let (is_player, orientation, vx, vy, vlen) =
                (v.is_player, v.orientation, v.x, v.y, v.length);

            if is_player
                && self
                    .exit
                    .is_reached(orientation, vx, vy, vlen, self.width, self.height)
            {
                self.is_won = true;
                return Some(SoundTrigger::Win);
            }

            // Only trigger full hard bump effect if moving at high speed against an obstacle
            let (new_min, new_max) = self.compute_movement_bounds(v_idx);
            let hit_boundary =
                (rounded_offset > 0 && new_max == 0) || (rounded_offset < 0 && new_min == 0);
            if hit_boundary && drag.velocity.abs() >= HARD_BUMP_SPEED_THRESHOLD {
                let dir = if rounded_offset > 0 { 1.0 } else { -1.0 };
                let bump_sound = self.trigger_bump(v_idx, dir, drag.velocity);
                return bump_sound.or(Some(SoundTrigger::Bump));
            }

            Some(SoundTrigger::Slide)
        } else if drag.velocity.abs() >= HARD_BUMP_SPEED_THRESHOLD {
            let dir = if drag.velocity != 0.0 {
                drag.velocity.signum()
            } else if offset != 0.0 {
                offset.signum()
            } else {
                1.0
            };
            let bump_sound = self.trigger_bump(v_idx, dir, drag.velocity);
            bump_sound.or(Some(SoundTrigger::Bump))
        } else if offset.abs() > 0.08 {
            // Gentle nudge against obstacle: quiet bump sound only, no alarm or flashers
            Some(SoundTrigger::Bump)
        } else {
            None
        }
    }

    /// Undoes the last vehicle move. Returns true if successful.
    pub fn undo(&mut self) -> bool {
        if self.is_won {
            return false;
        }

        if let Some(snapshot) = self.history.pop() {
            for (veh, (saved_x, saved_y)) in self.vehicles.iter_mut().zip(&snapshot.positions) {
                veh.x = *saved_x;
                veh.y = *saved_y;
                veh.drag_offset = 0.0;
                veh.bump_state = None;
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
        for v in &mut self.vehicles {
            v.bump_state = None;
        }
        self.history.clear();
        self.move_count = 0;
        self.active_drag = None;
        self.is_won = false;
        self.exit_animation_progress = 0.0;
    }

    /// Updates active vehicle bump timers and exit drive-off animation.
    /// Returns true if any animation (bump effect or exit drive-off) is actively running.
    pub fn update(&mut self, dt: f32) -> bool {
        let mut any_animating = false;
        for v in &mut self.vehicles {
            if let Some(bump) = &mut v.bump_state {
                if bump.update(dt) {
                    any_animating = true;
                } else {
                    v.bump_state = None;
                }
            }
        }

        let exit_anim = self.update_exit_animation(dt);
        any_animating || exit_anim
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
        board.active_drag = Some(DragState::new(
            1,
            (0.0, 0.0),
            c1_min as f32,
            c1_max as f32,
            0.0,
        ));
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

    #[test]
    fn test_fast_slide_bump_and_emergency_vehicle() {
        let vehicles = vec![
            Vehicle::new(
                "police",
                VehicleKind::CarPolice,
                0,
                0,
                2,
                Orientation::Horizontal,
                false,
            ),
            Vehicle::new(
                "ambulance",
                VehicleKind::Ambulance,
                0,
                1,
                3,
                Orientation::Horizontal,
                false,
            ),
            Vehicle::new(
                "sedan",
                VehicleKind::CarSedanBlue,
                0,
                2,
                2,
                Orientation::Horizontal,
                false,
            ),
        ];

        let exit = ExitPosition {
            side: ExitSide::Right,
            row: 0,
            col: 0,
        };

        let mut board = Board::new(6, 6, exit, vehicles);

        // Standard sedan bump -> Alarm trigger
        let trigger_sedan = board.trigger_bump(2, 1.0, 5.0);
        assert_eq!(trigger_sedan, Some(SoundTrigger::Alarm));
        assert!(!board.vehicles[2].bump_state.as_ref().unwrap().is_emergency);
        assert_eq!(
            board.vehicles[2]
                .bump_state
                .as_ref()
                .unwrap()
                .impact_direction,
            1.0
        );

        // Police car bump -> Siren trigger
        let trigger_police = board.trigger_bump(0, -1.0, -6.0);
        assert_eq!(trigger_police, Some(SoundTrigger::Siren));
        let police_bump = board.vehicles[0].bump_state.clone().unwrap();
        assert!(police_bump.is_emergency);
        assert_eq!(police_bump.impact_direction, -1.0);

        // Ambulance car bump -> Siren trigger
        let trigger_ambulance = board.trigger_bump(1, 1.0, 7.0);
        assert_eq!(trigger_ambulance, Some(SoundTrigger::Siren));
        assert!(board.vehicles[1].bump_state.as_ref().unwrap().is_emergency);

        // Test bounce offset & spring oscillation behavior
        let initial_bounce = police_bump.bounce_offset();
        assert!(initial_bounce.abs() >= 0.0);

        // Advance timer past duration
        assert!(board.update(0.5));
        assert!(board.vehicles[0].bump_state.is_some());
        assert!(!board.update(2.5));
        // All bump states should have expired and reset to None
        assert!(board.vehicles[0].bump_state.is_none());
        assert!(board.vehicles[1].bump_state.is_none());
        assert!(board.vehicles[2].bump_state.is_none());
    }
}
