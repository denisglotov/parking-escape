use super::icons::{
    draw_icon_back, draw_icon_reset, draw_icon_sound, draw_icon_undo, draw_star_rating_row,
};
use super::THEME;
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
    screen_w: f32,
) -> HudAction {
    let mut action = HudAction::None;
    let hud_h = 76.0;

    // Background bar
    draw_rectangle(0.0, 0.0, screen_w, hud_h, THEME.surface);
    draw_line(
        0.0,
        hud_h,
        screen_w,
        hud_h,
        1.0,
        Color::new(0.2, 0.24, 0.32, 0.6),
    );

    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);

    let btn_size = 44.0;
    let btn_y = (hud_h - btn_size) / 2.0;

    // Left Button: Back to Menu
    if draw_vector_button(
        16.0,
        btn_y,
        btn_size,
        true,
        is_mouse_down,
        mouse_pos,
        draw_icon_back,
    ) {
        action = HudAction::BackToMenu;
    }

    // Level Title in center
    let title_text = format!("Level {}", level.id);
    let title_font_size = 24.0;
    let title_dim = measure_text(&title_text, None, title_font_size as u16, 1.0);
    draw_text(
        &title_text,
        screen_w / 2.0 - title_dim.width / 2.0,
        btn_y + 18.0,
        title_font_size,
        THEME.text_primary,
    );

    // Moves vs Par moves & Star rating
    let stars = level.calculate_stars(moves);
    let stats_str = format!("Moves: {} / Par: {}", moves, level.par_moves);
    let stats_dim = measure_text(&stats_str, None, 16, 1.0);

    let center_y = btn_y + 36.0;
    draw_text(
        &stats_str,
        screen_w / 2.0 - stats_dim.width / 2.0 - 36.0,
        center_y,
        16.0,
        THEME.text_secondary,
    );

    // Draw stars right next to moves text
    draw_star_rating_row(
        screen_w / 2.0 + stats_dim.width / 2.0 + 8.0,
        center_y - 5.0,
        stars,
        3,
        7.0,
        4.0,
        THEME.accent_gold,
    );

    // Right Buttons: Sound, Reset, Undo
    let mut right_x = screen_w - 16.0 - btn_size;

    // Sound Toggle Button
    if draw_vector_button(
        right_x,
        btn_y,
        btn_size,
        true,
        is_mouse_down,
        mouse_pos,
        |cx, cy, sz, col| draw_icon_sound(cx, cy, sz, sound_enabled, col),
    ) {
        action = HudAction::ToggleSound;
    }
    right_x -= btn_size + 10.0;

    // Reset Button
    if draw_vector_button(
        right_x,
        btn_y,
        btn_size,
        true,
        is_mouse_down,
        mouse_pos,
        draw_icon_reset,
    ) {
        action = HudAction::Reset;
    }
    right_x -= btn_size + 10.0;

    // Undo Button
    if draw_vector_button(
        right_x,
        btn_y,
        btn_size,
        can_undo,
        is_mouse_down,
        mouse_pos,
        draw_icon_undo,
    ) {
        action = HudAction::Undo;
    }

    action
}

fn draw_vector_button<F>(
    x: f32,
    y: f32,
    size: f32,
    enabled: bool,
    is_mouse_down: bool,
    mouse_pos: (f32, f32),
    draw_icon: F,
) -> bool
where
    F: FnOnce(f32, f32, f32, Color),
{
    let hovered = enabled
        && mouse_pos.0 >= x
        && mouse_pos.0 <= x + size
        && mouse_pos.1 >= y
        && mouse_pos.1 <= y + size;

    let bg_color = if !enabled {
        Color::new(0.12, 0.13, 0.16, 0.4)
    } else if hovered {
        THEME.surface_hover
    } else {
        THEME.card_bg
    };

    let icon_color = if !enabled {
        THEME.text_muted
    } else if hovered {
        THEME.accent_gold
    } else {
        THEME.text_primary
    };

    draw_rectangle(x, y, size, size, bg_color);
    draw_rectangle_lines(
        x,
        y,
        size,
        size,
        1.5,
        Color::new(0.3, 0.35, 0.45, if enabled { 0.6 } else { 0.2 }),
    );

    draw_icon(x + size / 2.0, y + size / 2.0, size * 0.55, icon_color);

    hovered && is_mouse_down
}
