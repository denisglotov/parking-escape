pub mod background;
pub mod hud;
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameTheme {
    #[default]
    City,
    Marine,
}

impl GameTheme {
    pub const fn for_level(level_id: u32) -> Self {
        if level_id.is_multiple_of(2) {
            Self::Marine
        } else {
            Self::City
        }
    }

    pub const fn is_marine(&self) -> bool {
        matches!(self, Self::Marine)
    }
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
            font_size: 22.0,
            border_width: 1.5,
        }
    }
}

pub fn draw_ui_button(
    textures: &TextureStore,
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

    let text_col = if hovered {
        THEME.accent_gold
    } else {
        style.text_color
    };

    textures.draw_text_centered(
        label,
        bounds.x + bounds.w / 2.0,
        bounds.y + bounds.h / 2.0,
        style.font_size,
        text_col,
    );

    hovered && is_clicked
}

#[derive(Debug, Clone, Copy)]
pub struct ShadowTextStyle {
    pub font_size: f32,
    pub color: Color,
    pub shadow_color: Color,
    pub offset: f32,
}

impl ShadowTextStyle {
    pub fn new(font_size: f32, color: Color, shadow_color: Color, offset: f32) -> Self {
        Self {
            font_size,
            color,
            shadow_color,
            offset,
        }
    }
}

pub struct TextureStore {
    pub textures: HashMap<String, Texture2D>,
    pub font: Font,
}

impl TextureStore {
    pub async fn load_all() -> Self {
        let mut textures = HashMap::new();

        let font = load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/game_font.ttf"))
            .expect("Failed to load TTF game font");

        macro_rules! load_tex {
            ($key:expr, $path:expr) => {
                textures.insert(
                    $key.to_string(),
                    Texture2D::from_file_with_format(include_bytes!($path), Some(ImageFormat::Png)),
                );
            };
        }

        // Environment textures
        load_tex!(
            "park_background",
            "../../assets/environment/park_background.png"
        );
        load_tex!("asphalt", "../../assets/environment/asphalt.png");
        load_tex!("stall_marker", "../../assets/environment/stall_marker.png");
        load_tex!("exit_gate", "../../assets/environment/exit_gate.png");

        // Marine Environment textures
        load_tex!(
            "marine_background",
            "../../assets/environment/marine_background.png"
        );
        load_tex!("marine_water", "../../assets/environment/marine_water.png");
        load_tex!(
            "marine_exit_gate",
            "../../assets/environment/marine_exit_gate.png"
        );

        // UI icon assets
        load_tex!("badge_parking", "../../assets/ui/badge_parking.png");
        load_tex!("star_gold", "../../assets/ui/star_gold.png");
        load_tex!("star_empty", "../../assets/ui/star_empty.png");
        load_tex!("icon_undo", "../../assets/ui/icon_undo.png");
        load_tex!("icon_reset", "../../assets/ui/icon_reset.png");
        load_tex!("icon_back", "../../assets/ui/icon_back.png");
        load_tex!("icon_sound_on", "../../assets/ui/icon_sound_on.png");
        load_tex!("icon_sound_off", "../../assets/ui/icon_sound_off.png");

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

        // Marine ship textures (both H and V)
        load_veh!("ship_player_red_h", "ship_player_red_h.png");
        load_veh!("ship_player_red_v", "ship_player_red_v.png");

        load_veh!("ship_sail_blue_h", "ship_sail_blue_h.png");
        load_veh!("ship_sail_blue_v", "ship_sail_blue_v.png");

        load_veh!("ship_taxi_yellow_h", "ship_taxi_yellow_h.png");
        load_veh!("ship_taxi_yellow_v", "ship_taxi_yellow_v.png");

        load_veh!("ship_tug_green_h", "ship_tug_green_h.png");
        load_veh!("ship_tug_green_v", "ship_tug_green_v.png");

        load_veh!("ship_patrol_h", "ship_patrol_h.png");
        load_veh!("ship_patrol_v", "ship_patrol_v.png");

        load_veh!("ship_cargo_h", "ship_cargo_h.png");
        load_veh!("ship_cargo_v", "ship_cargo_v.png");

        load_veh!("ship_yacht_white_h", "ship_yacht_white_h.png");
        load_veh!("ship_yacht_white_v", "ship_yacht_white_v.png");

        load_veh!("ship_sar_rescue_h", "ship_sar_rescue_h.png");
        load_veh!("ship_sar_rescue_v", "ship_sar_rescue_v.png");

        load_veh!("ship_container_h", "ship_container_h.png");
        load_veh!("ship_container_v", "ship_container_v.png");

        load_veh!("ship_ferry_h", "ship_ferry_h.png");
        load_veh!("ship_ferry_v", "ship_ferry_v.png");

        Self { textures, font }
    }

