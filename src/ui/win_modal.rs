use super::{draw_ui_button, ButtonStyle, TextureStore, THEME};
use crate::game::level::LevelData;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinModalAction {
    None,
    NextLevel,
    Replay,
    LevelSelect,
}

pub fn render_win_modal(
    level: &LevelData,
    moves_taken: u32,
    has_next_level: bool,
    textures: &TextureStore,
    screen_w: f32,
    screen_h: f32,
) -> WinModalAction {
    let mut action = WinModalAction::None;

    // Semi-transparent backdrop overlay
    draw_rectangle(
        0.0,
        0.0,
        screen_w,
        screen_h,
        Color::new(0.0, 0.0, 0.0, 0.75),
    );

    let modal_w = (screen_w * 0.85).min(440.0);
    let modal_h = 360.0;
    let modal_x = (screen_w - modal_w) / 2.0;
    let modal_y = (screen_h - modal_h) / 2.0;

    // Modal card background & border
    draw_rectangle(modal_x, modal_y, modal_w, modal_h, THEME.card_bg);
    draw_rectangle_lines(modal_x, modal_y, modal_w, modal_h, 2.5, THEME.accent_gold);

    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);

    // Title: LEVEL CLEARED!
    let title = "LEVEL CLEARED!";
    let title_font_size = 28.0;
    let title_dim = measure_text(title, None, title_font_size as u16, 1.0);
    draw_text(
        title,
        modal_x + (modal_w - title_dim.width) / 2.0,
        modal_y + 48.0,
        title_font_size,
        THEME.accent_green,
    );

    // Large 3D Gold Stars
    let stars = level.calculate_stars(moves_taken);
    textures.draw_star_row(
        modal_x + modal_w / 2.0,
        modal_y + 104.0,
        stars,
        3,
        42.0,
        12.0,
    );

    // Moves vs Par moves summary
    let stats_text = format!(
        "Completed in {} moves! (Par: {})",
        moves_taken, level.par_moves
    );
    let stats_dim = measure_text(&stats_text, None, 18, 1.0);
    draw_text(
        &stats_text,
        modal_x + (modal_w - stats_dim.width) / 2.0,
        modal_y + 154.0,
        18.0,
        THEME.text_secondary,
    );

    let rating_eval = if moves_taken <= level.par_moves {
        "Perfect Strategy!"
    } else if moves_taken <= level.par_moves + 2 {
        "Great Driving!"
    } else {
        "Good Job!"
    };
    let eval_dim = measure_text(rating_eval, None, 16, 1.0);
    draw_text(
        rating_eval,
        modal_x + (modal_w - eval_dim.width) / 2.0,
        modal_y + 188.0,
        16.0,
        THEME.accent_gold,
    );

    // Action Buttons
    let btn_h = 46.0;
    let btn_w = modal_w - 48.0;
    let mut btn_y = modal_y + 224.0;

    if has_next_level {
        if draw_ui_button(
            Rect::new(modal_x + 24.0, btn_y, btn_w, btn_h),
            "NEXT LEVEL",
            ButtonStyle {
                bg_color: THEME.accent_green,
                ..Default::default()
            },
            mouse_pos,
            is_mouse_down,
        ) {
            action = WinModalAction::NextLevel;
        }
        btn_y += btn_h + 12.0;
    }

    let half_w = (btn_w - 12.0) / 2.0;
    if draw_ui_button(
        Rect::new(modal_x + 24.0, btn_y, half_w, btn_h),
        "REPLAY",
        ButtonStyle {
            bg_color: THEME.surface,
            text_color: THEME.text_secondary,
            ..Default::default()
        },
        mouse_pos,
        is_mouse_down,
    ) {
        action = WinModalAction::Replay;
    }

    if draw_ui_button(
        Rect::new(modal_x + 24.0 + half_w + 12.0, btn_y, half_w, btn_h),
        "MENU",
        ButtonStyle {
            bg_color: THEME.surface,
            text_color: THEME.text_secondary,
            ..Default::default()
        },
        mouse_pos,
        is_mouse_down,
    ) {
        action = WinModalAction::LevelSelect;
    }

    action
}
