pub mod hud;
pub mod icons;
pub mod level_select;
pub mod menu;
pub mod renderer;
pub mod win_modal;

use macroquad::prelude::*;
use std::collections::HashMap;

pub struct UITheme {
    pub bg_dark: Color,
    pub surface: Color,
    pub surface_hover: Color,
    pub card_bg: Color,
    pub accent_blue: Color,
    pub accent_gold: Color,
    pub accent_green: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
}

pub const THEME: UITheme = UITheme {
    bg_dark: Color::new(0.07, 0.08, 0.10, 1.0),
    surface: Color::new(0.12, 0.14, 0.18, 1.0),
    surface_hover: Color::new(0.18, 0.22, 0.28, 1.0),
    card_bg: Color::new(0.15, 0.17, 0.22, 1.0),
    accent_blue: Color::new(0.23, 0.51, 0.96, 1.0),
    accent_gold: Color::new(0.96, 0.62, 0.04, 1.0),
    accent_green: Color::new(0.06, 0.73, 0.51, 1.0),
    text_primary: Color::new(0.96, 0.97, 0.99, 1.0),
    text_secondary: Color::new(0.80, 0.84, 0.90, 1.0),
    text_muted: Color::new(0.45, 0.50, 0.58, 1.0),
};

#[derive(Debug, Clone, Copy)]
pub struct ButtonStyle {
    pub bg_color: Color,
    pub text_color: Color,
    pub font_size: f32,
    pub border_width: f32,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            bg_color: THEME.card_bg,
            text_color: THEME.text_primary,
            font_size: 18.0,
            border_width: 1.5,
        }
    }
}

pub fn draw_ui_button(
    bounds: Rect,
    label: &str,
    style: ButtonStyle,
    mouse_pos: (f32, f32),
    is_clicked: bool,
) -> bool {
    let hovered = bounds.contains(vec2(mouse_pos.0, mouse_pos.1));
    let fill = if hovered {
        Color::new(
            (style.bg_color.r * 1.15).min(1.0),
            (style.bg_color.g * 1.15).min(1.0),
            (style.bg_color.b * 1.15).min(1.0),
            1.0,
        )
    } else {
        style.bg_color
    };

    draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, fill);
    draw_rectangle_lines(
        bounds.x,
        bounds.y,
        bounds.w,
        bounds.h,
        style.border_width,
        Color::new(1.0, 1.0, 1.0, if hovered { 0.5 } else { 0.2 }),
    );

    let dim = measure_text(label, None, style.font_size as u16, 1.0);
    draw_text(
        label,
        bounds.x + (bounds.w - dim.width) / 2.0,
        bounds.y + (bounds.h + dim.height) / 2.0 - 2.0,
        style.font_size,
        if hovered {
            THEME.accent_gold
        } else {
            style.text_color
        },
    );

    hovered && is_clicked
}

pub struct TextureStore {
    pub textures: HashMap<String, Texture2D>,
}

impl TextureStore {
    pub async fn load_all() -> Self {
        let mut textures = HashMap::new();

        // Environment textures
        textures.insert(
            "asphalt".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/environment/asphalt.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            "stall_marker".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/environment/stall_marker.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            "exit_gate".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/environment/exit_gate.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            "curb_h".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/environment/curb_horizontal.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            "curb_v".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/environment/curb_vertical.png"),
                Some(ImageFormat::Png),
            ),
        );
        textures.insert(
            "curb_corner".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("../../assets/environment/curb_corner.png"),
                Some(ImageFormat::Png),
            ),
        );

        // Vehicle textures (both H and V)
        macro_rules! load_veh {
            ($key:expr, $file:expr) => {
                textures.insert(
                    $key.to_string(),
                    Texture2D::from_file_with_format(
                        include_bytes!(concat!("../../assets/vehicles/", $file)),
                        Some(ImageFormat::Png),
                    ),
                );
            };
        }

        load_veh!("player_red_h", "player_red_h.png");
        load_veh!("player_red_v", "player_red_v.png");

        load_veh!("car_sedan_blue_h", "car_sedan_blue_h.png");
        load_veh!("car_sedan_blue_v", "car_sedan_blue_v.png");

        load_veh!("car_taxi_yellow_h", "car_taxi_yellow_h.png");
        load_veh!("car_taxi_yellow_v", "car_taxi_yellow_v.png");

        load_veh!("car_hatchback_green_h", "car_hatchback_green_h.png");
        load_veh!("car_hatchback_green_v", "car_hatchback_green_v.png");

        load_veh!("car_police_h", "car_police_h.png");
        load_veh!("car_police_v", "car_police_v.png");

        load_veh!("truck_delivery_h", "truck_delivery_h.png");
        load_veh!("truck_delivery_v", "truck_delivery_v.png");

        load_veh!("limo_white_h", "limo_white_h.png");
        load_veh!("limo_white_v", "limo_white_v.png");

        load_veh!("ambulance_h", "ambulance_h.png");
        load_veh!("ambulance_v", "ambulance_v.png");

        load_veh!("semi_truck_h", "semi_truck_h.png");
        load_veh!("semi_truck_v", "semi_truck_v.png");

        load_veh!("bus_transit_h", "bus_transit_h.png");
        load_veh!("bus_transit_v", "bus_transit_v.png");

        Self { textures }
    }

    pub fn get(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }
}

/// Computes responsive board layout positioning and cell sizing.
#[derive(Debug, Clone, Copy)]
pub struct BoardLayout {
    pub origin_x: f32,
    pub origin_y: f32,
    pub cell_size: f32,
    pub total_width: f32,
    pub total_height: f32,
}

impl BoardLayout {
    pub fn calculate(screen_w: f32, screen_h: f32, grid_w: i32, grid_h: i32) -> Self {
        let top_hud_height = 90.0;
        let bottom_margin = 40.0;
        let available_w = screen_w * 0.92;
        let available_h = (screen_h - top_hud_height - bottom_margin).max(100.0);

        let cell_size_by_w = available_w / grid_w as f32;
        let cell_size_by_h = available_h / grid_h as f32;
        let cell_size = cell_size_by_w.min(cell_size_by_h).clamp(32.0, 110.0);

        let total_width = cell_size * grid_w as f32;
        let total_height = cell_size * grid_h as f32;

        let origin_x = (screen_w - total_width) / 2.0;
        let origin_y = top_hud_height + (available_h - total_height) / 2.0;

        Self {
            origin_x,
            origin_y,
            cell_size,
            total_width,
            total_height,
        }
    }
}
