use super::{TextureStore, THEME};
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
    if textures.draw_icon_button(
        "icon_back",
        Rect::new(16.0, btn_y, btn_size, btn_size),
        true,
        is_mouse_down,
        mouse_pos,
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
        screen_w / 2.0 - stats_dim.width / 2.0 - 40.0,
        center_y,
        16.0,
        THEME.text_secondary,
    );

    // Draw glossy star row
    textures.draw_star_row(
        screen_w / 2.0 + stats_dim.width / 2.0 + 10.0,
        center_y - 6.0,
        stars,
        3,
        16.0,
        4.0,
    );

    // Right Buttons: Sound, Reset, Undo
    let mut right_x = screen_w - 16.0 - btn_size;

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
    right_x -= btn_size + 10.0;

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
    right_x -= btn_size + 10.0;

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
