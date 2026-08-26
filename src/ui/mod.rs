pub mod background;
pub mod hud;
pub mod level_select;
pub mod menu;
pub mod renderer;
pub mod water_fx;
pub mod win_modal;

use macroquad::prelude::*;
use std::collections::HashMap;

pub use water_fx::WaterRippleManager;

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

use std::cell::RefCell;

pub struct TextureStore {
    textures: RefCell<HashMap<&'static str, Texture2D>>,
    pub font: Font,
}

impl TextureStore {
    pub const BASE_FONT_SIZE: u16 = 64;

    pub async fn load_all() -> Self {
        let font = load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/game_font.ttf"))
            .expect("Failed to load TTF game font");

        Self {
            textures: RefCell::new(HashMap::new()),
            font,
        }
    }

    pub fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        let scale = font_size / Self::BASE_FONT_SIZE as f32;
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(&self.font),
                font_size: Self::BASE_FONT_SIZE,
                font_scale: scale,
                color,
                ..Default::default()
            },
        );
    }

    pub fn measure_text(&self, text: &str, font_size: f32) -> TextDimensions {
        let scale = font_size / Self::BASE_FONT_SIZE as f32;
        measure_text(text, Some(&self.font), Self::BASE_FONT_SIZE, scale)
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

    pub fn get(&self, name: &str) -> Option<Texture2D> {
        let mut map = self.textures.borrow_mut();
        if let Some(tex) = map.get(name) {
            return Some(tex.clone());
        }

        let (key, bytes) = match_raw_texture(name)?;
        let tex = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
        map.insert(key, tex.clone());
        Some(tex)
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
                &tex,
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
                    &tex,
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

fn match_raw_texture(name: &str) -> Option<(&'static str, &'static [u8])> {
    let (key, bytes) = match name {
        // UI icon assets
        "badge_parking" => (
            "badge_parking",
            include_bytes!("../../assets/ui/badge_parking.png").as_slice(),
        ),
        "star_gold" => (
            "star_gold",
            include_bytes!("../../assets/ui/star_gold.png").as_slice(),
        ),
        "star_empty" => (
            "star_empty",
            include_bytes!("../../assets/ui/star_empty.png").as_slice(),
        ),
        "icon_undo" => (
            "icon_undo",
            include_bytes!("../../assets/ui/icon_undo.png").as_slice(),
        ),
        "icon_reset" => (
            "icon_reset",
            include_bytes!("../../assets/ui/icon_reset.png").as_slice(),
        ),
        "icon_back" => (
            "icon_back",
            include_bytes!("../../assets/ui/icon_back.png").as_slice(),
        ),
        "icon_sound_on" => (
            "icon_sound_on",
            include_bytes!("../../assets/ui/icon_sound_on.png").as_slice(),
        ),
        "icon_sound_off" => (
            "icon_sound_off",
            include_bytes!("../../assets/ui/icon_sound_off.png").as_slice(),
        ),

        // City Environment & Obstacles
        "city_background" => (
            "city_background",
            include_bytes!("../../assets/themes/city/environment/background.png").as_slice(),
        ),
        "city_ground" => (
            "city_ground",
            include_bytes!("../../assets/themes/city/environment/ground.png").as_slice(),
        ),
        "stall_marker" => (
            "stall_marker",
            include_bytes!("../../assets/themes/city/environment/stall_marker.png").as_slice(),
        ),
        "city_exit_gate" => (
            "city_exit_gate",
            include_bytes!("../../assets/themes/city/environment/exit_gate.png").as_slice(),
        ),
        "city_rock" => (
            "city_rock",
            include_bytes!("../../assets/themes/city/obstacles/rock.png").as_slice(),
        ),
        "city_pillar" => (
            "city_pillar",
            include_bytes!("../../assets/themes/city/obstacles/pillar.png").as_slice(),
        ),
        "city_barrier" => (
            "city_barrier",
            include_bytes!("../../assets/themes/city/obstacles/barrier.png").as_slice(),
        ),

        // City Vehicles
        "player_red_h" => (
            "player_red_h",
            include_bytes!("../../assets/themes/city/vehicles/player_red_h.png").as_slice(),
        ),
        "player_red_v" => (
            "player_red_v",
            include_bytes!("../../assets/themes/city/vehicles/player_red_v.png").as_slice(),
        ),
        "car_sedan_blue_h" => (
            "car_sedan_blue_h",
            include_bytes!("../../assets/themes/city/vehicles/car_sedan_blue_h.png").as_slice(),
        ),
        "car_sedan_blue_v" => (
            "car_sedan_blue_v",
            include_bytes!("../../assets/themes/city/vehicles/car_sedan_blue_v.png").as_slice(),
        ),
        "car_taxi_yellow_h" => (
            "car_taxi_yellow_h",
            include_bytes!("../../assets/themes/city/vehicles/car_taxi_yellow_h.png").as_slice(),
        ),
        "car_taxi_yellow_v" => (
            "car_taxi_yellow_v",
            include_bytes!("../../assets/themes/city/vehicles/car_taxi_yellow_v.png").as_slice(),
        ),
        "car_hatchback_green_h" => (
            "car_hatchback_green_h",
            include_bytes!("../../assets/themes/city/vehicles/car_hatchback_green_h.png")
                .as_slice(),
        ),
        "car_hatchback_green_v" => (
            "car_hatchback_green_v",
            include_bytes!("../../assets/themes/city/vehicles/car_hatchback_green_v.png")
                .as_slice(),
        ),
        "car_police_h" => (
            "car_police_h",
            include_bytes!("../../assets/themes/city/vehicles/car_police_h.png").as_slice(),
        ),
        "car_police_v" => (
            "car_police_v",
            include_bytes!("../../assets/themes/city/vehicles/car_police_v.png").as_slice(),
        ),
        "truck_delivery_h" => (
            "truck_delivery_h",
            include_bytes!("../../assets/themes/city/vehicles/truck_delivery_h.png").as_slice(),
        ),
        "truck_delivery_v" => (
            "truck_delivery_v",
            include_bytes!("../../assets/themes/city/vehicles/truck_delivery_v.png").as_slice(),
        ),
        "limo_white_h" => (
            "limo_white_h",
            include_bytes!("../../assets/themes/city/vehicles/limo_white_h.png").as_slice(),
        ),
        "limo_white_v" => (
            "limo_white_v",
            include_bytes!("../../assets/themes/city/vehicles/limo_white_v.png").as_slice(),
        ),
        "ambulance_h" => (
            "ambulance_h",
            include_bytes!("../../assets/themes/city/vehicles/ambulance_h.png").as_slice(),
        ),
        "ambulance_v" => (
            "ambulance_v",
            include_bytes!("../../assets/themes/city/vehicles/ambulance_v.png").as_slice(),
        ),
        "semi_truck_h" => (
            "semi_truck_h",
            include_bytes!("../../assets/themes/city/vehicles/semi_truck_h.png").as_slice(),
        ),
        "semi_truck_v" => (
            "semi_truck_v",
            include_bytes!("../../assets/themes/city/vehicles/semi_truck_v.png").as_slice(),
        ),
        "bus_transit_h" => (
            "bus_transit_h",
            include_bytes!("../../assets/themes/city/vehicles/bus_transit_h.png").as_slice(),
        ),
        "bus_transit_v" => (
            "bus_transit_v",
            include_bytes!("../../assets/themes/city/vehicles/bus_transit_v.png").as_slice(),
        ),

        // Marine Environment & Obstacles
        "marine_background" => (
            "marine_background",
            include_bytes!("../../assets/themes/marine/environment/background.png").as_slice(),
        ),
        "marine_ground" => (
            "marine_ground",
            include_bytes!("../../assets/themes/marine/environment/ground.png").as_slice(),
        ),
        "marine_exit_gate" => (
            "marine_exit_gate",
            include_bytes!("../../assets/themes/marine/environment/exit_gate.png").as_slice(),
        ),
        "marine_buoy" => (
            "marine_buoy",
            include_bytes!("../../assets/themes/marine/obstacles/buoy.png").as_slice(),
        ),
        "marine_buoy_green" => (
            "marine_buoy_green",
            include_bytes!("../../assets/themes/marine/obstacles/buoy_green.png").as_slice(),
        ),
        "marine_rock" => (
            "marine_rock",
            include_bytes!("../../assets/themes/marine/obstacles/rock.png").as_slice(),
        ),

        // Marine Ships
        "ship_player_red_h" => (
            "ship_player_red_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_player_red_h.png").as_slice(),
        ),
        "ship_player_red_v" => (
            "ship_player_red_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_player_red_v.png").as_slice(),
        ),
        "ship_sail_blue_h" => (
            "ship_sail_blue_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_sail_blue_h.png").as_slice(),
        ),
        "ship_sail_blue_v" => (
            "ship_sail_blue_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_sail_blue_v.png").as_slice(),
        ),
        "ship_taxi_yellow_h" => (
            "ship_taxi_yellow_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_taxi_yellow_h.png").as_slice(),
        ),
        "ship_taxi_yellow_v" => (
            "ship_taxi_yellow_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_taxi_yellow_v.png").as_slice(),
        ),
        "ship_tug_green_h" => (
            "ship_tug_green_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_tug_green_h.png").as_slice(),
        ),
        "ship_tug_green_v" => (
            "ship_tug_green_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_tug_green_v.png").as_slice(),
        ),
        "ship_patrol_h" => (
            "ship_patrol_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_patrol_h.png").as_slice(),
        ),
        "ship_patrol_v" => (
            "ship_patrol_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_patrol_v.png").as_slice(),
        ),
        "ship_cargo_h" => (
            "ship_cargo_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_cargo_h.png").as_slice(),
        ),
        "ship_cargo_v" => (
            "ship_cargo_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_cargo_v.png").as_slice(),
        ),
        "ship_yacht_white_h" => (
            "ship_yacht_white_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_yacht_white_h.png").as_slice(),
        ),
        "ship_yacht_white_v" => (
            "ship_yacht_white_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_yacht_white_v.png").as_slice(),
        ),
        "ship_sar_rescue_h" => (
            "ship_sar_rescue_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_sar_rescue_h.png").as_slice(),
        ),
        "ship_sar_rescue_v" => (
            "ship_sar_rescue_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_sar_rescue_v.png").as_slice(),
        ),
        "ship_container_h" => (
            "ship_container_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_container_h.png").as_slice(),
        ),
        "ship_container_v" => (
            "ship_container_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_container_v.png").as_slice(),
        ),
        "ship_ferry_h" => (
            "ship_ferry_h",
            include_bytes!("../../assets/themes/marine/vehicles/ship_ferry_h.png").as_slice(),
        ),
        "ship_ferry_v" => (
            "ship_ferry_v",
            include_bytes!("../../assets/themes/marine/vehicles/ship_ferry_v.png").as_slice(),
        ),

        // Railroad Environment & Obstacles
        "railroad_background" => (
            "railroad_background",
            include_bytes!("../../assets/themes/railroad/environment/background.png").as_slice(),
        ),
        "railroad_ground" => (
            "railroad_ground",
            include_bytes!("../../assets/themes/railroad/environment/ground.png").as_slice(),
        ),
        "railroad_exit_gate" => (
            "railroad_exit_gate",
            include_bytes!("../../assets/themes/railroad/environment/exit_gate.png").as_slice(),
        ),
        "railroad_buffer_stop" => (
            "railroad_buffer_stop",
            include_bytes!("../../assets/themes/railroad/obstacles/buffer_stop.png").as_slice(),
        ),
        "railroad_coal_pile" => (
            "railroad_coal_pile",
            include_bytes!("../../assets/themes/railroad/obstacles/coal_pile.png").as_slice(),
        ),
        "railroad_semaphore" => (
            "railroad_semaphore",
            include_bytes!("../../assets/themes/railroad/obstacles/semaphore.png").as_slice(),
        ),
        "railroad_rock" => (
            "railroad_rock",
            include_bytes!("../../assets/themes/railroad/obstacles/rock.png").as_slice(),
        ),

        // Railroad Trains
        "train_locomotive_red_h" => (
            "train_locomotive_red_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_locomotive_red_h.png")
                .as_slice(),
        ),
        "train_locomotive_red_v" => (
            "train_locomotive_red_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_locomotive_red_v.png")
                .as_slice(),
        ),
        "train_coach_blue_h" => (
            "train_coach_blue_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_coach_blue_h.png")
                .as_slice(),
        ),
        "train_coach_blue_v" => (
            "train_coach_blue_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_coach_blue_v.png")
                .as_slice(),
        ),
        "train_tanker_yellow_h" => (
            "train_tanker_yellow_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_tanker_yellow_h.png")
                .as_slice(),
        ),
        "train_tanker_yellow_v" => (
            "train_tanker_yellow_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_tanker_yellow_v.png")
                .as_slice(),
        ),
        "train_shunter_green_h" => (
            "train_shunter_green_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_shunter_green_h.png")
                .as_slice(),
        ),
        "train_shunter_green_v" => (
            "train_shunter_green_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_shunter_green_v.png")
                .as_slice(),
        ),
        "train_caboose_red_h" => (
            "train_caboose_red_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_caboose_red_h.png")
                .as_slice(),
        ),
        "train_caboose_red_v" => (
            "train_caboose_red_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_caboose_red_v.png")
                .as_slice(),
        ),
        "train_cargo_flat_h" => (
            "train_cargo_flat_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_cargo_flat_h.png")
                .as_slice(),
        ),
        "train_cargo_flat_v" => (
            "train_cargo_flat_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_cargo_flat_v.png")
                .as_slice(),
        ),
        "train_luxury_pullman_h" => (
            "train_luxury_pullman_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_luxury_pullman_h.png")
                .as_slice(),
        ),
        "train_luxury_pullman_v" => (
            "train_luxury_pullman_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_luxury_pullman_v.png")
                .as_slice(),
        ),
        "train_heavy_crane_h" => (
            "train_heavy_crane_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_heavy_crane_h.png")
                .as_slice(),
        ),
        "train_heavy_crane_v" => (
            "train_heavy_crane_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_heavy_crane_v.png")
                .as_slice(),
        ),
        "train_coal_hopper_h" => (
            "train_coal_hopper_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_coal_hopper_h.png")
                .as_slice(),
        ),
        "train_coal_hopper_v" => (
            "train_coal_hopper_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_coal_hopper_v.png")
                .as_slice(),
        ),
        "train_passenger_long_h" => (
            "train_passenger_long_h",
            include_bytes!("../../assets/themes/railroad/vehicles/train_passenger_long_h.png")
                .as_slice(),
        ),
        "train_passenger_long_v" => (
            "train_passenger_long_v",
            include_bytes!("../../assets/themes/railroad/vehicles/train_passenger_long_v.png")
                .as_slice(),
        ),

        _ => return None,
    };
    Some((key, bytes))
}
