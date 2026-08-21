use super::{TextureStore, UiMetrics, THEME};
use crate::game::level::LevelData;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HudAction {
    None,
    BackToMenu,
    Undo,
    Reset,
    ToggleSound,
}

pub fn render_hud(
    level: &LevelData,
    moves: u32,
    can_undo: bool,
    sound_enabled: bool,
    textures: &TextureStore,
    screen_w: f32,
) -> HudAction {
    let mut action = HudAction::None;
    let metrics = UiMetrics::new(screen_w, 800.0);
    let hud_h = metrics.hud_height;

    // Background bar
    draw_rectangle(0.0, 0.0, screen_w, hud_h, THEME.surface);
    let border_thick = (1.5 * metrics.scale).max(1.0);
    draw_line(
        0.0,
        hud_h,
        screen_w,
        hud_h,
        border_thick,
        Color::new(0.2, 0.24, 0.32, 0.6),
    );

    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);

    let btn_size = metrics.s(48.0);
    let btn_pad_x = metrics.s(14.0);
    let btn_y = (hud_h - btn_size) / 2.0;

    // Left Button: Back to Menu
    if textures.draw_icon_button(
        "icon_back",
        Rect::new(btn_pad_x, btn_y, btn_size, btn_size),
        true,
        is_mouse_down,
        mouse_pos,
    ) {
        action = HudAction::BackToMenu;
    }

    // Level Title in center
    let title_text = format!("Level {}", level.id);
    let title_font_size = metrics.s(24.0);
    let title_dim = measure_text(&title_text, None, title_font_size as u16, 1.0);
    draw_text(
        &title_text,
        screen_w / 2.0 - title_dim.width / 2.0,
        btn_y + metrics.s(18.0),
        title_font_size,
        THEME.text_primary,
    );

    // Moves vs Par moves & Star rating
    let stars = level.calculate_stars(moves);
    let stats_str = format!("Moves: {} / Par: {}", moves, level.par_moves);
    let stats_font_size = metrics.s(15.0);
    let stats_dim = measure_text(&stats_str, None, stats_font_size as u16, 1.0);

    let star_size = metrics.s(16.0);
    let star_spacing = metrics.s(3.5);
    let star_group_w = 3.0 * star_size + 2.0 * star_spacing;
    let gap = metrics.s(10.0);
    let total_stat_w = stats_dim.width + gap + star_group_w;
    let stat_start_x = screen_w / 2.0 - total_stat_w / 2.0;

    let center_y = btn_y + metrics.s(38.0);
    draw_text(
        &stats_str,
        stat_start_x,
        center_y,
        stats_font_size,
        THEME.text_secondary,
    );

    // Draw glossy star row grouped with stats text
    textures.draw_star_row(
        stat_start_x + stats_dim.width + gap + star_group_w / 2.0,
        center_y - metrics.s(5.0),
        stars,
        3,
        star_size,
        star_spacing,
    );

    // Right Buttons: Sound, Reset, Undo
    let btn_spacing = metrics.s(8.0);
    let mut right_x = screen_w - btn_pad_x - btn_size;

    // Sound Toggle Button
    let sound_tex = if sound_enabled {
        "icon_sound_on"
    } else {
        "icon_sound_off"
    };
    if textures.draw_icon_button(
        sound_tex,
        Rect::new(right_x, btn_y, btn_size, btn_size),
        true,
        is_mouse_down,
        mouse_pos,
    ) {
        action = HudAction::ToggleSound;
    }
    right_x -= btn_size + btn_spacing;

    // Reset Button
    if textures.draw_icon_button(
        "icon_reset",
        Rect::new(right_x, btn_y, btn_size, btn_size),
        true,
        is_mouse_down,
        mouse_pos,
    ) {
        action = HudAction::Reset;
    }
    right_x -= btn_size + btn_spacing;

    // Undo Button
    if textures.draw_icon_button(
        "icon_undo",
        Rect::new(right_x, btn_y, btn_size, btn_size),
        can_undo,
        is_mouse_down,
        mouse_pos,
    ) {
        action = HudAction::Undo;
    }

    action
}