    pub fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(&self.font),
                font_size: font_size.round() as u16,
                font_scale: 1.0,
                color,
                ..Default::default()
            },
        );
    }

    pub fn measure_text(&self, text: &str, font_size: f32) -> TextDimensions {
        measure_text(text, Some(&self.font), font_size.round() as u16, 1.0)
    }

    pub fn draw_text_centered(&self, text: &str, cx: f32, cy: f32, font_size: f32, color: Color) {
        let dim = self.measure_text(text, font_size);
        self.draw_text(
            text,
            cx - dim.width / 2.0,
            cy - dim.height / 2.0 + dim.offset_y,
            font_size,
            color,
        );
    }

    pub fn draw_text_with_shadow(&self, text: &str, cx: f32, cy: f32, style: ShadowTextStyle) {
        let dim = self.measure_text(text, style.font_size);
        let x = cx - dim.width / 2.0;
        let y = cy - dim.height / 2.0 + dim.offset_y;
        self.draw_text(
            text,
            x + style.offset,
            y + style.offset,
            style.font_size,
            style.shadow_color,
        );
        self.draw_text(text, x, y, style.font_size, style.color);
    }

    pub fn get(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    pub fn draw_icon_button(
        &self,
        tex_key: &str,
        rect: Rect,
        enabled: bool,
        is_mouse_down: bool,
        mouse_pos: (f32, f32),
    ) -> bool {
        let hovered = enabled && rect.contains(vec2(mouse_pos.0, mouse_pos.1));

        let bg_color = if !enabled {
            Color::new(0.12, 0.13, 0.16, 0.4)
        } else if hovered {
            THEME.surface_hover
        } else {
            THEME.card_bg
        };

        draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);
        let border_width = (rect.w * 0.04).clamp(1.5, 4.0);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            border_width,
            Color::new(0.3, 0.35, 0.45, if enabled { 0.6 } else { 0.2 }),
        );

        if let Some(tex) = self.get(tex_key) {
            let icon_sz = rect.w.min(rect.h) * 0.72;
            let icon_pad_x = (rect.w - icon_sz) / 2.0;
            let icon_pad_y = (rect.h - icon_sz) / 2.0;
            let tint = if !enabled {
                Color::new(0.5, 0.5, 0.5, 0.4)
            } else if hovered {
                WHITE
            } else {
                Color::new(0.9, 0.9, 0.9, 0.9)
            };

            draw_texture_ex(
                tex,
                rect.x + icon_pad_x,
                rect.y + icon_pad_y,
                tint,
                DrawTextureParams {
                    dest_size: Some(vec2(icon_sz, icon_sz)),
                    ..Default::default()
                },
            );
        }

        hovered && is_mouse_down
    }

    pub fn draw_star_row(&self, cx: f32, cy: f32, earned: u8, total: u8, size: f32, spacing: f32) {
        let total_w = total as f32 * size + (total - 1) as f32 * spacing;
        let start_x = cx - total_w / 2.0;

        for i in 0..total {
            let sx = start_x + i as f32 * (size + spacing);
            let tex_name = if i < earned {
                "star_gold"
            } else {
                "star_empty"
            };
            if let Some(tex) = self.get(tex_name) {
                draw_texture_ex(
                    tex,
                    sx,
                    cy - size / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(size, size)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

/// Scaling metrics and helper methods for responsive UI across all screen densities.
#[derive(Debug, Clone, Copy)]
pub struct UiMetrics {
    pub scale: f32,
    pub hud_height: f32,
}

impl UiMetrics {
    pub const BASE_WIDTH: f32 = 540.0;
    pub const BASE_HEIGHT: f32 = 800.0;

    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        let scale_w = screen_w / Self::BASE_WIDTH;
        let scale_h = screen_h / Self::BASE_HEIGHT;
        // In portrait mode, scale based on width, but guard against extreme landscape/stretched aspect ratios
        let scale = scale_w.min(scale_h * 1.25).clamp(0.75, 4.0);
        let hud_height = (84.0 * scale).round();

        Self { scale, hud_height }
    }

    /// Scale a base pixel value proportionally.
    pub fn s(&self, val: f32) -> f32 {
        (val * self.scale).round()
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
    pub hud_height: f32,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl BoardLayout {
    pub fn calculate(screen_w: f32, screen_h: f32, grid_w: i32, grid_h: i32) -> Self {
        let metrics = UiMetrics::new(screen_w, screen_h);
        let top_hud_height = metrics.hud_height + metrics.s(16.0);
        let bottom_margin = metrics.s(32.0);
        let available_w = screen_w * 0.92;
        let available_h = (screen_h - top_hud_height - bottom_margin).max(100.0);

        let cell_size_by_w = available_w / grid_w as f32;
        let cell_size_by_h = available_h / grid_h as f32;
        let min_cell_size = metrics.s(28.0);
        let max_cell_size = metrics.s(200.0);
        let cell_size = cell_size_by_w
            .min(cell_size_by_h)
            .clamp(min_cell_size, max_cell_size);

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
            hud_height: metrics.hud_height,
            screen_width: screen_w,
            screen_height: screen_h,
        }
    }
}
