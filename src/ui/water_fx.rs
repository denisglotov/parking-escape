use macroquad::prelude::*;

/// Dynamic water ripple ring spawned on touch, movement, or impact.
#[derive(Debug, Clone, Copy)]
pub struct WaterRipple {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub max_radius: f32,
    pub life: f32,
    pub max_life: f32,
    pub color: Color,
}

impl WaterRipple {
    pub fn new(x: f32, y: f32, max_radius: f32, max_life: f32, color: Color) -> Self {
        Self {
            x,
            y,
            radius: 2.0,
            max_radius,
            life: 0.0,
            max_life,
            color,
        }
    }

    /// Updates ripple expansion and returns `false` when expired.
    pub fn update(&mut self, dt: f32) -> bool {
        self.life += dt;
        if self.life >= self.max_life {
            return false;
        }

        let progress = self.life / self.max_life;
        // Ease-out expansion
        self.radius = self.max_radius * (1.0 - (1.0 - progress).powi(2));
        true
    }

    /// Draws the concentric ripple rings with fading alpha.
    pub fn render(&self) {
        let progress = (self.life / self.max_life).clamp(0.0, 1.0);
        let alpha = (1.0 - progress) * self.color.a;

        if alpha <= 0.005 {
            return;
        }

        let ring_col = Color::new(self.color.r, self.color.g, self.color.b, alpha * 0.75);
        let core_col = Color::new(self.color.r, self.color.g, self.color.b, alpha * 0.40);

        draw_circle_lines(self.x, self.y, self.radius, 2.0, ring_col);
        if self.radius > 6.0 {
            draw_circle_lines(self.x, self.y, self.radius * 0.55, 1.2, core_col);
        }
    }
}

/// Lightweight manager for tracking active water ripple effects.
#[derive(Debug, Default, Clone)]
pub struct WaterRippleManager {
    ripples: Vec<WaterRipple>,
}

impl WaterRippleManager {
    pub fn new() -> Self {
        Self {
            ripples: Vec::with_capacity(32),
        }
    }

    /// Spawns a touch/interaction ripple at `(x, y)`.
    pub fn spawn_touch_ripple(&mut self, x: f32, y: f32, cs: f32) {
        if self.ripples.len() >= 32 {
            self.ripples.remove(0);
        }
        let color = Color::new(0.65, 0.90, 1.0, 0.85);
        self.ripples
            .push(WaterRipple::new(x, y, cs * 0.65, 0.65, color));
    }

    /// Spawns an impact ripple burst at `(x, y)`.
    pub fn spawn_impact_ripple(&mut self, x: f32, y: f32, cs: f32, intensity: f32) {
        if self.ripples.len() >= 32 {
            self.ripples.remove(0);
        }
        let color = Color::new(0.85, 0.96, 1.0, (0.7 * intensity).min(0.95));
        self.ripples.push(WaterRipple::new(
            x,
            y,
            cs * (0.5 + intensity * 0.5),
            0.8,
            color,
        ));
    }

    /// Updates active ripples and removes expired ones.
    pub fn update(&mut self, dt: f32) {
        self.ripples.retain_mut(|r| r.update(dt));
    }

    /// Draws all active ripples.
    pub fn render(&self) {
        for ripple in &self.ripples {
            ripple.render();
        }
    }

    /// Clears all active ripples (e.g. on level change).
    pub fn clear(&mut self) {
        self.ripples.clear();
    }
}

/// Computes idle buoyancy heave (drift sway) and roll (subtle rocking) for a floating ship at a given timestamp.
pub fn compute_vessel_buoyancy_at_time(
    time: f32,
    idx: usize,
    px: f32,
    py: f32,
    cs: f32,
    is_dragged: bool,
) -> (f32, f32, f32) {
    if is_dragged {
        return (0.0, 0.0, 0.0);
    }

    let phase = idx as f32 * 1.45 + (px * 0.03 + py * 0.03);

    // Lateral and vertical heave displacement (±0.8 to 1.5 pixels)
    let heave_x = (time * 1.6 + phase * 0.7).sin() * (cs * 0.015);
    let heave_y = (time * 2.1 + phase).sin() * (cs * 0.022);

    // Subtle rotational rocking roll (±0.8 to 1.2 degrees in radians)
    let roll = (time * 1.7 + phase + 0.6).sin() * 0.018;

    (heave_x, heave_y, roll)
}

/// Computes idle buoyancy heave and roll using current frame time.
pub fn compute_vessel_buoyancy(
    idx: usize,
    px: f32,
    py: f32,
    cs: f32,
    is_dragged: bool,
) -> (f32, f32, f32) {
    compute_vessel_buoyancy_at_time(get_time() as f32, idx, px, py, cs, is_dragged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_ripple_lifecycle() {
        let mut ripple = WaterRipple::new(100.0, 100.0, 50.0, 1.0, WHITE);
        assert_eq!(ripple.x, 100.0);
        assert_eq!(ripple.y, 100.0);
        assert!(ripple.radius < 50.0);

        // Advance halfway
        assert!(ripple.update(0.5));
        assert!(ripple.radius > 20.0);
        assert!(ripple.radius < 50.0);

        // Advance to expiry
        assert!(!ripple.update(0.6));
    }

    #[test]
    fn test_water_ripple_manager() {
        let mut manager = WaterRippleManager::new();
        assert!(manager.ripples.is_empty());

        manager.spawn_touch_ripple(50.0, 50.0, 60.0);
        manager.spawn_impact_ripple(150.0, 150.0, 60.0, 1.2);
        assert_eq!(manager.ripples.len(), 2);

        // Advance past lifetime
        manager.update(1.5);
        assert!(manager.ripples.is_empty());

        // Test clear
        manager.spawn_touch_ripple(20.0, 20.0, 40.0);
        assert_eq!(manager.ripples.len(), 1);
        manager.clear();
        assert!(manager.ripples.is_empty());
    }

    #[test]
    fn test_vessel_buoyancy_calculation() {
        let (drag_hx, drag_hy, drag_roll) =
            compute_vessel_buoyancy_at_time(1.0, 0, 50.0, 50.0, 60.0, true);
        assert_eq!(drag_hx, 0.0);
        assert_eq!(drag_hy, 0.0);
        assert_eq!(drag_roll, 0.0);

        let (idle_hx, idle_hy, idle_roll) =
            compute_vessel_buoyancy_at_time(1.0, 0, 50.0, 50.0, 60.0, false);
        // Buoyancy heave and roll should be subtle and bounded
        assert!(idle_hx.abs() <= 60.0 * 0.02);
        assert!(idle_hy.abs() <= 60.0 * 0.03);
        assert!(idle_roll.abs() <= 0.05);
    }
}
