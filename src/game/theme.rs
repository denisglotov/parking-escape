use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    City,
    Marine,
    Railroad,
}

impl Theme {
    /// Determines the theme for a given level ID, rotating City -> Marine -> Railroad.
    pub const fn for_level(level_id: u32) -> Self {
        match level_id % 3 {
            1 => Self::City,
            2 => Self::Marine,
            _ => Self::Railroad,
        }
    }

    /// Inertial touch/drag tracking responsiveness factor.
    /// Marine features smooth momentum lag, Railroad features weighty steel inertia.
    pub const fn drag_responsiveness(&self, mass: f32) -> f32 {
        match self {
            Self::City => 28.0 / mass,
            Self::Marine => 13.0 / mass,
            Self::Railroad => 22.0 / mass,
        }
    }

    /// Velocity threshold to trigger an inertial coasting swipe launch upon finger release.
    pub const fn swipe_launch_threshold(&self) -> f32 {
        match self {
            Self::City => 1.2,
            Self::Marine => 0.60,
            Self::Railroad => 0.90,
        }
    }

    /// Coasting kinetic / hydrodynamic / rolling friction coefficient.
    /// Railroads have low rolling resistance allowing train cars to glide smoothly on tracks.
    pub const fn friction(&self, mass: f32) -> f32 {
        match self {
            Self::City => 13.0 / mass,
            Self::Marine => 5.2 / mass,
            Self::Railroad => 8.0 / mass,
        }
    }

    /// Grid cell snap speed when stopping from a coast.
    pub const fn snap_speed(&self) -> f32 {
        match self {
            Self::City => 18.0,
            Self::Marine => 11.0,
            Self::Railroad => 16.0,
        }
    }

    /// Environment background texture key.
    pub const fn background_texture_key(&self) -> &'static str {
        match self {
            Self::City => "city_background",
            Self::Marine => "marine_background",
            Self::Railroad => "railroad_background",
        }
    }

    /// Environment ground tile texture key.
    pub const fn ground_texture_key(&self) -> &'static str {
        match self {
            Self::City => "city_ground",
            Self::Marine => "marine_ground",
            Self::Railroad => "railroad_ground",
        }
    }

    /// Exit gate texture key.
    pub const fn exit_gate_texture_key(&self) -> &'static str {
        match self {
            Self::City => "city_exit_gate",
            Self::Marine => "marine_exit_gate",
            Self::Railroad => "railroad_exit_gate",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_rotation_for_levels() {
        assert_eq!(Theme::for_level(1), Theme::City);
        assert_eq!(Theme::for_level(2), Theme::Marine);
        assert_eq!(Theme::for_level(3), Theme::Railroad);
        assert_eq!(Theme::for_level(4), Theme::City);
        assert_eq!(Theme::for_level(5), Theme::Marine);
        assert_eq!(Theme::for_level(6), Theme::Railroad);
    }

    #[test]
    fn test_theme_physics_parameters() {
        let mass = 1.0;
        assert_eq!(Theme::Railroad.friction(mass), 8.0);
        assert_eq!(Theme::Railroad.drag_responsiveness(mass), 22.0);
        assert_eq!(Theme::Railroad.swipe_launch_threshold(), 0.90);
        assert_eq!(Theme::Railroad.snap_speed(), 16.0);
    }

    #[test]
    fn test_theme_texture_keys() {
        assert_eq!(
            Theme::Railroad.background_texture_key(),
            "railroad_background"
        );
        assert_eq!(Theme::Railroad.ground_texture_key(), "railroad_ground");
        assert_eq!(
            Theme::Railroad.exit_gate_texture_key(),
            "railroad_exit_gate"
        );
    }
}
