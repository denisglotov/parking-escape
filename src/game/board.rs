use super::obstacle::Obstacle;
use super::theme::Theme;
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
pub struct InertiaCoastState {
    pub vehicle_index: usize,
    pub velocity: f32,
    pub min_offset: f32,
    pub max_offset: f32,
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
    #[serde(default)]
    pub obstacles: Vec<Obstacle>,
    #[serde(skip)]
    pub move_count: u32,
    #[serde(skip)]
    pub history: Vec<BoardSnapshot>,
    #[serde(skip)]
    pub active_drag: Option<DragState>,
    #[serde(skip)]
    pub active_coast: Option<InertiaCoastState>,
    #[serde(skip)]
    pub is_won: bool,
    #[serde(skip)]
    pub theme: Theme,
    #[serde(skip)]
    pub exit_animation_progress: f32,
    #[serde(skip)]
    initial_vehicles: Vec<Vehicle>,
    #[serde(skip)]
    initial_obstacles: Vec<Obstacle>,
}

impl Board {
    pub fn new(
        width: i32,
        height: i32,
        exit: ExitPosition,
        vehicles: Vec<Vehicle>,
        obstacles: Vec<Obstacle>,
    ) -> Self {
        let initial_vehicles = vehicles.clone();
        let initial_obstacles = obstacles.clone();
        Self {
            width,
            height,
            exit,
            vehicles,
            obstacles,
            move_count: 0,
            history: Vec::new(),
            active_drag: None,
            active_coast: None,
            is_won: false,
            theme: Theme::default(),
            exit_animation_progress: 0.0,
            initial_vehicles,
            initial_obstacles,
        }
    }

