use super::{draw_ui_button, ButtonStyle, TextureStore, UiMetrics, THEME};
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
    let metrics = UiMetrics::new(screen_w, screen_h);

    // Semi-transparent backdrop overlay
    draw_rectangle(
        0.0,
        0.0,
        screen_w,
        screen_h,
        Color::new(0.0, 0.0, 0.0, 0.75),
    );

    let modal_w = (screen_w * 0.88).min(metrics.s(460.0));
    let modal_h = metrics.s(380.0);
    let modal_x = (screen_w - modal_w) / 2.0;
    let modal_y = (screen_h - modal_h) / 2.0;

    // Modal card background & border
    draw_rectangle(modal_x, modal_y, modal_w, modal_h, THEME.card_bg);
    let modal_border = (2.5 * metrics.scale).max(2.0);
    draw_rectangle_lines(
        modal_x,
        modal_y,
        modal_w,
        modal_h,
        modal_border,
        THEME.accent_gold,
    );

    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);

    // Title: LEVEL CLEARED!
    let title = "LEVEL CLEARED!";
    let title_font_size = metrics.s(28.0);
    let title_dim = measure_text(title, None, title_font_size as u16, 1.0);
    draw_text(
        title,
        modal_x + (modal_w - title_dim.width) / 2.0,
        modal_y + metrics.s(48.0),
        title_font_size,
        THEME.accent_green,
    );

    // Large 3D Gold Stars
    let stars = level.calculate_stars(moves_taken);
    let star_size = metrics.s(44.0);
    let star_spacing = metrics.s(12.0);
    textures.draw_star_row(
        modal_x + modal_w / 2.0,
        modal_y + metrics.s(106.0),
        stars,
        3,
        star_size,
        star_spacing,
    );

    // Moves vs Par moves summary
    let stats_text = format!(
        "Completed in {} moves! (Par: {})",
        moves_taken, level.par_moves
    );
    let stats_font = metrics.s(18.0);
    let stats_dim = measure_text(&stats_text, None, stats_font as u16, 1.0);
    draw_text(
        &stats_text,
        modal_x + (modal_w - stats_dim.width) / 2.0,
        modal_y + metrics.s(158.0),
        stats_font,
        THEME.text_secondary,
    );

    let rating_eval = if moves_taken <= level.par_moves {
        "Perfect Strategy!"
    } else if moves_taken <= level.par_moves + 2 {
        "Great Driving!"
    } else {
        "Good Job!"
    };
    let eval_font = metrics.s(16.0);
    let eval_dim = measure_text(rating_eval, None, eval_font as u16, 1.0);
    draw_text(
        rating_eval,
        modal_x + (modal_w - eval_dim.width) / 2.0,
        modal_y + metrics.s(192.0),
        eval_font,
        THEME.accent_gold,
    );

    // Action Buttons
    let btn_h = metrics.s(48.0);
    let btn_w = modal_w - metrics.s(48.0);
    let mut btn_y = modal_y + metrics.s(232.0);
    let btn_font_size = metrics.s(18.0);
    let btn_border = (1.5 * metrics.scale).max(1.0);
    let btn_spacing = metrics.s(12.0);

    if has_next_level {
        if draw_ui_button(
            Rect::new(modal_x + metrics.s(24.0), btn_y, btn_w, btn_h),
            "NEXT LEVEL",
            ButtonStyle {
                bg_color: THEME.accent_green,
                font_size: btn_font_size,
                border_width: btn_border,
                ..Default::default()
            },
            mouse_pos,
            is_mouse_down,
        ) {
            action = WinModalAction::NextLevel;
        }
        btn_y += btn_h + btn_spacing;
    }

    let half_w = (btn_w - btn_spacing) / 2.0;
    if draw_ui_button(
        Rect::new(modal_x + metrics.s(24.0), btn_y, half_w, btn_h),
        "REPLAY",
        ButtonStyle {
            bg_color: THEME.surface,
            text_color: THEME.text_secondary,
            font_size: btn_font_size,
            border_width: btn_border,
        },
        mouse_pos,
        is_mouse_down,
    ) {
        action = WinModalAction::Replay;
    }

    if draw_ui_button(
        Rect::new(
            modal_x + metrics.s(24.0) + half_w + btn_spacing,
            btn_y,
            half_w,
            btn_h,
        ),
        "MENU",
        ButtonStyle {
            bg_color: THEME.surface,
            text_color: THEME.text_secondary,
            font_size: btn_font_size,
            border_width: btn_border,
        },
        mouse_pos,
        is_mouse_down,
    ) {
        action = WinModalAction::LevelSelect;
    }

    action
}
