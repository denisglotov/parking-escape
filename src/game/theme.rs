use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    City,
    Marine,
}

impl Theme {
    /// Determines the theme for a given level ID.
    pub const fn for_level(level_id: u32) -> Self {
        if level_id.is_multiple_of(2) {
            Self::Marine
        } else {
            Self::City
        }
    }

    /// Inertial touch/drag tracking responsiveness factor.
    /// Marine/water physics features lower responsiveness with smoother momentum lag.
    pub const fn drag_responsiveness(&self, mass: f32) -> f32 {
        match self {
            Self::City => 28.0 / mass,
            Self::Marine => 13.0 / mass,
        }
    }

    /// Velocity threshold to trigger an inertial coasting swipe launch upon finger release.
    pub const fn swipe_launch_threshold(&self) -> f32 {
        match self {
            Self::City => 1.2,
            Self::Marine => 0.60,
        }
    }

    /// Coasting kinetic / hydrodynamic friction coefficient.
    /// Marine/water theme has lower friction allowing vessels to glide farther.
    pub const fn friction(&self, mass: f32) -> f32 {
        match self {
            Self::City => 13.0 / mass,
            Self::Marine => 5.2 / mass,
        }
    }

    /// Grid cell snap speed when stopping from a coast.
    pub const fn snap_speed(&self) -> f32 {
        match self {
            Self::City => 18.0,
            Self::Marine => 11.0,
        }
    }

    /// Environment background texture key.
    pub const fn background_texture_key(&self) -> &'static str {
        match self {
            Self::City => "city_background",
            Self::Marine => "marine_background",
        }
    }

    /// Environment ground tile texture key.
    pub const fn ground_texture_key(&self) -> &'static str {
        match self {
            Self::City => "city_ground",
            Self::Marine => "marine_ground",
        }
    }

    /// Exit gate texture key.
    pub const fn exit_gate_texture_key(&self) -> &'static str {
        match self {
            Self::City => "city_exit_gate",
            Self::Marine => "marine_exit_gate",
        }
    }
}