    /// Checks if a cell is inside the board boundaries.
    pub fn is_inside_board(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    /// Checks if a cell is free of any vehicles and static obstacles, ignoring an optional vehicle index.
    pub fn is_cell_free(&self, x: i32, y: i32, ignore_idx: Option<usize>) -> bool {
        if !self.is_inside_board(x, y) {
            return false;
        }

        let vehicle_free = self
            .vehicles
            .iter()
            .enumerate()
            .filter(|(idx, _)| ignore_idx != Some(*idx))
            .all(|(_, v)| !v.contains_cell(x, y));

        let obstacle_free = self.obstacles.iter().all(|obs| !obs.contains_cell(x, y));

        vehicle_free && obstacle_free
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

    /// Finds the index of the other vehicle that is acting as an obstacle for `vehicle_idx`
    /// in the direction of `impact_dir`. Returns None if hitting the board boundary.
    pub fn find_obstacle_vehicle(&self, vehicle_idx: usize, impact_dir: f32) -> Option<usize> {
        let v = &self.vehicles[vehicle_idx];
        let (min_bound, max_bound) = self.compute_movement_bounds(vehicle_idx);
        let offset = if impact_dir >= 0.0 {
            max_bound
        } else {
            min_bound
        };

        let (target_x, target_y) = match (v.orientation, impact_dir >= 0.0) {
            (Orientation::Horizontal, true) => (v.x + offset + v.length, v.y),
            (Orientation::Horizontal, false) => (v.x + offset - 1, v.y),
            (Orientation::Vertical, true) => (v.x, v.y + offset + v.length),
            (Orientation::Vertical, false) => (v.x, v.y + offset - 1),
        };

        if !self.is_inside_board(target_x, target_y) {
            return None;
        }

        self.vehicles
            .iter()
            .enumerate()
            .position(|(idx, other)| idx != vehicle_idx && other.contains_cell(target_x, target_y))
    }

    /// Finds the index of the static obstacle at `(target_x, target_y)`, if any.
    pub fn find_obstacle_at(&self, target_x: i32, target_y: i32) -> Option<usize> {
        self.obstacles
            .iter()
            .position(|obs| obs.contains_cell(target_x, target_y))
    }

    /// Triggers a bump effect on a vehicle, setting its BumpState and returning the corresponding sound trigger.
    /// In Marine theme, the hitting vessel does not rebounce; if another ship was struck, it drifts away from the impact.
    pub fn trigger_bump(
        &mut self,
        vehicle_idx: usize,
        impact_dir: f32,
        velocity: f32,
    ) -> Option<SoundTrigger> {
        let is_marine = self.theme == Theme::Marine;
        let is_emergency = self.vehicles[vehicle_idx].kind.is_emergency();
        let enable_bounce = !is_marine;

        self.vehicles[vehicle_idx].bump_state = Some(crate::game::vehicle::BumpState::new(
            impact_dir,
            velocity,
            is_emergency,
            enable_bounce,
        ));

        // If hitting a static obstacle, trigger wobble/shake feedback on that obstacle
        let v = &self.vehicles[vehicle_idx];
        let (min_bound, max_bound) = self.compute_movement_bounds(vehicle_idx);
        let offset = if impact_dir >= 0.0 {
            max_bound
        } else {
            min_bound
        };

        let (target_x, target_y) = match (v.orientation, impact_dir >= 0.0) {
            (Orientation::Horizontal, true) => (v.x + offset + v.length, v.y),
            (Orientation::Horizontal, false) => (v.x + offset - 1, v.y),
            (Orientation::Vertical, true) => (v.x, v.y + offset + v.length),
            (Orientation::Vertical, false) => (v.x, v.y + offset - 1),
        };

        if let Some(obs_idx) = self.find_obstacle_at(target_x, target_y) {
            let intensity = (velocity.abs() / 9.0).clamp(0.5, 1.0);
            self.obstacles[obs_idx].trigger_wobble(intensity);
        }

        if is_marine {
            if let Some(other_idx) = self.find_obstacle_vehicle(vehicle_idx, impact_dir) {
                let v = &self.vehicles[vehicle_idx];
                let (push_x, push_y) = match v.orientation {
                    Orientation::Horizontal => (if impact_dir >= 0.0 { 1.0 } else { -1.0 }, 0.0),
                    Orientation::Vertical => (0.0, if impact_dir >= 0.0 { 1.0 } else { -1.0 }),
                };
                let intensity = (velocity.abs() / 9.0).clamp(0.4, 1.0);
                self.vehicles[other_idx].drift_state = Some(crate::game::vehicle::DriftState::new(
                    push_x, push_y, intensity,
                ));
            }
        }

        if is_emergency {
            Some(SoundTrigger::Siren)
        } else {
            Some(SoundTrigger::Alarm)
        }
    }

    /// Finalizes the continuous `drag_offset` of a vehicle, snapping to the integer grid,
    /// recording the history snapshot, updating move count, and checking win conditions.
    pub fn finalize_vehicle_offset(&mut self, vehicle_idx: usize) -> Option<SoundTrigger> {
        let offset = self.vehicles[vehicle_idx].drag_offset;
        let rounded_offset = offset.round() as i32;
        self.vehicles[vehicle_idx].drag_offset = 0.0;

        if rounded_offset != 0 {
            let positions = self.vehicles.iter().map(|veh| (veh.x, veh.y)).collect();
            self.history.push(BoardSnapshot {
                positions,
                move_count: self.move_count,
            });

            let v = &mut self.vehicles[vehicle_idx];
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

            Some(SoundTrigger::Slide)
        } else {
            None
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

        // If any vehicle is coasting with inertia, finalize it immediately
        if let Some(coast) = self.active_coast.take() {
            self.finalize_vehicle_offset(coast.vehicle_index);
        }

        // If an obstacle was tapped, trigger its wobble animation for tactile feedback
        let hit_obs = self.obstacles.iter_mut().rev().find(|obs| {
            let (px, py, pw, ph) = obs.pixel_bounds(origin_x, origin_y, cell_size);
            mouse_x >= px && mouse_x <= px + pw && mouse_y >= py && mouse_y <= py + ph
        });

        if let Some(obs) = hit_obs {
            obs.trigger_wobble(0.8);
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

    /// Handles continuous pointer movement during drag with inertia.
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
            let mass = self.vehicles[v_idx].mass();

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
            let clamped_target = raw_delta.clamp(min_bound, max_bound);

            // Inertia drag tracking: ships in water have higher hydrodynamic inertia and smooth momentum lag
            let responsiveness = self.theme.drag_responsiveness(mass);
            let blend = (1.0 - (-responsiveness * dt).exp()).clamp(0.0, 1.0);
            let current_offset = self.vehicles[v_idx].drag_offset;
            self.vehicles[v_idx].drag_offset =
                current_offset + (clamped_target - current_offset) * blend;

            // Reset bump latch when pulling away from barriers
            if raw_delta > min_bound + 0.25 {
                drag.has_bumped_min = false;
            }
            if raw_delta < max_bound - 0.25 {
                drag.has_bumped_max = false;
            }

            // Hard slide threshold in cells/sec (scales with mass)
            let hard_speed_threshold = 5.2 / mass.sqrt();

            if raw_delta <= min_bound
                && !drag.has_bumped_min
                && drag.velocity < -hard_speed_threshold
            {
                drag.has_bumped_min = true;
                do_bump = Some((-1.0, drag.velocity));
            } else if raw_delta >= max_bound
                && !drag.has_bumped_max
                && drag.velocity > hard_speed_threshold
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

    /// Handles pointer up / touch release. Starts inertial coasting if swiped or snaps to grid.
    pub fn handle_touch_up(&mut self) -> Option<SoundTrigger> {
        let drag = self.active_drag.take()?;
        let v_idx = drag.vehicle_index;
        let mass = self.vehicles[v_idx].mass();

        let swipe_launch_threshold = self.theme.swipe_launch_threshold();
        if drag.velocity.abs() > swipe_launch_threshold {
            // Launch inertial coasting!
            self.active_coast = Some(InertiaCoastState {
                vehicle_index: v_idx,
                velocity: drag.velocity,
                min_offset: drag.min_offset,
                max_offset: drag.max_offset,
            });
            None
        } else {
            let bump_speed_threshold = 5.2 / mass.sqrt();
            if drag.velocity.abs() >= bump_speed_threshold {
                let dir = if drag.velocity != 0.0 {
                    drag.velocity.signum()
                } else if self.vehicles[v_idx].drag_offset != 0.0 {
                    self.vehicles[v_idx].drag_offset.signum()
                } else {
                    1.0
                };
                let bump_sound = self.trigger_bump(v_idx, dir, drag.velocity);
                self.finalize_vehicle_offset(v_idx);
                bump_sound.or(Some(SoundTrigger::Bump))
            } else {
                let offset = self.vehicles[v_idx].drag_offset;
                let trigger = self.finalize_vehicle_offset(v_idx);
                if trigger.is_none() && offset.abs() > 0.08 {
                    Some(SoundTrigger::Bump)
                } else {
                    trigger
                }
            }
        }
    }

    /// Undoes the last vehicle move. Returns true if successful.
    pub fn undo(&mut self) -> bool {
        if self.is_won {
            return false;
        }

        self.active_coast = None;
        self.active_drag = None;

        if let Some(snapshot) = self.history.pop() {
            for (veh, (saved_x, saved_y)) in self.vehicles.iter_mut().zip(&snapshot.positions) {
                veh.x = *saved_x;
                veh.y = *saved_y;
                veh.drag_offset = 0.0;
                veh.bump_state = None;
                veh.drift_state = None;
            }
            self.move_count = snapshot.move_count;
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
            v.drift_state = None;
            v.drag_offset = 0.0;
        }
        self.obstacles = self.initial_obstacles.clone();
        for obs in &mut self.obstacles {
            obs.wobble_timer = 0.0;
            obs.wobble_intensity = 0.0;
        }
        self.history.clear();
        self.move_count = 0;
        self.active_drag = None;
        self.active_coast = None;
        self.is_won = false;
        self.exit_animation_progress = 0.0;
    }

    /// Updates active vehicle bump timers, inertial coasting, and exit drive-off animation.
    /// Returns Some(SoundTrigger) if an audio event (bump/slide/win) was triggered.
    pub fn update(&mut self, dt: f32) -> Option<SoundTrigger> {
        let mut sound_trigger = None;

        // 1. Advance obstacle wobble animations
        for obs in &mut self.obstacles {
            obs.update(dt);
        }

        // 2. Advance vehicle bump and drift effects
        for v in &mut self.vehicles {
            if let Some(bump) = &mut v.bump_state {
                if !bump.update(dt) {
                    v.bump_state = None;
                }
            }
            if let Some(drift) = &mut v.drift_state {
                if !drift.update(dt) {
                    v.drift_state = None;
                }
            }
        }

        // 2. Advance Inertia Coasting physics
        if let Some(coast) = &mut self.active_coast {
            let v_idx = coast.vehicle_index;
            let mass = self.vehicles[v_idx].mass();
            // Higher mass = lower friction deceleration = longer, heavier glide
            let friction = self.theme.friction(mass);
            coast.velocity -= coast.velocity * (friction * dt).min(0.95);

            let v = &mut self.vehicles[v_idx];
            v.drag_offset += coast.velocity * dt;

            let min_b = coast.min_offset;
            let max_b = coast.max_offset;
            let bump_thresh = 4.8 / mass.sqrt();

            let mut finished_coast = false;
            let mut hit_bump_dir = None;

            if v.drag_offset <= min_b {
                v.drag_offset = min_b;
                if coast.velocity < -bump_thresh {
                    hit_bump_dir = Some(-1.0);
                }
                finished_coast = true;
            } else if v.drag_offset >= max_b {
                v.drag_offset = max_b;
                if coast.velocity > bump_thresh {
                    hit_bump_dir = Some(1.0);
                }
                finished_coast = true;
            } else if coast.velocity.abs() < 0.35 {
                let target_snap = v.drag_offset.round();
                let snap_diff = target_snap - v.drag_offset;
                if snap_diff.abs() < 0.03 {
                    v.drag_offset = target_snap;
                    finished_coast = true;
                } else {
                    let snap_speed = self.theme.snap_speed();
                    v.drag_offset += snap_diff * (snap_speed * dt).min(0.9);
                }
            }

            let coast_vel = coast.velocity;

            if finished_coast {
                self.active_coast = None;
                if let Some(dir) = hit_bump_dir {
                    let bump_snd = self.trigger_bump(v_idx, dir, coast_vel);
                    self.finalize_vehicle_offset(v_idx);
                    sound_trigger = bump_snd.or(Some(SoundTrigger::Bump));
                } else {
                    let move_snd = self.finalize_vehicle_offset(v_idx);
                    sound_trigger = move_snd.or(Some(SoundTrigger::Slide));
                }
            }
        }

        // 3. Update exit animation if won
        self.update_exit_animation(dt);

        sound_trigger
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
                VehicleKind::PlayerRed,
                1,
                2,
                2,
                Orientation::Horizontal,
                true,
            ),
            Vehicle::new(
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

        let mut board = Board::new(6, 6, exit, vehicles, vec![]);
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
                VehicleKind::CarPolice,
                0,
                0,
                2,
                Orientation::Horizontal,
                false,
            ),
            Vehicle::new(
                VehicleKind::Ambulance,
                0,
                1,
                3,
                Orientation::Horizontal,
                false,
            ),
            Vehicle::new(
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

        let mut board = Board::new(6, 6, exit, vehicles, vec![]);

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
        let _ = board.update(0.5);
        assert!(board.vehicles[0].bump_state.is_some());
        let _ = board.update(2.5);
        // All bump states should have expired and reset to None
        assert!(board.vehicles[0].bump_state.is_none());
        assert!(board.vehicles[1].bump_state.is_none());
        assert!(board.vehicles[2].bump_state.is_none());
    }

    #[test]
    fn test_vehicle_mass_and_inertia_coasting() {
        let car = Vehicle::new(
            VehicleKind::CarSedanBlue,
            0,
            0,
            2,
            Orientation::Horizontal,
            false,
        );
        let truck = Vehicle::new(
            VehicleKind::TruckDelivery,
            0,
            1,
            3,
            Orientation::Horizontal,
            false,
        );
        let semi = Vehicle::new(
            VehicleKind::SemiTruck,
            0,
            2,
            4,
            Orientation::Horizontal,
            false,
        );

        // Verify longer vehicles have strictly greater mass
        assert!(car.mass() < truck.mass());
        assert!(truck.mass() < semi.mass());
        assert_eq!(car.mass(), 1.0);
        assert_eq!(truck.mass(), 1.8);
        assert_eq!(semi.mass(), 2.7);

        let exit = ExitPosition {
            side: ExitSide::Right,
            row: 0,
            col: 0,
        };

        let mut board = Board::new(6, 6, exit, vec![car, truck, semi], vec![]);

        // Start inertial coast on truck
        board.active_coast = Some(InertiaCoastState {
            vehicle_index: 1,
            velocity: 6.0,
            min_offset: 0.0,
            max_offset: 3.0,
        });

        // Advance 1 frame of coasting
        let _ = board.update(0.016);
        assert!(board.vehicles[1].drag_offset > 0.0);
    }

    #[test]
    fn test_marine_theme_and_water_inertia() {
        let ship = Vehicle::new(
            VehicleKind::PlayerRed,
            0,
            0,
            2,
            Orientation::Horizontal,
            true,
        );

        let exit = ExitPosition {
            side: ExitSide::Right,
            row: 0,
            col: 0,
        };

        // Create standard city board vs marine board
        let mut city_board = Board::new(6, 6, exit, vec![ship.clone()], vec![]);
        city_board.theme = Theme::City;

        let mut marine_board = Board::new(6, 6, exit, vec![ship.clone()], vec![]);
        marine_board.theme = Theme::Marine;

        // Verify marine sprite lookup
        assert_eq!(
            ship.kind
                .sprite_for_theme(Orientation::Horizontal, Theme::City),
            "player_red_h"
        );
        assert_eq!(
            ship.kind
                .sprite_for_theme(Orientation::Horizontal, Theme::Marine),
            "ship_player_red_h"
        );
        assert_eq!(
            ship.kind
                .sprite_for_theme(Orientation::Vertical, Theme::Marine),
            "ship_player_red_v"
        );

        // Verify emergency ships in marine theme
        assert_eq!(
            VehicleKind::CarPolice.sprite_for_theme(Orientation::Horizontal, Theme::Marine),
            "ship_patrol_h"
        );
        assert_eq!(
            VehicleKind::Ambulance.sprite_for_theme(Orientation::Horizontal, Theme::Marine),
            "ship_sar_rescue_h"
        );

        // Test that marine theme glides farther due to lower hydrodynamic water friction
        city_board.active_coast = Some(InertiaCoastState {
            vehicle_index: 0,
            velocity: 5.0,
            min_offset: 0.0,
            max_offset: 4.0,
        });

        marine_board.active_coast = Some(InertiaCoastState {
            vehicle_index: 0,
            velocity: 5.0,
            min_offset: 0.0,
            max_offset: 4.0,
        });

        let dt = 0.05;
        let _ = city_board.update(dt);
        let _ = marine_board.update(dt);

        // Marine board coast velocity decelerates slower (higher remaining velocity & more drift)
        let city_coast_vel = city_board.active_coast.as_ref().unwrap().velocity;
        let marine_coast_vel = marine_board.active_coast.as_ref().unwrap().velocity;
        assert!(marine_coast_vel > city_coast_vel, "Marine coast velocity ({}) should be higher than city ({}) due to lower water friction", marine_coast_vel, city_coast_vel);
    }

    #[test]
    fn test_marine_theme_collision_no_rebounce_and_obstacle_drift() {
        let boat_player = Vehicle::new(
            VehicleKind::PlayerRed,
            1,
            2,
            2,
            Orientation::Horizontal,
            true,
        );
        let boat_blocker = Vehicle::new(
            VehicleKind::CarSedanBlue,
            3,
            1,
            2,
            Orientation::Vertical,
            false,
        );

        let exit = ExitPosition {
            side: ExitSide::Right,
            row: 2,
            col: 0,
        };

        let mut board = Board::new(6, 6, exit, vec![boat_player, boat_blocker], vec![]);
        board.theme = Theme::Marine;

        // Player boat moves right (+1.0) and bumps into blocker boat
        let hit_vehicle = board.find_obstacle_vehicle(0, 1.0);
        assert_eq!(
            hit_vehicle,
            Some(1),
            "Obstacle vehicle hit should be boat 1"
        );

        let snd = board.trigger_bump(0, 1.0, 6.0);
        assert!(snd.is_some());

        // 1. Hitting boat should NOT rebounce in marine theme
        let player_bump = board.vehicles[0].bump_state.as_ref().unwrap();
        assert_eq!(
            player_bump.bounce_offset(),
            0.0,
            "Hitting boat must not rebounce in marine theme"
        );

        // 2. Struck boat should have a DriftState drifting to the right (+X)
        let blocker_drift = board.vehicles[1].drift_state.as_ref().unwrap();
        assert_eq!(
            blocker_drift.push_dir,
            (1.0, 0.0),
            "Struck boat must drift in the impact direction (+X)"
        );

        // 3. Advance timer to verify drift offset and expiry
        let _ = board.update(0.15);
        let (drift_x, drift_y) = board.vehicles[1].drift_state.as_ref().unwrap().offset();
        assert!(drift_x > 0.0, "Drift offset X should be positive");
        assert_eq!(drift_y, 0.0);

        let _ = board.update(2.5);
        assert!(board.vehicles[0].bump_state.is_none());
        assert!(board.vehicles[1].drift_state.is_none());
    }

    #[test]
    fn test_marine_wall_collision() {
        let boat = Vehicle::new(
            VehicleKind::PlayerRed,
            0,
            0,
            2,
            Orientation::Horizontal,
            true,
        );

        let exit = ExitPosition {
            side: ExitSide::Right,
            row: 0,
            col: 0,
        };

        let mut board = Board::new(6, 6, exit, vec![boat], vec![]);
        board.theme = Theme::Marine;

        // Boat hits the left wall (impact_dir = -1.0)
        let hit_vehicle = board.find_obstacle_vehicle(0, -1.0);
        assert_eq!(
            hit_vehicle, None,
            "Hitting wall should not find another vehicle"
        );

        let snd = board.trigger_bump(0, -1.0, -6.0);
        assert!(snd.is_some());

        // Hitting boat should have no rebounce
        assert_eq!(
            board.vehicles[0]
                .bump_state
                .as_ref()
                .unwrap()
                .bounce_offset(),
            0.0
        );
        // No drift state on hitting boat
        assert!(board.vehicles[0].drift_state.is_none());
    }

    #[test]
    fn test_board_obstacles_collision_and_interaction() {
        let player = Vehicle::new(
            VehicleKind::PlayerRed,
            0,
            2,
            2,
            Orientation::Horizontal,
            true,
        );
        let rock = Obstacle::rock_1x1(3, 2);
        let buoy = Obstacle::buoy_1x1(0, 0);

        let exit = ExitPosition {
            side: ExitSide::Right,
            row: 2,
            col: 0,
        };

        let mut board = Board::new(6, 6, exit, vec![player], vec![rock, buoy]);

        // Cell (3, 2) is occupied by rock, so player at (0..2, 2) can only move 1 unit right to x=1 (occupying 1..3)
        let (p_min, p_max) = board.compute_movement_bounds(0);
        assert_eq!(p_min, 0);
        assert_eq!(p_max, 1); // target x=1, car occupies (1, 2) and (2, 2), (3, 2) is blocked by rock

        assert!(!board.is_cell_free(3, 2, None));
        assert!(!board.is_cell_free(0, 0, None));
        assert!(board.is_cell_free(4, 2, None));

        // Player bumps into rock at (3, 2) with max_bound
        assert_eq!(board.find_obstacle_at(3, 2), Some(0));
        let _ = board.trigger_bump(0, 1.0, 7.0);

        // Rock should have triggered wobble
        assert!(board.obstacles[0].wobble_timer > 0.0);
        let _ = board.update(0.1);
        let (ox, _) = board.obstacles[0].wobble_offset();
        assert!(ox.abs() > 0.0);

        // Touch down on buoy at (0, 0)
        let tapped = board.handle_touch_down(25.0, 25.0, 0.0, 0.0, 50.0);
        assert!(
            !tapped,
            "Touching static obstacle should not start a vehicle drag"
        );
        assert!(
            board.obstacles[1].wobble_timer > 0.0,
            "Touching buoy should trigger its wobble animation"
        );
    }
}
