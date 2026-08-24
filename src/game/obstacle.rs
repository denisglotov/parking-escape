use super::theme::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObstacleKind {
    Rock,
    Buoy,
    Pillar,
    Barrier,
    #[serde(other)]
    Unknown,
}

impl ObstacleKind {
    /// Returns the default obstacle kind for a given theme if unspecified.
    pub const fn default_for_theme(theme: Theme) -> Self {
        match theme {
            Theme::Marine => Self::Buoy,
            Theme::City => Self::Rock,
            Theme::Railroad => Self::Barrier,
        }
    }

    pub const fn city_sprite_name(&self) -> &'static str {
        match self {
            Self::Rock => "city_rock",
            Self::Pillar => "city_pillar",
            Self::Barrier => "city_barrier",
            Self::Buoy => "city_pillar",
            Self::Unknown => "city_rock",
        }
    }

    pub const fn marine_sprite_name(&self) -> &'static str {
        match self {
            Self::Rock => "marine_rock",
            Self::Barrier => "marine_rock",
            Self::Buoy => "marine_buoy",
            Self::Pillar => "marine_buoy",
            Self::Unknown => "marine_buoy",
        }
    }

    pub const fn railroad_sprite_name(&self) -> &'static str {
        match self {
            Self::Rock => "railroad_coal_pile",
            Self::Barrier => "railroad_buffer_stop",
            Self::Buoy => "railroad_buffer_stop",
            Self::Pillar => "railroad_semaphore",
            Self::Unknown => "railroad_buffer_stop",
        }
    }

    pub const fn sprite_for_theme(&self, theme: Theme) -> &'static str {
        match theme {
            Theme::City => self.city_sprite_name(),
            Theme::Marine => self.marine_sprite_name(),
            Theme::Railroad => self.railroad_sprite_name(),
        }
    }
}

