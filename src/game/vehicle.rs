use super::theme::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    /// Returns an iterator of grid coordinates occupied starting from (x, y) for a given length.
    pub fn cells(self, x: i32, y: i32, length: i32) -> impl Iterator<Item = (i32, i32)> {
        (0..length).map(move |i| match self {
            Self::Horizontal => (x + i, y),
            Self::Vertical => (x, y + i),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VehicleKind {
    PlayerRed,
    CarSedanBlue,
    CarTaxiYellow,
    CarHatchbackGreen,
    CarPolice,
    TruckDelivery,
    LimoWhite,
    Ambulance,
    SemiTruck,
    BusTransit,
    #[serde(other)]
    Unknown,
}

impl VehicleKind {
    pub const fn is_emergency(&self) -> bool {
        matches!(self, Self::CarPolice | Self::Ambulance)
    }

    pub const fn city_sprite_name(&self, orientation: Orientation) -> &'static str {
        match (self, orientation) {
            (Self::PlayerRed, Orientation::Horizontal) => "player_red_h",
            (Self::PlayerRed, Orientation::Vertical) => "player_red_v",

            (Self::CarSedanBlue, Orientation::Horizontal) => "car_sedan_blue_h",
            (Self::CarSedanBlue, Orientation::Vertical) => "car_sedan_blue_v",

            (Self::CarTaxiYellow, Orientation::Horizontal) => "car_taxi_yellow_h",
            (Self::CarTaxiYellow, Orientation::Vertical) => "car_taxi_yellow_v",

            (Self::CarHatchbackGreen, Orientation::Horizontal) => "car_hatchback_green_h",
            (Self::CarHatchbackGreen, Orientation::Vertical) => "car_hatchback_green_v",

            (Self::CarPolice, Orientation::Horizontal) => "car_police_h",
            (Self::CarPolice, Orientation::Vertical) => "car_police_v",

            (Self::TruckDelivery, Orientation::Horizontal) => "truck_delivery_h",
            (Self::TruckDelivery, Orientation::Vertical) => "truck_delivery_v",

            (Self::LimoWhite, Orientation::Horizontal) => "limo_white_h",
            (Self::LimoWhite, Orientation::Vertical) => "limo_white_v",

            (Self::Ambulance, Orientation::Horizontal) => "ambulance_h",
            (Self::Ambulance, Orientation::Vertical) => "ambulance_v",

            (Self::SemiTruck, Orientation::Horizontal) => "semi_truck_h",
            (Self::SemiTruck, Orientation::Vertical) => "semi_truck_v",

            (Self::BusTransit, Orientation::Horizontal) => "bus_transit_h",
            (Self::BusTransit, Orientation::Vertical) => "bus_transit_v",

            (Self::Unknown, Orientation::Horizontal) => "player_red_h",
            (Self::Unknown, Orientation::Vertical) => "player_red_v",
        }
    }

    pub const fn marine_sprite_name(&self, orientation: Orientation) -> &'static str {
        match (self, orientation) {
            (Self::PlayerRed, Orientation::Horizontal) => "ship_player_red_h",
            (Self::PlayerRed, Orientation::Vertical) => "ship_player_red_v",

            (Self::CarSedanBlue, Orientation::Horizontal) => "ship_sail_blue_h",
            (Self::CarSedanBlue, Orientation::Vertical) => "ship_sail_blue_v",

            (Self::CarTaxiYellow, Orientation::Horizontal) => "ship_taxi_yellow_h",
            (Self::CarTaxiYellow, Orientation::Vertical) => "ship_taxi_yellow_v",

            (Self::CarHatchbackGreen, Orientation::Horizontal) => "ship_tug_green_h",
            (Self::CarHatchbackGreen, Orientation::Vertical) => "ship_tug_green_v",

            (Self::CarPolice, Orientation::Horizontal) => "ship_patrol_h",
            (Self::CarPolice, Orientation::Vertical) => "ship_patrol_v",

            (Self::TruckDelivery, Orientation::Horizontal) => "ship_cargo_h",
            (Self::TruckDelivery, Orientation::Vertical) => "ship_cargo_v",

            (Self::LimoWhite, Orientation::Horizontal) => "ship_yacht_white_h",
            (Self::LimoWhite, Orientation::Vertical) => "ship_yacht_white_v",

            (Self::Ambulance, Orientation::Horizontal) => "ship_sar_rescue_h",
            (Self::Ambulance, Orientation::Vertical) => "ship_sar_rescue_v",

            (Self::SemiTruck, Orientation::Horizontal) => "ship_container_h",
            (Self::SemiTruck, Orientation::Vertical) => "ship_container_v",

            (Self::BusTransit, Orientation::Horizontal) => "ship_ferry_h",
            (Self::BusTransit, Orientation::Vertical) => "ship_ferry_v",

            (Self::Unknown, Orientation::Horizontal) => "ship_player_red_h",
            (Self::Unknown, Orientation::Vertical) => "ship_player_red_v",
        }
    }

    pub const fn railroad_sprite_name(&self, orientation: Orientation) -> &'static str {
        match (self, orientation) {
            (Self::PlayerRed, Orientation::Horizontal) => "train_locomotive_red_h",
            (Self::PlayerRed, Orientation::Vertical) => "train_locomotive_red_v",

            (Self::CarSedanBlue, Orientation::Horizontal) => "train_coach_blue_h",
            (Self::CarSedanBlue, Orientation::Vertical) => "train_coach_blue_v",

            (Self::CarTaxiYellow, Orientation::Horizontal) => "train_tanker_yellow_h",
            (Self::CarTaxiYellow, Orientation::Vertical) => "train_tanker_yellow_v",

            (Self::CarHatchbackGreen, Orientation::Horizontal) => "train_shunter_green_h",
            (Self::CarHatchbackGreen, Orientation::Vertical) => "train_shunter_green_v",

            (Self::CarPolice, Orientation::Horizontal) => "train_patrol_railcar_h",
            (Self::CarPolice, Orientation::Vertical) => "train_patrol_railcar_v",

            (Self::TruckDelivery, Orientation::Horizontal) => "train_cargo_flat_h",
            (Self::TruckDelivery, Orientation::Vertical) => "train_cargo_flat_v",

            (Self::LimoWhite, Orientation::Horizontal) => "train_luxury_pullman_h",
            (Self::LimoWhite, Orientation::Vertical) => "train_luxury_pullman_v",

            (Self::Ambulance, Orientation::Horizontal) => "train_rescue_crane_h",
            (Self::Ambulance, Orientation::Vertical) => "train_rescue_crane_v",

            (Self::SemiTruck, Orientation::Horizontal) => "train_coal_hopper_h",
            (Self::SemiTruck, Orientation::Vertical) => "train_coal_hopper_v",

            (Self::BusTransit, Orientation::Horizontal) => "train_passenger_long_h",
            (Self::BusTransit, Orientation::Vertical) => "train_passenger_long_v",

            (Self::Unknown, Orientation::Horizontal) => "train_locomotive_red_h",
            (Self::Unknown, Orientation::Vertical) => "train_locomotive_red_v",
        }
    }

    pub const fn sprite_for_theme(&self, orientation: Orientation, theme: Theme) -> &'static str {
        match theme {
            Theme::City => self.city_sprite_name(orientation),
            Theme::Marine => self.marine_sprite_name(orientation),
            Theme::Railroad => self.railroad_sprite_name(orientation),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BumpState {
    /// Direction of the impact (+1.0 along positive axis, -1.0 along negative axis).
    pub impact_direction: f32,
    /// Elapsed time since bump (in seconds).
    pub timer: f32,
    /// Total duration of visual flashing & alarm effects (in seconds).
    pub total_duration: f32,
    /// Impact intensity (clamped 0.35 to 1.0).
    pub intensity: f32,
    /// Whether this vehicle is an emergency vehicle (police / ambulance).
    pub is_emergency: bool,
    /// Whether physical spring recoil bounce is enabled (disabled in Marine theme).
    pub enable_bounce: bool,
}

impl BumpState {
    pub fn new(
        impact_direction: f32,
        velocity: f32,
        is_emergency: bool,
        enable_bounce: bool,
    ) -> Self {
        let intensity = (velocity.abs() / 9.0).clamp(0.6, 1.0);
        let total_duration = if is_emergency { 2.2 } else { 1.3 };
        Self {
            impact_direction: if impact_direction >= 0.0 { 1.0 } else { -1.0 },
            timer: 0.0,
            total_duration,
            intensity,
            is_emergency,
            enable_bounce,
        }
    }

    /// Computes physical spring recoil bounce offset (in grid fraction units).
    /// Bounces opposite to the impact direction with a visible damped oscillation over ~0.35s.
    /// Returns 0.0 if bounce is disabled (e.g. in Marine theme).
    pub fn bounce_offset(&self) -> f32 {
        if !self.enable_bounce {
            return 0.0;
        }
        const DURATION: f32 = 0.35;
        if self.timer >= DURATION {
            return 0.0;
        }
        let t = self.timer / DURATION;
        // Damped harmonic recoil: reaches peak recoil away from obstacle, then springs back
        let wave = (t * std::f32::consts::PI * 2.8).sin();
        let decay = (1.0 - t).powi(2);
        -self.impact_direction * 0.28 * self.intensity * wave * decay
    }

    /// Computes squash and stretch scale factors `(scale_along_axis, scale_perpendicular)`.
    /// On initial impact (t < 0.12s), the vehicle slightly compresses against the obstacle.
    pub fn squash_factors(&self) -> (f32, f32) {
        const SQUASH_TIME: f32 = 0.12;
        if self.timer >= SQUASH_TIME {
            return (1.0, 1.0);
        }
        let t = self.timer / SQUASH_TIME;
        let compression = (t * std::f32::consts::PI).sin() * 0.07 * self.intensity;
        (1.0 - compression, 1.0 + compression * 0.35)
    }

    /// Returns true if headlights / hazard lights are currently lit in the blink cycle.
    pub fn is_hazard_on(&self) -> bool {
        let blink_phase = (self.timer * 9.0 * std::f32::consts::PI).sin();
        blink_phase > 0.0
    }

    /// Phase of emergency vehicle rooftop strobe cycle (0.0 to 1.0).
    pub fn emergency_strobe_phase(&self) -> f32 {
        (self.timer * 6.0) % 1.0
    }

    /// Advance timer by `dt`. Returns true if effect is still active.
    pub fn update(&mut self, dt: f32) -> bool {
        self.timer += dt;
        self.timer < self.total_duration
    }
}

/// Hydrodynamic water drift state when a floating vessel is struck by another vessel.
#[derive(Debug, Clone, PartialEq)]
pub struct DriftState {
    /// 2D direction of the drift push `(push_x, push_y)` in grid units.
    pub push_dir: (f32, f32),
    /// Elapsed time since impact (in seconds).
    pub timer: f32,
    /// Total duration of the water drift animation (in seconds).
    pub total_duration: f32,
    /// Impact intensity (clamped 0.35 to 1.0).
    pub intensity: f32,
}

impl DriftState {
    pub fn new(push_x: f32, push_y: f32, intensity: f32) -> Self {
        Self {
            push_dir: (push_x, push_y),
            timer: 0.0,
            total_duration: 0.7,
            intensity: intensity.clamp(0.35, 1.0),
        }
    }

    /// Computes current 2D visual drift displacement `(drift_x, drift_y)` in grid fraction units.
    pub fn offset(&self) -> (f32, f32) {
        if self.timer >= self.total_duration {
            return (0.0, 0.0);
        }
        let t = (self.timer / self.total_duration).clamp(0.0, 1.0);
        let envelope = (1.0 - t).powf(1.8);
        let wave = (t * std::f32::consts::PI * 1.8).sin();
        let amount = wave * envelope * 0.22 * self.intensity;
        (self.push_dir.0 * amount, self.push_dir.1 * amount)
    }

    /// Computes subtle rotational rocking roll (in radians) induced by the impact drift.
    pub fn roll(&self) -> f32 {
        if self.timer >= self.total_duration {
            return 0.0;
        }
        let t = (self.timer / self.total_duration).clamp(0.0, 1.0);
        let envelope = (1.0 - t).powi(2);
        let wave = (t * std::f32::consts::PI * 2.2).sin();
        let dir = if self.push_dir.0 != 0.0 {
            self.push_dir.0
        } else {
            self.push_dir.1
        };
        wave * envelope * 0.045 * self.intensity * dir
    }

    /// Advance timer by `dt`. Returns true if drift effect is still active.
    pub fn update(&mut self, dt: f32) -> bool {
        self.timer += dt;
        self.timer < self.total_duration
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vehicle {
    pub kind: VehicleKind,
    pub x: i32,
    pub y: i32,
    pub length: i32,
    pub orientation: Orientation,
    #[serde(default)]
    pub is_player: bool,
    #[serde(skip)]
    pub drag_offset: f32,
    #[serde(skip)]
    pub bump_state: Option<BumpState>,
    #[serde(skip)]
    pub drift_state: Option<DriftState>,
}

impl Vehicle {
    #[allow(dead_code)]
    pub fn new(
        kind: VehicleKind,
        x: i32,
        y: i32,
        length: i32,
        orientation: Orientation,
        is_player: bool,
    ) -> Self {
        Self {
            kind,
            x,
            y,
            length,
            orientation,
            is_player,
            drag_offset: 0.0,
            bump_state: None,
            drift_state: None,
        }
    }

    /// Returns the physical inertia mass of the vehicle based on its length and type.
    /// The longer the vehicle is, the more inertia and mass it possesses.
    pub fn mass(&self) -> f32 {
        match self.length {
            1 => 0.8,
            2 => 1.0,
            3 => 1.8,
            4 => 2.7,
            len => len as f32 * 0.7,
        }
    }

    /// Checks whether a given grid coordinate is occupied by this vehicle in O(1).
    pub fn contains_cell(&self, cell_x: i32, cell_y: i32) -> bool {
        match self.orientation {
            Orientation::Horizontal => {
                cell_y == self.y && cell_x >= self.x && cell_x < self.x + self.length
            }
            Orientation::Vertical => {
                cell_x == self.x && cell_y >= self.y && cell_y < self.y + self.length
            }
        }
    }

    /// Returns the pixel bounding box for rendering and touch hit testing.
    pub fn pixel_bounds(
        &self,
        origin_x: f32,
        origin_y: f32,
        cell_size: f32,
    ) -> (f32, f32, f32, f32) {
        let bounce = self.bump_state.as_ref().map_or(0.0, |b| b.bounce_offset());
        let (drift_x, drift_y) = self.drift_state.as_ref().map_or((0.0, 0.0), |d| d.offset());
        let total_offset = self.drag_offset + bounce;

        let (px, py) = match self.orientation {
            Orientation::Horizontal => (
                origin_x + (self.x as f32 + total_offset + drift_x) * cell_size,
                origin_y + (self.y as f32 + drift_y) * cell_size,
            ),
            Orientation::Vertical => (
                origin_x + (self.x as f32 + drift_x) * cell_size,
                origin_y + (self.y as f32 + total_offset + drift_y) * cell_size,
            ),
        };

        let (w, h) = match self.orientation {
            Orientation::Horizontal => (self.length as f32 * cell_size, cell_size),
            Orientation::Vertical => (cell_size, self.length as f32 * cell_size),
        };

        (px, py, w, h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bump_state_enable_bounce_flag() {
        let city_bump = BumpState::new(1.0, 6.0, false, true);
        let marine_bump = BumpState::new(1.0, 6.0, false, false);

        assert!(city_bump.bounce_offset().abs() >= 0.0);
        // Marine bump with enable_bounce = false should strictly return 0.0
        assert_eq!(marine_bump.bounce_offset(), 0.0);
    }

    #[test]
    fn test_drift_state_lifecycle_and_offset() {
        let mut drift = DriftState::new(1.0, 0.0, 0.8);
        assert_eq!(drift.push_dir, (1.0, 0.0));

        // Advance 0.15s
        assert!(drift.update(0.15));
        let (ox, oy) = drift.offset();
        assert!(ox > 0.0, "Drift offset X ({}) should be positive", ox);
        assert_eq!(oy, 0.0);
        assert!(drift.roll().abs() > 0.0);

        // Advance past total duration
        assert!(!drift.update(0.7));
        let (end_ox, end_oy) = drift.offset();
        assert_eq!(end_ox, 0.0);
        assert_eq!(end_oy, 0.0);
        assert_eq!(drift.roll(), 0.0);
    }

    #[test]
    fn test_railroad_sprite_names() {
        assert_eq!(
            VehicleKind::PlayerRed.sprite_for_theme(Orientation::Horizontal, Theme::Railroad),
            "train_locomotive_red_h"
        );
        assert_eq!(
            VehicleKind::PlayerRed.sprite_for_theme(Orientation::Vertical, Theme::Railroad),
            "train_locomotive_red_v"
        );
        assert_eq!(
            VehicleKind::CarSedanBlue.sprite_for_theme(Orientation::Horizontal, Theme::Railroad),
            "train_coach_blue_h"
        );
        assert_eq!(
            VehicleKind::SemiTruck.sprite_for_theme(Orientation::Horizontal, Theme::Railroad),
            "train_coal_hopper_h"
        );
    }
}