const fn default_obstacle_size() -> i32 {
    1
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Obstacle {
    pub x: i32,
    pub y: i32,
    #[serde(default = "default_obstacle_size")]
    pub width: i32,
    #[serde(default = "default_obstacle_size")]
    pub height: i32,
    #[serde(default)]
    pub kind: Option<ObstacleKind>,
    #[serde(skip)]
    pub wobble_timer: f32,
    #[serde(skip)]
    pub wobble_intensity: f32,
}

#[allow(dead_code)]
impl Obstacle {
    pub fn new(x: i32, y: i32, width: i32, height: i32, kind: Option<ObstacleKind>) -> Self {
        Self {
            x,
            y,
            width: width.max(1),
            height: height.max(1),
            kind,
            wobble_timer: 0.0,
            wobble_intensity: 0.0,
        }
    }

    /// Creates a standard 1x1 rock obstacle.
    pub fn rock_1x1(x: i32, y: i32) -> Self {
        Self::new(x, y, 1, 1, Some(ObstacleKind::Rock))
    }

    /// Creates a standard 1x1 marine navigation buoy obstacle.
    pub fn buoy_1x1(x: i32, y: i32) -> Self {
        Self::new(x, y, 1, 1, Some(ObstacleKind::Buoy))
    }

    /// Returns the sprite name for this obstacle given the active theme.
    pub fn sprite_name(&self, theme: Theme) -> &'static str {
        self.kind
            .unwrap_or_else(|| ObstacleKind::default_for_theme(theme))
            .sprite_for_theme(theme)
    }

    /// Checks whether a given grid coordinate is occupied by this obstacle in O(1).
    pub fn contains_cell(&self, cell_x: i32, cell_y: i32) -> bool {
        cell_x >= self.x
            && cell_x < self.x + self.width
            && cell_y >= self.y
            && cell_y < self.y + self.height
    }

    /// Returns an iterator over all grid coordinates occupied by this obstacle.
    pub fn cells(&self) -> impl Iterator<Item = (i32, i32)> {
        let min_x = self.x;
        let min_y = self.y;
        let width = self.width;
        let height = self.height;

        (0..height).flat_map(move |dy| (0..width).map(move |dx| (min_x + dx, min_y + dy)))
    }

    /// Triggers physical impact wobble / bobbing oscillation on vehicle bump.
    pub fn trigger_wobble(&mut self, intensity: f32) {
        self.wobble_timer = 0.001;
        self.wobble_intensity = intensity.clamp(0.2, 1.0);
    }

    /// Advances the wobble / bobbing animation timer. Returns false when animation completes.
    pub fn update(&mut self, dt: f32) -> bool {
        if self.wobble_timer <= 0.0 {
            return false;
        }
        self.wobble_timer += dt;
        if self.wobble_timer >= 0.45 {
            self.wobble_timer = 0.0;
            self.wobble_intensity = 0.0;
            false
        } else {
            true
        }
    }

    /// Computes current visual pixel displacement `(offset_x, offset_y)` in grid units.
    pub fn wobble_offset(&self) -> (f32, f32) {
        if self.wobble_timer <= 0.0 {
            return (0.0, 0.0);
        }
        let t = self.wobble_timer / 0.45;
        let envelope = (1.0 - t).powi(2);
        let wave = (t * std::f32::consts::PI * 3.0).sin();
        let amp = wave * envelope * 0.12 * self.wobble_intensity;
        (amp, amp * 0.5)
    }

    /// Returns the pixel rectangle `(px, py, pw, ph)` for rendering this obstacle.
    pub fn pixel_bounds(
        &self,
        origin_x: f32,
        origin_y: f32,
        cell_size: f32,
    ) -> (f32, f32, f32, f32) {
        let (wx, wy) = self.wobble_offset();
        let px = origin_x + (self.x as f32 + wx) * cell_size;
        let py = origin_y + (self.y as f32 + wy) * cell_size;
        let pw = self.width as f32 * cell_size;
        let ph = self.height as f32 * cell_size;
        (px, py, pw, ph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obstacle_contains_cell_and_iterator() {
        let obs = Obstacle::new(2, 3, 2, 2, Some(ObstacleKind::Rock));
        assert!(obs.contains_cell(2, 3));
        assert!(obs.contains_cell(3, 3));
        assert!(obs.contains_cell(2, 4));
        assert!(obs.contains_cell(3, 4));
        assert!(!obs.contains_cell(1, 3));
        assert!(!obs.contains_cell(4, 4));

        let cells: Vec<(i32, i32)> = obs.cells().collect();
        assert_eq!(cells, vec![(2, 3), (3, 3), (2, 4), (3, 4)]);
    }

    #[test]
    fn test_obstacle_wobble_lifecycle() {
        let mut obs = Obstacle::buoy_1x1(1, 1);
        assert_eq!(obs.wobble_offset(), (0.0, 0.0));

        obs.trigger_wobble(1.0);
        assert!(obs.wobble_timer > 0.0);
        let (ox, _) = obs.wobble_offset();
        assert!(ox != 0.0);

        assert!(obs.update(0.1));
        let (ox2, _) = obs.wobble_offset();
        assert!(ox2.abs() > 0.0);

        assert!(!obs.update(0.5));
        assert_eq!(obs.wobble_offset(), (0.0, 0.0));
    }

    #[test]
    fn test_obstacle_theme_conversion() {
        // Marine sprite conversions
        assert_eq!(ObstacleKind::Pillar.marine_sprite_name(), "marine_buoy");
        assert_eq!(ObstacleKind::Barrier.marine_sprite_name(), "marine_rock");
        assert_eq!(ObstacleKind::Rock.marine_sprite_name(), "marine_rock");
        assert_eq!(ObstacleKind::Buoy.marine_sprite_name(), "marine_buoy");

        // City sprite conversions
        assert_eq!(ObstacleKind::Buoy.city_sprite_name(), "city_pillar");
        assert_eq!(ObstacleKind::Pillar.city_sprite_name(), "city_pillar");
        assert_eq!(ObstacleKind::Barrier.city_sprite_name(), "city_barrier");
        assert_eq!(ObstacleKind::Rock.city_sprite_name(), "city_rock");

        // Unified sprite_for_theme method
        assert_eq!(
            ObstacleKind::Pillar.sprite_for_theme(Theme::Marine),
            "marine_buoy"
        );
        assert_eq!(
            ObstacleKind::Pillar.sprite_for_theme(Theme::City),
            "city_pillar"
        );

        // Sprite name on Obstacle struct
        let pillar_obs = Obstacle::new(0, 0, 1, 1, Some(ObstacleKind::Pillar));
        assert_eq!(pillar_obs.sprite_name(Theme::Marine), "marine_buoy");
        assert_eq!(pillar_obs.sprite_name(Theme::City), "city_pillar");

        let barrier_obs = Obstacle::new(0, 0, 1, 1, Some(ObstacleKind::Barrier));
        assert_eq!(barrier_obs.sprite_name(Theme::Marine), "marine_rock");
        assert_eq!(barrier_obs.sprite_name(Theme::City), "city_barrier");

        let unspecified_obs = Obstacle::new(0, 0, 1, 1, None);
        assert_eq!(unspecified_obs.sprite_name(Theme::Marine), "marine_buoy");
        assert_eq!(unspecified_obs.sprite_name(Theme::City), "city_rock");
        assert_eq!(
            unspecified_obs.sprite_name(Theme::Railroad),
            "railroad_buffer_stop"
        );

        // Railroad sprite conversions
        assert_eq!(
            ObstacleKind::Pillar.railroad_sprite_name(),
            "railroad_semaphore"
        );
        assert_eq!(
            ObstacleKind::Barrier.railroad_sprite_name(),
            "railroad_buffer_stop"
        );
        assert_eq!(
            ObstacleKind::Rock.railroad_sprite_name(),
            "railroad_coal_pile"
        );
        assert_eq!(
            ObstacleKind::Pillar.sprite_for_theme(Theme::Railroad),
            "railroad_semaphore"
        );
    }
}
